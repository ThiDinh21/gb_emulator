mod mbc0;
mod mbc1;
mod mbc2;

use std::path::PathBuf;
use std::{fs::File, io::Read};

use self::mbc0::MBC0;
use self::mbc1::MBC1;
use self::mbc2::MBC2;

pub trait MBC {
    // a ROM bank size is 0x4000
    fn read_rom(&self, addr: u16) -> u8;
    fn read_ram(&self, addr: u16) -> u8;
    fn write_rom(&mut self, addr: u16, data: u8);
    fn write_ram(&mut self, addr: u16, data: u8);
}

/// Receive a path and return the correct MBC type,
/// or error if unrecognized
pub fn get_mbc(path: PathBuf) -> Result<Box<dyn MBC + 'static>, &'static str> {
    let mut data: Vec<u8> = vec![];
    File::open(&path)
        .and_then(|mut f| f.read_to_end(&mut data))
        .map_err(|_| "Could not read ROM")?;

    if data.len() < 0x0148 {
        return Err("ROM is too small");
    }

    match data[0x0147] {
        0x00 => Ok(Box::new(MBC0::new(data)?)),
        0x01..=0x03 => Ok(Box::new(MBC1::new(data, path)?)),
        0x05..=0x06 => Ok(Box::new(MBC2::new(data, path)?)),
        0x0F..=0x13 => todo!("MBC3"),
        0x19..=0x1E => todo!("MBC5"),
        _ => todo!("MBC format not supported. Only support MBC0, 1, 2, 3 and 5"),
    }
}

/// https://gbdev.io/pandocs/The_Cartridge_Header.html#0149--ram-size
pub fn get_ram_size(byte_0149: u8) -> u32 {
    match byte_0149 {
        0x02 => 0x2000,
        0x03 => 0x8000,
        0x04 => 0x20000,
        0x05 => 0x4000,
        _ => 0,
    }
}
