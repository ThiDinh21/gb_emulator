use crate::cartridge::mbc::MBC;

pub struct MBC0 {
    rom: Vec<u8>,
}

impl MBC0 {
    pub fn new(raw: Vec<u8>) -> Result<Self, &'static str> {
        Ok(MBC0 { rom: raw })
    }
}

impl MBC for MBC0 {
    fn read_rom(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }

    fn read_ram(&self, _addr: u16) -> u8 {
        0
    }

    fn write_rom(&mut self, _addr: u16, _data: u8) {
        ()
    }

    fn write_ram(&mut self, _addr: u16, _data: u8) {
        ()
    }
}
