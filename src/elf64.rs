use std::mem;

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

impl Elf64Header {
    pub fn from_bytes(buffer: [u8; HEADER_SIZE]) -> Elf64Header {
        unsafe {
            mem::transmute::<[u8; HEADER_SIZE], Elf64Header>(buffer)
        }
    }

    pub fn to_bytes(self) -> [u8; HEADER_SIZE] {
        unsafe {
            mem::transmute::<Elf64Header, [u8; HEADER_SIZE]>(self)
        }
    }
}