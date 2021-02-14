use mental_elf::utils::{Result, StringError};
use std::{env, fs, process};

struct Arguments {
    filepath: String,
}

fn main() -> Result<()> {
    // Get file
    let args = parse_args().unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1);
    });

    // Open file
    let mut fd = fs::OpenOptions::new()
        .read(true).write(true).open(&args.filepath)?;

    // Read Elf Header (assume 64bit)
    let mut header = mental_elf::read_elf64_header(&mut fd)?;

    // Modify header
    println!("Old Signature: {:x?}", header.e_ident);
    let signature: [u8; 16] = [
        0x7f, 0x45, 0x4c, 0x46, 0x02, 0x01, 0x01, 0x00,
        0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68
    ];
    header.e_ident = signature;
    println!("New Signature: {:x?}", header.e_ident);
    
    // Write Elf Header
    mental_elf::write_elf64_header(&mut fd, header)?;

    Ok(())
}

fn parse_args() -> Result<Arguments> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(StringError::boxed(
            &format!("Usage: {} <ELF File>", args[0])
        ));
    }

    let filepath = args[1].clone();
    Ok(Arguments { filepath })
}