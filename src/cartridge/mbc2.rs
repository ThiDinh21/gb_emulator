use crate::cartridge::MBC;

pub struct MBC2 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_bank_idx: usize,
}
