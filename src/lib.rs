pub mod utils;
pub mod elf64;

use utils::Result;
use elf64::Elf64Header;

use std::io::prelude::*;
use std::io::SeekFrom;
use std::fs::File;

pub fn read_elf64_header(fd: &mut File) -> Result<Elf64Header> {
    let mut buffer = [0u8; elf64::HEADER_SIZE];
    fd.seek(SeekFrom::Start(0))?;
    fd.read(&mut buffer)?;
    Elf64Header::from_bytes(&buffer)
}

pub fn write_elf64_header(fd: &mut File, header: Elf64Header) -> Result<()> {
    let buffer = header.to_bytes();
    fd.seek(SeekFrom::Start(0))?;
    fd.write(&buffer)?;
    Ok(())
}