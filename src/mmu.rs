use crate::cpu::Mem;

/// Memory map:
/// https://gbdev.io/pandocs/Memory_Map.html
pub struct MMU {
    memory: [u8; 0x8000], // TODO: MBC
    vram: [u8; 0x2000],
    ex_ram: [u8; 0x2000],
    wram: [u8; 0x2000],
    oam: [u8; 0xA0],
    io_regs: [u8; 0x80],
    hram: [u8; 0x7F],
    interrupt_enable: u8,
}

impl Mem for MMU {
    fn mem_read_u8(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.memory[addr as usize],
            0x8000..=0x9FFF => todo!("VRAM"),
            0xA000..=0xBFFF => todo!("Extra RAM"),
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
            0x0000..=0x7FFF => self.memory[addr as usize] = data,
            0x8000..=0x9FFF => todo!("VRAM"),
            0xA000..=0xBFFF => todo!("Extra RAM"),
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

impl MMU {
    pub fn new() -> Self {
        let mut mmu = MMU {
            memory: [0; 0x8000],
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
