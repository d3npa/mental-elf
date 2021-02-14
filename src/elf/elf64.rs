use std::fs::File;
use std::io::prelude::*;
use std::io::{SeekFrom};

use plain::Plain;
use crate::elf::ElfHeader;
use crate::utils::{Result, StringError};

pub const HEADER_SIZE: usize = 0x40;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Elf64Header {
    pub e_ident: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

impl ElfHeader for Elf64Header {
    fn to_bytes(&self) -> Vec<u8> {
        let buffer = unsafe {
            let ptr = self as *const Self as *const u8;
            std::slice::from_raw_parts(ptr, HEADER_SIZE)
        };
            
        buffer.to_vec()
    }
}

unsafe impl Plain for Elf64Header {}

impl Elf64Header {
    pub fn from_bytes(buf: &[u8]) -> Result<Elf64Header> {
        use plain::Error::*;
        
        match plain::from_bytes(buf) {
            Ok(v) => Ok(*v),
            Err(e) => match e {
                TooShort => Err(StringError::boxed("Too short")),
                BadAlignment => Err(StringError::boxed("Bad alignment")),
            },
        }
    }

    pub fn from_fd(fd: &mut File) -> Result<Elf64Header> {
        let mut buf = [0u8; HEADER_SIZE];
        fd.seek(SeekFrom::Start(0))?;
        fd.read(&mut buf)?;
        Elf64Header::from_bytes(&buf)
    }
}