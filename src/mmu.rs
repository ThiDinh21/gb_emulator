use crate::{cartridge::Cartridge, cpu::Mem};

const VRAM: u16 = 0x8000;
const VRAM_END: u16 = 0x9FFF;
const EX_RAM: u16 = 0xA000;
const EX_RAM_END: u16 = 0xBFFF;
const WRAM: u16 = 0xC000;
const WRAM_END: u16 = 0xDFFF;
const OAM: u16 = 0xFE00;
const OAM_END: u16 = 0xFE9F;
const IO_REG: u16 = 0xFF00;
const IO_REG_END: u16 = 0xFF7F;
const HRAM: u16 = 0xFF80;
const HRAM_END: u16 = 0xFFFE;

/// Memory map:
/// https://gbdev.io/pandocs/Memory_Map.html
pub struct MMU {
    memory: [u8; 0x8000], // TODO: MBC
    vram: [u8; 0x2000],
    ex_ram: [u8; 0x2000],
    wram: [u8; 0x2000], // TODO: MBC
    oam: [u8; 0xA0],
    io_regs: [u8; 0x80],
    hram: [u8; 0x7F],
    interrupt_enable: u8,
}

impl Mem for MMU {
    fn mem_read_u8(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.memory[addr as usize],
            VRAM..=VRAM_END => todo!("VRAM"),
            EX_RAM..=EX_RAM_END => todo!("Extra RAM"),
            WRAM..=WRAM_END => todo!("Work RAM"),
            OAM..=OAM_END => todo!("OAM"),
            IO_REG..=IO_REG_END => todo!(),
            HRAM..=HRAM_END => self.hram[(addr - 0xFF80) as usize],
            0xFFFF => self.interrupt_enable,
            0xE000..=0xFDFF | 0xFEA0..=0xFEFF => {
                panic!("Attempt to access prohibited memory region")
            }
        }
    }

    fn mem_write_u8(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..=0x7FFF => self.memory[addr as usize] = data,
            VRAM..=VRAM_END => todo!("VRAM"),
            EX_RAM..=EX_RAM_END => todo!("Extra RAM"),
            WRAM..=WRAM_END => todo!("Work RAM"),
            OAM..=OAM_END => todo!("OAM"),
            IO_REG..=IO_REG_END => todo!(),
            HRAM..=HRAM_END => self.hram[(addr - 0xFF80) as usize] = data,
            0xFFFF => self.interrupt_enable = data,
            0xE000..=0xFDFF | 0xFEA0..=0xFEFF => {
                panic!("Attempt to access prohibited memory region");
            }
        };
    }
}

impl MMU {
    pub fn new(rom: Cartridge) -> Self {
        let memory: [u8; 0x8000] = (*rom.prg_rom.into_boxed_slice())
            .try_into()
            .expect("ROM has wrong size");

        let mut mmu = MMU {
            memory,
            vram: [0; 0x2000],
            ex_ram: [0; 0x2000],
            wram: [0; 0x2000],
            oam: [0; 0xA0],
            io_regs: [0; 0x80],
            hram: [0; 0x7F],
            interrupt_enable: 0,
        };
        // mmu.initiate();
        mmu
    }

    pub fn empty() -> Self {
        MMU {
            memory: [0; 0x8000],
            vram: [0; 0x2000],
            ex_ram: [0; 0x2000],
            wram: [0; 0x2000],
            oam: [0; 0xA0],
            io_regs: [0; 0x80],
            hram: [0; 0x7F],
            interrupt_enable: 0,
        }
    }

    fn initiate(&mut self) {
        todo!()
    }
}
