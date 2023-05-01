use std::path::PathBuf;

use crate::{
    cartridge::{get_mbc, MBC},
    cpu::Mem,
};

/// Memory map:
/// https://gbdev.io/pandocs/Memory_Map.html
pub struct MMU {
    mbc: Box<dyn MBC + 'static>,
    vram: [u8; 0x2000],
    wram: [u8; 0x2000],
    oam: [u8; 0xA0],
    io_regs: [u8; 0x80],
    hram: [u8; 0x7F],
    interrupt_enable: u8,
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
            wram: [0; 0x2000],
            oam: [0; 0xA0],
            io_regs: [0; 0x80],
            hram: [0; 0x7F],
            interrupt_enable: 0,
        };
        // mmu.initiate();
        mmu
    }

    // pub fn empty() -> Self {
    //     MMU {
    //         memory: [0; 0x8000],
    //         vram: [0; 0x2000],
    //         ex_ram: [0; 0x2000],
    //         wram: [0; 0x2000],
    //         oam: [0; 0xA0],
    //         io_regs: [0; 0x80],
    //         hram: [0; 0x7F],
    //         interrupt_enable: 0,
    //     }
    // }

    fn initiate(&mut self) {
        todo!()
    }
}

impl Mem for MMU {
    fn mem_read_u8(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.mbc.read_rom(addr),
            0x8000..=0x9FFF => todo!("VRAM"),
            0xA000..=0xBFFF => self.mbc.read_ram(addr),
            0xC000..=0xDFFF => todo!("Work RAM"),
            0xFE00..=0xFE9F => todo!("OAM"),
            0xFF00..=0xFF7F => todo!(),
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize],
            0xFFFF => self.interrupt_enable,
            0xE000..=0xFDFF | 0xFEA0..=0xFEFF => {
                panic!("Attempt to access prohibited memory region")
            }
        }
    }

    fn mem_write_u8(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..=0x7FFF => self.mbc.write_rom(addr, data),
            0x8000..=0x9FFF => todo!("VRAM"),
            0xA000..=0xBFFF => self.mbc.write_ram(addr, data),
            0xC000..=0xDFFF => todo!("Work RAM"),
            0xFE00..=0xFE9F => todo!("OAM"),
            0xFF00..=0xFF7F => todo!(),
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize] = data,
            0xFFFF => self.interrupt_enable = data,
            0xE000..=0xFDFF | 0xFEA0..=0xFEFF => {
                panic!("Attempt to access prohibited memory region");
            }
        };
    }
}
