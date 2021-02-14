use std::mem;

pub const ELF_HEADER_SIZE: usize = 0x40;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ElfHeader {
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

impl ElfHeader {
    pub fn from_bytes(buffer: [u8; ELF_HEADER_SIZE]) -> ElfHeader {
        unsafe {
            mem::transmute::<[u8; ELF_HEADER_SIZE], ElfHeader>(buffer)
        }
    }

    pub fn to_bytes(self) -> [u8; ELF_HEADER_SIZE] {
        unsafe {
            mem::transmute::<ElfHeader, [u8; ELF_HEADER_SIZE]>(self)
        }
    }
}

pub const PROGRAM_HEADER_SIZE: usize = 0x38;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ProgramHeader {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

impl ProgramHeader {
    pub fn from_bytes(buffer: [u8; PROGRAM_HEADER_SIZE]) -> ProgramHeader {
        unsafe {
            mem::transmute::<[u8; PROGRAM_HEADER_SIZE], ProgramHeader>(buffer)
        }
    }

    pub fn to_bytes(self) -> [u8; PROGRAM_HEADER_SIZE] {
        unsafe {
            mem::transmute::<ProgramHeader, [u8; PROGRAM_HEADER_SIZE]>(self)
        }
    }
}