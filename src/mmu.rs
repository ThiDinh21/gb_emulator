use std::path::PathBuf;

use crate::{
    cartridge::{get_mbc, MBC},
    cpu::Mem,
    timer::Timer,
};

#[derive(PartialEq, Copy, Clone)]
pub enum GbMode {
    Classic,
    Color,
    ColorAsClassic,
}

/// Memory map:
/// https://gbdev.io/pandocs/Memory_Map.html
pub struct MMU {
    pub mbc: Box<dyn MBC + 'static>,
    vram: [u8; 0x2000],
    wram: [u8; 0x8000],
    wram_bank_idx: usize,
    timer: Timer,
    oam: [u8; 0xA0],
    hram: [u8; 0x7F],
    pub interrupt_enable: u8,
    pub mode: GbMode,
}

impl MMU {
    pub fn new(path: PathBuf) -> Self {
        let mbc = match get_mbc(path) {
            Ok(m) => m,
            Err(s) => panic!("Error creating MMU: {s}"),
        };

        let mut mmu = MMU {
            mbc,
            vram: [0; 0x2000],
            wram: [0; 0x8000],
            wram_bank_idx: 1,
            timer: Timer::new(),
            oam: [0; 0xA0],
            hram: [0; 0x7F],
            interrupt_enable: 0,
            mode: GbMode::Classic,
        };
        // mmu.initiate();
        mmu
    }

    fn initiate(&mut self) {
        unimplemented!()
    }

    fn execute_cycle(&mut self) {
        unimplemented!()
    }
}

impl Mem for MMU {
    fn mem_read_u8(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.mbc.read_rom(addr),
            0x8000..=0x9FFF => todo!("VRAM"),
            0xA000..=0xBFFF => self.mbc.read_ram(addr),
            0xC000..=0xCFFF => self.wram[addr as usize - 0xC000],
            0xD000..=0xDFFF => self.wram[(self.wram_bank_idx * 0x1000) + (addr as usize - 0xC000)],
            0xFE00..=0xFE9F => todo!("OAM"),
            0xFF00 => todo!("Joypad input"),
            0xFF01..=0xFF02 => unimplemented!("Serial transfer"),
            0xFF04..=0xFF07 => self.timer.mem_read_u8(addr),
            0xFF10..=0xFF26 => unimplemented!("Audio"),
            0xFF30..=0xFF3F => unimplemented!("Wave pattern"),
            0xFF40..=0xFF4B => {
                unimplemented!("LCD Control, Status, Position, Scrolling, and Palettes")
            }
            0xFF4F => unimplemented!("VRAM Bank Select"),
            0xFF50 => unimplemented!("Set to non-zero to disable boot ROM"),
            0xFF51..=0xFF55 => unimplemented!("VRAM DMA"),
            0xFF68..=0xFF69 => unimplemented!("BG / OBJ Palettes"),
            0xFF70 => self.wram_bank_idx as u8,
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize],
            0xFFFF => self.interrupt_enable,
            0xE000..=0xFDFF | 0xFEA0..=0xFEFF => {
                panic!("Attempt to access prohibited memory region")
            }
            _ => {
                panic!("Attempt to access unused memory region")
            }
        }
    }

    fn mem_write_u8(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..=0x7FFF => self.mbc.write_rom(addr, data),
            0x8000..=0x9FFF => todo!("VRAM"),
            0xA000..=0xBFFF => self.mbc.write_ram(addr, data),
            0xC000..=0xCFFF => self.wram[addr as usize - 0xC000] = data,
            0xD000..=0xDFFF => {
                self.wram[(self.wram_bank_idx * 0x1000) + (addr as usize - 0xC000)] = data
            }
            0xFE00..=0xFE9F => todo!("OAM"),
            0xFF00 => todo!("Joypad input"),
            0xFF01..=0xFF02 => unimplemented!("Serial transfer"),
            0xFF04..=0xFF07 => self.timer.mem_write_u8(addr, data),
            0xFF10..=0xFF26 => unimplemented!("Audio"),
            0xFF30..=0xFF3F => unimplemented!("Wave pattern"),
            0xFF40..=0xFF4B => {
                unimplemented!("LCD Control, Status, Position, Scrolling, and Palettes")
            }
            0xFF4F => unimplemented!("VRAM Bank Select"),
            0xFF50 => unimplemented!("Set to non-zero to disable boot ROM"),
            0xFF51..=0xFF55 => unimplemented!("VRAM DMA"),
            0xFF68..=0xFF69 => unimplemented!("BG / OBJ Palettes"),
            0xFF70 => self.wram_bank_idx = data.max(1) as usize,
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize] = data,
            0xFFFF => self.interrupt_enable = data,
            0xE000..=0xFDFF | 0xFEA0..=0xFEFF => {
                panic!("Attempt to access prohibited memory region");
            }
            _ => {
                panic!("Attempt to access unused memory region")
            }
        };
    }
}
