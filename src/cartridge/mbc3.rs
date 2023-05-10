use super::get_ram_size;
use crate::cartridge::MBC;
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

/// $08  RTC S   Seconds   0-59 ($00-$3B)
/// $09  RTC M   Minutes   0-59 ($00-$3B)
/// $0A  RTC H   Hours     0-23 ($00-$17)
/// $0B  RTC DL  Lower 8 bits of Day Counter ($00-$FF)
/// $0C  RTC DH  Upper 1 bit of Day Counter, Carry Bit, Halt Flag
///       Bit 0  Most significant bit of Day Counter (Bit 8)
///       Bit 6  Halt (0=Active, 1=Stop Timer)
///       Bit 7  Day Counter Carry Bit (1=Counter Overflow)
struct RTCRegister {
    pub sec: u8,
    pub min: u8,
    pub hour: u8,
    pub day_low: u8,
    pub day_high: u8,
}

impl RTCRegister {
    pub fn new() -> Self {
        RTCRegister {
            sec: 0,
            min: 0,
            hour: 0,
            day_low: 0,
            day_high: 0,
        }
    }

    pub fn copy_from(&mut self, source: &RTCRegister) {
        self.sec = source.sec;
        self.min = source.min;
        self.hour = source.hour;
        self.day_low = source.day_low;
        self.day_high = source.day_high;
    }
}

/// https://gbdev.io/pandocs/MBC3.html
pub struct MBC3 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_bank_idx: usize,
    ram_bank_idx: usize,
    ram_enabled: bool,
    ram_mode: bool,
    rtc_reg: RTCRegister,
    rtc_reg_latch: RTCRegister,
    rtc_result: Option<u64>,
    rtc_halt: bool,
    save_file: Option<PathBuf>,
}

impl MBC3 {
    pub fn new(raw: Vec<u8>, path: PathBuf) -> Result<Self, &'static str> {
        let subtype = raw[0x0147];

        let save_file = match subtype {
            0x0F | 0x10 | 0x13 => Some(path.with_extension("save")),
            _ => None,
        };

        let ram_size = match subtype {
            0x10 | 0x12 | 0x13 => get_ram_size(raw[0x0149]),
            _ => 0,
        };

        let rtc_result = match subtype {
            0x0F | 0x10 => Some(0),
            _ => None,
        };

        let mut mbc = MBC3 {
            rom: raw,
            ram: vec![0; ram_size as usize],
            rom_bank_idx: 1,
            ram_bank_idx: 0,
            ram_enabled: false,
            ram_mode: false,
            rtc_reg: RTCRegister::new(),
            rtc_reg_latch: RTCRegister::new(),
            rtc_result,
            rtc_halt: false,
            save_file,
        };

        mbc.load_save_file()?;

        Ok(mbc)
    }

    fn load_save_file(&mut self) -> Result<(), &'static str> {
        match &self.save_file {
            None => Ok(()),
            Some(path) => {
                let mut file = match File::open(path) {
                    Ok(f) => f,
                    Err(_) => {
                        dbg!("Unable to open save file");
                        return Ok(());
                    }
                };

                let mut data = vec![];
                match file.read_to_end(&mut data) {
                    Err(..) => Err("Could not read save file"),
                    Ok(..) => {
                        self.ram = data;
                        Ok(())
                    }
                }
            }
        }
    }

    fn latch_clock_data(&mut self) {
        unimplemented!("Latch RTC for MBC3");
        self.calc_rtc_reg();
        self.rtc_reg_latch.copy_from(&self.rtc_reg);
    }

    fn calc_rtc_reg(&mut self) {
        unimplemented!("Latch RTC for MBC3");
    }

    fn calc_rtc_result(&mut self) {
        unimplemented!("Latch RTC for MBC3");
    }
}

/// auto save when drop CPU
impl Drop for MBC3 {
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

impl MBC for MBC3 {
    fn read_rom(&self, addr: u16) -> u8 {
        let index = match addr {
            0x0000..=0x3FFF => addr as usize,
            0x4000..=0x7FFF => self.rom_bank_idx * 0x4000 + (addr as usize - 0x4000),
            _ => return 0,
        };

        *self.rom.get(index).unwrap_or(&0)
    }

    fn read_ram(&self, addr: u16) -> u8 {
        if !self.ram_enabled {
            return 0;
        }

        *self
            .ram
            .get(self.ram_bank_idx * 0x0200 + (addr as usize - 0x2000))
            .unwrap_or(&0)
    }

    fn write_rom(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..=0x1FFF => self.ram_enabled = data == 0x0A,
            0x2000..=0x3FFF => self.rom_bank_idx = (data as usize & 0b0111_1111).max(1),
            0x4000..=0x5FFF => self.ram_bank_idx = data as usize,
            0x6000..=0x7FFF => unimplemented!("Latch RTC for MBC3"),
            _ => panic!("Cannot write to {addr:04x} - MBC3"),
        }
    }

    fn write_ram(&mut self, addr: u16, data: u8) {
        if !self.ram_enabled {
            return;
        }

        if self.ram_bank_idx <= 3 {
            let bank = if self.ram_mode { self.ram_bank_idx } else { 0 };
            let index = bank * 0x2000 + (addr as usize - 0x2000);

            self.ram[index] = data;
        } else {
            unimplemented!("Latch RTC for MBC3");
        }
    }
}
