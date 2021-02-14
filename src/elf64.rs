use crate::utils::{Result, StringError};

use std::mem;
use plain::Plain;

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

unsafe impl Plain for Elf64Header {}

impl Elf64Header {
    pub fn from_bytes(buf: &[u8]) -> Result<Elf64Header> {
        use plain::Error::*;
        
        // NOTE: Can maybe simplify with trait to convert dyn Error into StringError by calling to_string()?
        match plain::from_bytes(buf) {
            Ok(v) => Ok(*v),
            Err(e) => match e {
                TooShort => Err(StringError::boxed("Too short")),
                BadAlignment => Err(StringError::boxed("Bad alignment")),
            },
        }
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let buffer = unsafe {
            mem::transmute::<Elf64Header, [u8; HEADER_SIZE]>(self)
            // let ptr = &self as *const Self as *const u8;
            // std::slice::from_raw_parts(ptr, HEADER_SIZE)
        };
            
        buffer.to_vec()
    }
}