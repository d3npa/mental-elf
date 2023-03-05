pub mod elf64;
pub mod utils;

use elf64::{ElfHeader, ProgramHeader, SectionHeader, SymbolTableEntry};
use utils::{Result, StringError};

use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
// use std::mem;

// pub unsafe trait Serialize<const SIZE: usize>: Sized {
//     fn from_bytes(buffer: [u8; SIZE]) -> Self {
//         unsafe {
//             mem::transmute::<[u8; SIZE], Self>(buffer)
//         }
//     }

//     fn to_bytes(self: Self) -> [u8; SIZE] {
//         unsafe {
//             mem::transmute::<Self, [u8; SIZE]>(self)
//         }
//     }
// }

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
    fd: &mut File,
    phdr_offset: u64,
    phdr_num: u16,
) -> Result<Vec<ProgramHeader>> {
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
    fd: &mut File,
    phdr_offset: u64,
    phdr_num: u16,
    program_headers: Vec<ProgramHeader>,
) -> Result<()> {
    if phdr_num as usize != program_headers.len() {
        return Err(StringError::boxed(&format!(
            "Attempting to write more or fewer program headers than the file contains: {}:{}",
            program_headers.len(),
            phdr_num
        )));
    }

    fd.seek(SeekFrom::Start(phdr_offset))?;

    for header in program_headers {
        let buffer = header.to_bytes();
        fd.write(&buffer)?;
    }

    Ok(())
}

pub fn read_elf64_section_headers(
    fd: &mut File,
    shdr_offset: u64,
    shdr_num: u16,
) -> Result<Vec<SectionHeader>> {
    let mut section_headers = Vec::new();
    fd.seek(SeekFrom::Start(shdr_offset))?;

    for _ in 0..shdr_num {
        let mut buffer = [0u8; elf64::SECTION_HEADER_SIZE];
        fd.read(&mut buffer)?;
        section_headers.push(SectionHeader::from_bytes(buffer));
    }

    Ok(section_headers)
}

pub fn read_elf64_symbol_table(
    fd: &mut File,
    symtab_hdr: &SectionHeader,
) -> Result<Vec<SymbolTableEntry>> {
    let offset: u64 = symtab_hdr.sh_offset;
    let size: u64 = symtab_hdr.sh_size;
    let entsize: u64 = symtab_hdr.sh_entsize;

    if entsize != elf64::SYMBOL_TABLE_ENTRY_SIZE as u64 {
        return Err(StringError::boxed(
            "entsize ~ symbol_table_entry_size mismatch",
        ));
    }

    fd.seek(SeekFrom::Start(offset))?;
    let num = size / entsize;
    let mut symbol_table = Vec::new();
    for _ in 0..num {
        let mut buffer = [0u8; elf64::SYMBOL_TABLE_ENTRY_SIZE];
        fd.read(&mut buffer)?;
        symbol_table.push(SymbolTableEntry::from_bytes(buffer));
    }

    Ok(symbol_table)
}
