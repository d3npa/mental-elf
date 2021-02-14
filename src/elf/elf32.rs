use crate::elf::ElfHeader;

#[derive(Debug)]
pub struct Elf32Header {}

impl ElfHeader for Elf32Header {
    fn to_bytes(&self) -> Vec<u8> {
        vec![]
    }
}