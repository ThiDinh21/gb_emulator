use std::{fs::File, io::Write, path::PathBuf};

use crate::cartridge::MBC;

/// https://gbdev.io/pandocs/MBC2.html
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
        let mut mbc = MBC2 {
            rom: raw,
            ram: vec![0; 0x200],
            rom_bank_idx: 1,
            ram_enabled: false,
            save_file,
        };

        mbc.load_save_file();

        Ok(mbc)
    }

    fn load_save_file(&mut self) {
        match File::open(&self.save_file) {
            Ok(mut f) => {
                f.write_all(&self.ram)
                    .expect("Error loading save file. Save file corrupt?");
            }
            Err(_) => {
                dbg!("Unable to open save file");
            }
        };
    }
}

/// auto save when drop CPU
impl Drop for MBC2 {
    fn drop(&mut self) {
        let mut save_file =
            File::create(&self.save_file).expect("Cannot create save file at {path}");
        save_file
            .write_all(&self.ram)
            .expect("Cannot write to save file at {path}");
    }
}

impl MBC for MBC2 {
    fn read_rom(&self, addr: u16) -> u8 {
        let index = match addr {
            0x0000..=0x3FFF => 0,
            0x4000..=0x7FFF => self.rom_bank_idx * 0x4000 + (addr as usize - 0x4000),
            _ => return 0,
        };

        *self.rom.get(index).unwrap_or(&0)
    }

    fn read_ram(&self, addr: u16) -> u8 {
        let index = (addr as usize - 0xA000) % 0x0200;

        let res = *self.rom.get(index).unwrap_or(&0);
        res & 0x0F
    }

    fn write_rom(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..=0x3FFF => {
                if addr & 0x0100 == 0 {
                    self.ram_enabled = (data & 0x0F) == 0x0A;
                } else {
                    self.rom_bank_idx = (data as usize & 0x0F).max(1)
                }
            }
            0x4000..=0x7FFF => panic!("Trying to write to read-only {addr:04x} - MBC2"),
            _ => panic!("Cannot write to {addr:04x} - MBC2"),
        }
    }

    fn write_ram(&mut self, addr: u16, data: u8) {
        if !self.ram_enabled {
            return;
        }
        let index = (addr as usize - 0xA000) % 0x0200;
        self.ram[index] = data & 0x0F;
    }
}
