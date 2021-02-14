pub mod utils;
pub mod elf64;

use utils::Result;
use elf64::Elf64Header;
use elf64::Elf64ProgramHeader;

use std::io::prelude::*;
use std::io::SeekFrom;
use std::fs::File;

pub fn read_elf64_header(fd: &mut File) -> Result<Elf64Header> {
    let mut buffer = [0u8; elf64::ELF64_HEADER_SIZE];
    fd.seek(SeekFrom::Start(0))?;
    fd.read(&mut buffer)?;
    Ok(Elf64Header::from_bytes(buffer))
}

pub fn write_elf64_header(fd: &mut File, header: Elf64Header) -> Result<()> {
    let buffer = header.to_bytes();
    fd.seek(SeekFrom::Start(0))?;
    fd.write(&buffer)?;
    Ok(())
}

pub fn read_elf64_program_headers(
    fd: &mut File, phdr_offset: u64, phdr_num: u16) 
    -> Result<Vec<Elf64ProgramHeader>> 
{
    fd.seek(SeekFrom::Start(phdr_offset))?;
    
    let mut program_headers = Vec::new();

    for _ in 0..phdr_num {
        let mut buffer = [0u8; elf64::ELF64_PHDR_SIZE];
        fd.read(&mut buffer)?;
        program_headers.push(Elf64ProgramHeader::from_bytes(buffer));
    }

    Ok(program_headers)
}