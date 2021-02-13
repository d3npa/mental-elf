use std::{fs, io};
use std::io::prelude::*;
use mental_elf::ElfHeader64;

fn main() -> io::Result<()> {
    let mut fd = fs::OpenOptions::new()
        .read(true).write(true).open("./example_elf")?;

    let mut buffer = [0u8; 0x40];
    let _ = fd.read(&mut buffer);
    let header = ElfHeader64::from_bytes(&buffer);

    println!("{:?}", header);

    Ok(())
}