use mental_elf::utils::{Result, StringError};
use std::{env, fs, process};

fn main() -> Result<()> {
    // Get Arguments
    let args = parse_args().unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    // Open ELF File
    let mut elf_fd = fs::OpenOptions::new()
        .read(true).write(true).open(&args.elf_path)?;

    // Parse and print ELF Header
    let elf_header = mental_elf::read_elf64_header(&mut elf_fd)?;
    println!("{:x?}", elf_header);

    // Parse and print Program Headers
    let phdr_offset = elf_header.e_phoff;
    let phdr_num = elf_header.e_phnum;
    let program_headers = mental_elf::read_elf64_program_headers(
        &mut elf_fd, 
        phdr_offset, 
        phdr_num
    )?;

    for phdr in &program_headers {
        println!("{:x?}", phdr);
    }

    Ok(())
}

struct Arguments {
    elf_path: String,
}

fn parse_args() -> Result<Arguments> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(StringError::boxed(
            &format!("Usage: {} <ELF File>", args[0])
        ));
    }

    Ok(Arguments { 
        elf_path: args[1].clone(), 
    })
}