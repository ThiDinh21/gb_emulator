use std::path::PathBuf;

use crate::cartridge::MBC;

pub struct MBC2 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_bank_idx: usize,
    ram_enabled: bool,
    save_file: PathBuf,
}

impl MBC2 {
    pub fn new(raw: Vec<u8>, path: PathBuf) -> Result<Self, &'static str> {
        let save_file = path.with_extension("save");
        let ram = todo!();

        Ok(MBC2 {
            rom: (),
            ram: (),
            rom_bank_idx: (),
            ram_enabled: (),
            save_file: (),
        })
    }
}
