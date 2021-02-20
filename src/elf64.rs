use std::mem;
pub use constants::*;

// TODO Separate this module into its own file ; ex: elf64/constants.rs
pub mod constants {
    /* Some of these would be good candidates for enums but... I'm not sure
    how to use enums while making sure they can be transmuted from [u8] so...
    for now I'm using constants ^^; */

    // ElfHeader
    pub const ELF_HEADER_SIZE: usize = 0x40;

    // ElfHeader.e_type
    pub const ET_NONE: u16 = 0;
    pub const ET_REL: u16 = 1;
    pub const ET_EXEC: u16 = 2;
    pub const ET_DYN: u16 = 3;
    pub const ET_CORE: u16 = 4;
    pub const ET_LOPROC: u16 = 0xff00;
    pub const ET_HIPROC: u16 = 0xffff;

    // ElfHeader.e_machine
    pub const EM_NONE: u16 = 0x0;
    pub const EM_M32: u16 = 0x1;
    pub const EM_SPARC: u16 = 0x2;
    pub const EM_386: u16 = 0x3;
    pub const EM_68K: u16 = 0x4;
    pub const EM_88K: u16 = 0x5;
    pub const EM_860: u16 = 0x7;
    pub const EM_MIPS: u16 = 0x8;

    // ElfHeader.e_version
    pub const EV_NONE: u32 = 0x0;
    pub const EV_CURRENT: u32 = 0x1;

    // ElfHeader.e_ident indexes
    pub const EI_MAG0: usize = 0x0;
    pub const EI_MAG1: usize = 0x1;
    pub const EI_MAG2: usize = 0x2;
    pub const EI_MAG3: usize = 0x3;
    pub const EI_CLASS: usize = 0x4;
    pub const EI_DATA: usize = 0x5;
    pub const EI_VERSION: usize = 0x6;
    pub const EI_PAD: usize = 0x7;
    pub const EI_NIDENT: usize = 0x10;

    // ElfHeader.e_ident[EI_CLASS]
    pub const ELFCLASSNONE: u8 = 0x0;
    pub const ELFCLASS32: u8 = 0x1;
    pub const ELFCLASS64: u8 = 0x2;

    // ElfHeader.e_ident[EI_DATA]
    // TODO: continue

    // Program Headers
    pub const PROGRAM_HEADER_SIZE: usize = 0x38;
    pub const PT_NULL: u32 = 0x0;
    pub const PT_LOAD: u32 = 0x1;
    pub const PT_DYNAMIC: u32 = 0x2;
    pub const PT_INTERP: u32 = 0x3;
    pub const PT_NOTE: u32 = 0x4;
    pub const PT_SHLIB: u32 = 0x5;
    pub const PT_PHDR: u32 = 0x6;
    pub const PT_LOPROC: u32 = 0x70000000;
    pub const PT_HIPROC: u32 = 0x7fffffff;

    pub const PF_X: u32 = 0b001;
    pub const PF_W: u32 = 0b010;
    pub const PF_R: u32 = 0b100;
}


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