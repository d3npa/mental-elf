pub mod elf32;
pub mod elf64;

pub use elf32::Elf32Header;
pub use elf64::Elf64Header;

use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use crate::utils::{Result, StringError};

pub enum Architecture {
    Elf32,
    Elf64,
}

pub trait ElfHeader: fmt::Debug {
    fn to_bytes(&self) -> Vec<u8>;
}

impl dyn ElfHeader {
    pub fn from_fd(fd: &mut File) -> Result<Box<dyn ElfHeader>> {

        let architecture = read_architecture(fd)?;
    
        match architecture {
            Architecture::Elf32 => 
                Err(StringError::boxed("32bit is not yet implemented")),
            Architecture::Elf64 => 
                Ok(Box::new(Elf64Header::from_fd(fd)?)),
        }
    }
}

pub fn read_architecture(fd: &mut File) -> Result<Architecture> {
    let mut buf = [0u8];
    fd.seek(SeekFrom::Start(0x4))?;
    fd.read(&mut buf)?;
    fd.seek(SeekFrom::Start(0))?;

    match &buf[0] {
        1 => Ok(Architecture::Elf32),
        2 => Ok(Architecture::Elf64),
        _ => Err(StringError::boxed("unknown architecture")),
    }
}