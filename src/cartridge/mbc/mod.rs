mod mbc0;

use std::{fs::File, io::Read, path};

use self::mbc0::MBC0;

pub trait MBC {
    fn read_rom(&self, addr: u16) -> u8;
    fn read_ram(&self, addr: u16) -> u8;
    fn write_rom(&self, addr: u16, data: u8);
    fn write_ram(&self, addr: u16, data: u8);
}

/// Receive a path and return the correct MBC type,
/// or error if unrecognized
pub fn get_mbc(path: path::PathBuf) -> Result<Box<dyn MBC + 'static>, &'static str> {
    let mut data: Vec<u8> = vec![];
    File::open(path)
        .and_then(|mut f| f.read_to_end(&mut data))
        .map_err(|_| "Could not read ROM")?;

    if data.len() < 0x0148 {
        return Err("ROM too small");
    }

    match data[0x0147] {
        0x00 => Ok(Box::new(MBC0::new(data)?)),
        0x01..=0x03 => todo!("MBC1"),
        0x05..=0x06 => todo!("MBC2"),
        0x0F..=0x13 => todo!("MBC3"),
        0x19..=0x1E => todo!("MBC5"),
        _ => todo!("MBC format not supported. Only support MBC0, 1, 2, 3 and 5"),
    }
}
