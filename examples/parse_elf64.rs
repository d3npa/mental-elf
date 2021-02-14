use std::{env, fs, process};
use std::error::Error;

use mental_elf::elf::ElfHeader;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <ELF File>", args[0]);
        process::exit(1);
    }

    let path = &args[1];

    let mut fd = fs::OpenOptions::new()
        .read(true).write(true).open(&path)?;

    let header = ElfHeader::from_fd(&mut fd)?;

    println!("{:x?}", header);

    let bytes = header.to_bytes();
    println!("{:x?}", bytes);

    Ok(())
}