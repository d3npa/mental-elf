pub mod utils;
pub mod elf64;

use utils::{Result, StringError};
use elf64::ElfHeader;
use elf64::ProgramHeader;

use std::io::prelude::*;
use std::io::SeekFrom;
use std::fs::File;

pub fn read_elf64_header(fd: &mut File) -> Result<ElfHeader> {
    let mut buffer = [0u8; elf64::ELF_HEADER_SIZE];
    fd.seek(SeekFrom::Start(0))?;
    fd.read(&mut buffer)?;
    Ok(ElfHeader::from_bytes(buffer))
}

pub fn write_elf64_header(fd: &mut File, header: ElfHeader) -> Result<()> {
    let buffer = header.to_bytes();
    fd.seek(SeekFrom::Start(0))?;
    fd.write(&buffer)?;
    Ok(())
}

pub fn read_elf64_program_headers(
    fd: &mut File, phdr_offset: u64, phdr_num: u16) 
    -> Result<Vec<ProgramHeader>> 
{
    fd.seek(SeekFrom::Start(phdr_offset))?;
    
    let mut program_headers = Vec::new();

    for _ in 0..phdr_num {
        let mut buffer = [0u8; elf64::PROGRAM_HEADER_SIZE];
        fd.read(&mut buffer)?;
        program_headers.push(ProgramHeader::from_bytes(buffer));
    }

    Ok(program_headers)
}

pub fn write_elf64_program_headers(
    fd: &mut File, phdr_offset: u64, phdr_num: u16, 
    program_headers: Vec<ProgramHeader>) -> Result<()> 
{
    if phdr_num as usize != program_headers.len() {
        return Err(StringError::boxed(
            &format!("Attempting to write more or fewer program headers than the file contains: {}:{}", program_headers.len(), phdr_num)
        ));
    }

    fd.seek(SeekFrom::Start(phdr_offset))?;

    for header in program_headers {
        let buffer = header.to_bytes();
        fd.write(&buffer)?;
    }

    Ok(())
}