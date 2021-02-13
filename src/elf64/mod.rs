use plain::Plain;

#[repr(C)]
#[derive(Debug)]
pub struct ElfHeader64 {
    pub e_ident: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: usize,
    pub e_phoff: usize,
    pub e_shoff: usize,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

unsafe impl Plain for ElfHeader64 {}

impl ElfHeader64 {
    pub fn from_bytes(buf: &[u8]) -> &ElfHeader64 {
        plain::from_bytes(buf)
            .expect("The buffer is either too short or not aligned!")
    }
}