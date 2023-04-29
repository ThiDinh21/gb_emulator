use crate::cartridge::mbc::MBC;
use std::{fs::File, io::Write, path::PathBuf};

use super::get_ram_size;

pub struct MBC1 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_bank_idx: usize,
    ram_bank_idx: usize,
    ram_enabled: bool,
    ram_selected: bool,
    save_file: Option<PathBuf>,
}

impl MBC1 {
    pub fn new(raw: Vec<u8>, path: PathBuf) -> Result<Self, &'static str> {
        let (save_file, ram_size) = match raw[0x0147] {
            0x02 => (None, get_ram_size(raw[0x0149])),
            0x03 => (Some(path.with_extension("save")), get_ram_size(raw[0x0149])),
            _ => (None, 0),
        };

        Ok(MBC1 {
            rom: raw,
            ram: vec![0; ram_size as usize],
            rom_bank_idx: 1,
            ram_bank_idx: 0,
            ram_enabled: false,
            ram_selected: false,
            save_file,
        })
    }
}

impl MBC for MBC1 {
    fn read_rom(&self, addr: u16) -> u8 {
        let index = match addr {
            0x0000..=0x3FFF => addr as usize,
            0x4000..=0x7FFF => self.rom_bank_idx * 0x4000 + (addr as usize - 0x4000),
            _ => {
                return 0;
            }
        };

        *self.rom.get(index).unwrap_or(&0)
    }

    fn read_ram(&self, addr: u16) -> u8 {
        if !self.ram_enabled {
            return 0;
        }

        *self
            .ram
            .get(self.ram_bank_idx * 0x2000 + (addr as usize - 0x2000))
            .unwrap_or(&0)
    }

    fn write_rom(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..=0x1FFF => self.ram_enabled = data == 0x0A,
            // https://gbdev.io/pandocs/MBC1.html#20003fff--rom-bank-number-write-only
            0x2000..=0x3FFF => {
                self.rom_bank_idx = (self.ram_bank_idx & 0b0110_0000) // to keep the 2 bits 5th and 6th in case rom bank > 5 bits
                    | match data as usize & 0x1F {
                        0 => 1,
                        n => n,
                    }
            }
            // https://gbdev.io/pandocs/MBC1.html#40005fff--ram-bank-number--or--upper-bits-of-rom-bank-number-write-only
            0x4000..=0x5FFF => {
                if self.ram_selected {
                    self.ram_bank_idx = data as usize & 0b0000_0011;
                } else {
                    self.rom_bank_idx =
                        (self.rom_bank_idx & 0x1F) | ((data as usize & 0b0000_0011) << 5);
                }
            }
            0x6000..=0x7FFF => self.ram_selected = data & 0b1 == 1,
            _ => panic!("Cannot write to {addr:04x} - MBC1"),
        };
    }

    fn write_ram(&mut self, addr: u16, data: u8) {
        if !self.ram_enabled {
            return;
        }

        let bank = if self.ram_selected {
            self.ram_bank_idx
        } else {
            0
        };
        let index = bank * 0x2000 + (addr as usize - 0x2000);

        self.ram[index] = data;
    }
}

/// auto save when drop CPU
impl Drop for MBC1 {
    fn drop(&mut self) {
        match &self.save_file {
            None => (),
            Some(path) => {
                let mut save_file = File::create(path).expect("Cannot create save file at {path}");
                save_file
                    .write_all(&self.ram)
                    .expect("Cannot write to save file at {path}");
            }
        }
    }
}
