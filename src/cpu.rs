use crate::opcodes::OPCODES_MAP;
use bitflags::bitflags;

bitflags! {
    /// https://gbdev.io/pandocs/CPU_Registers_and_Flags.html
    ///
    /// 7     bit     0
    /// ----       ----
    /// Z N H CF 0 0 0 0
    /// | | | |  | | | |
    /// | | | |  | | | |
    /// | | | |  | | | |
    /// | | | |  +-+-+-+- Always 0
    /// | | | +--------- Carry flag
    /// | | + ---------- Half Carry flag (BCD)
    /// | +------------- Substraction flag (BCD)
    /// +--------------- Zero flag
    pub struct StatusFlags: u8 {
        const Z = 0b1000_0000;
        const N = 0b0100_0000;
        const H = 0b0010_0000;
        const C = 0b0001_0000;
    }
}

trait Mem {
    fn mem_read(&self, addr: u16) -> u8;
    fn mem_write(&mut self, addr: u16, data: u8);

    fn mem_read_u16(&self, addr: u16) -> u16 {
        let lo = self.mem_read(addr);
        let hi = self.mem_read(addr.wrapping_add(1));
        u16::from_le_bytes([lo, hi])
    }

    fn mem_write_u16(&mut self, addr: u16, data: u16) {
        let [lo, hi] = data.to_le_bytes();
        self.mem_write(addr, lo);
        self.mem_write(addr.wrapping_add(1), hi);
    }
}

pub struct CPU {
    pub program_counter: u16,
    pub stack_pointer: u16,
    pub status: StatusFlags,
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub memory: [u8; 0xFFFF],
}

impl Mem for CPU {
    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            status: StatusFlags::from_bits_truncate(0x00),
            program_counter: 0,
            stack_pointer: 0,
            memory: [0; 0xFFFF],
        }
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        for i in 0..(program.len() as u16) {
            // TODO: implement MBC
            self.mem_write(i, program[i as usize]);
        }

        self.run();
    }

    fn run(&mut self) {
        let ref all_opcodes = *OPCODES_MAP;

        loop {
            let code = &self.mem_read(self.program_counter);
            self.program_counter += 1;
            let pc_state = self.program_counter;

            let opcode = all_opcodes
                .get(code)
                .expect(&format!("Opcode {:x} is not recognized", code));

            match opcode.code {
                //* 8-bit Load/Store/Move *//
                // LD (BC),A
                0x02 => self.set_data_at_bc(self.a),
                // LD B,u8
                0x06 => {
                    self.b = self.mem_read(self.program_counter);
                }
                // LD A,(BC)
                0x0A => {
                    self.a = self.mem_read(self.get_bc());
                }
                // LD C,u8
                0x0E => {
                    self.c = self.mem_read(self.program_counter);
                }

                // LD (DE),A
                0x12 => {
                    self.set_data_at_de(self.a);
                }
                // LD D,u8
                0x16 => {
                    self.d = self.mem_read(self.program_counter);
                }
                // LD A,(DE)
                0x1A => {
                    self.a = self.get_data_at_de();
                }
                // LD E,u8
                0x1E => {
                    self.e = self.mem_read(self.program_counter);
                }

                // LD (HL+),A
                0x22 => {
                    self.set_data_at_hl(self.a);
                    self.set_hl(self.get_hl().wrapping_add(1));
                }
                // LD H,u8
                0x26 => {
                    self.h = self.mem_read(self.program_counter);
                }
                // LD A,(HL+)
                0x2A => {
                    self.a = self.get_data_at_hl();
                    self.set_hl(self.get_hl().wrapping_add(1));
                }
                // LD L,u8
                0x2E => {
                    self.l = self.mem_read(self.program_counter);
                }

                // LD (HL-),A
                0x32 => {
                    self.set_data_at_hl(self.a);
                    self.set_hl(self.get_hl().wrapping_sub(1));
                }
                // LD (HL),u8
                0x36 => {
                    let data = self.mem_read(self.program_counter);
                    self.set_data_at_hl(data);
                }
                // LD A,(HL-)
                0x3A => {
                    self.a = self.get_data_at_hl();
                    self.set_hl(self.get_hl().wrapping_sub(1));
                }
                // LD A,u8
                0x3E => {
                    self.a = self.mem_read(self.program_counter);
                }

                // LD B,B
                0x40 => { /* NOP */ }
                // LD B,C
                0x41 => self.b = self.c,
                // LD B,D
                0x42 => self.b = self.d,
                // LD B,E
                0x43 => self.b = self.e,
                // LD B,H
                0x44 => self.b = self.h,
                // LD B,L
                0x45 => self.b = self.l,
                // LD B,(HL)
                0x46 => self.b = self.get_data_at_hl(),
                // LD B,A
                0x47 => self.b = self.a,

                // LD C,B
                0x48 => self.c = self.b,
                // LD C,C
                0x49 => { /* NOP */ }
                // LD C,D
                0x4A => self.c = self.d,
                // LD C,E
                0x4B => self.c = self.e,
                // LD C,H
                0x4C => self.c = self.h,
                // LD C,L
                0x4D => self.c = self.l,
                // LD C,(HL)
                0x4E => self.c = self.get_data_at_hl(),
                // LD C,A
                0x4F => self.c = self.a,

                // LD D,B
                0x50 => self.d = self.b,
                // LD D,C
                0x51 => self.d = self.c,
                // LD D,D
                0x52 => { /* NOP */ }
                // LD D,E
                0x53 => self.d = self.e,
                // LD D,H
                0x54 => self.d = self.h,
                // LD D,L
                0x55 => self.d = self.l,
                // LD D,(HL)
                0x56 => self.d = self.get_data_at_hl(),
                // LD D,A
                0x57 => self.d = self.a,

                // LD E,B
                0x58 => self.e = self.b,
                // LD E,C
                0x59 => self.e = self.d,
                // LD E,D
                0x5A => self.e = self.d,
                // LD E,E
                0x5B => { /* NOP */ }
                // LD E,H
                0x5C => self.e = self.h,
                // LD E,L
                0x5D => self.e = self.l,
                // LD E,(HL)
                0x5E => self.e = self.get_data_at_hl(),
                // LD E,A
                0x5F => self.e = self.a,

                // LD H,B
                0x60 => self.h = self.b,
                // LD H,C
                0x61 => self.h = self.c,
                // LD H,D
                0x62 => self.h = self.d,
                // LD H,E
                0x63 => self.h = self.e,
                // LD H,H
                0x64 => { /* NOP */ }
                // LD H,L
                0x65 => self.h = self.l,
                // LD H,(HL)
                0x66 => self.h = self.get_data_at_hl(),
                // LD H,A
                0x67 => self.h = self.a,

                // LD L,B
                0x68 => self.l = self.b,
                // LD L,C
                0x69 => self.l = self.d,
                // LD L,D
                0x6A => self.l = self.d,
                // LD L,E
                0x6B => self.l = self.e,
                // LD L,H
                0x6C => self.l = self.h,
                // LD L,L
                0x6D => { /* NOP */ }
                // LD L,(HL)
                0x6E => self.l = self.get_data_at_hl(),
                // LD L,A
                0x6F => self.l = self.a,

                // LD (HL),B
                0x70 => self.set_data_at_hl(self.b),
                // LD (HL),C
                0x71 => self.set_data_at_hl(self.c),
                // LD (HL),D
                0x72 => self.set_data_at_hl(self.d),
                // LD (HL),E
                0x73 => self.set_data_at_hl(self.e),
                // LD (HL),H
                0x74 => self.set_data_at_hl(self.h),
                // LD (HL),L
                0x75 => self.set_data_at_hl(self.l),
                // LD (HL),A
                0x77 => self.set_data_at_hl(self.a),

                // LD A,B
                0x78 => self.a = self.b,
                // LD A,C
                0x79 => self.a = self.d,
                // LD A,D
                0x7A => self.a = self.d,
                // LD A,E
                0x7B => self.a = self.e,
                // LD A,H
                0x7C => self.a = self.h,
                // LD A,L
                0x7D => self.a = self.l,
                // LD A,(HL)
                0x7E => self.a = self.get_data_at_hl(),
                // LD A,A
                0x7F => { /* NOP */ }

                // LD (FF00+u8),A
                0xE0 => {
                    let operand = self.mem_read(self.program_counter);
                    let addr = 0xFF00_u16.wrapping_add(operand as u16);
                    self.mem_write(addr, self.a);
                }
                // LD A,(FF00+u8)
                0xF0 => {
                    let operand = self.mem_read(self.program_counter);
                    let addr = 0xFF00_u16.wrapping_add(operand as u16);
                    self.a = self.mem_read(addr);
                }
                // LD (FF00+C),A
                0xE2 => {
                    let addr = 0xFF00_u16 + (self.c as u16);
                    self.mem_write(addr, self.a);
                }
                // LD A,(FF00+C)
                0xF2 => {
                    let addr = 0xFF00_u16 + (self.c as u16);
                    self.a = self.mem_read(addr);
                }
                // LD (u16),A
                0xEA => {
                    let addr = self.mem_read_u16(self.program_counter);
                    self.mem_write(addr, self.a);
                }
                // LD A,(u16)
                0xFA => {
                    let addr = self.mem_read_u16(self.program_counter);
                    self.a = self.mem_read(addr);
                }

                //* control/branch *//
                // NOP
                0x00 => { /* NOP */ }
                // STOP
                0x10 => return,
                // HALT
                0x76 => todo!("Impl HALT!"),
                _ => todo!(""),
            }

            if self.program_counter == pc_state {
                self.program_counter += opcode.bytes as u16 - 1;
            }
        }
    }

    //* Getters and Setters *//
    fn get_a(&self) -> u8 {
        self.a
    }

    fn get_b(&self) -> u8 {
        self.a
    }

    fn get_c(&self) -> u8 {
        self.a
    }

    fn get_d(&self) -> u8 {
        self.a
    }

    fn get_e(&self) -> u8 {
        self.a
    }

    fn get_h(&self) -> u8 {
        self.a
    }

    fn get_l(&self) -> u8 {
        self.a
    }

    /// get the register BC
    /// B hi, C lo
    fn get_bc(&self) -> u16 {
        u16::from_le_bytes([self.c, self.b])
    }

    /// set the register BC with data
    /// B hi, C lo
    fn set_bc(&mut self, data: u16) {
        let [lo, hi] = data.to_le_bytes();
        self.b = hi;
        self.c = lo;
    }

    /// get the register DE
    /// D hi, E lo
    fn get_de(&self) -> u16 {
        u16::from_le_bytes([self.e, self.d])
    }

    /// set the register DE with data
    /// D hi, E lo
    fn set_de(&mut self, data: u16) {
        let [lo, hi] = data.to_le_bytes();
        self.d = hi;
        self.e = lo;
    }

    /// get the register HL
    /// H hi, L lo
    fn get_hl(&self) -> u16 {
        u16::from_le_bytes([self.l, self.h])
    }

    /// set the register HL with data
    /// H hi, L lo
    fn set_hl(&mut self, data: u16) {
        let [lo, hi] = data.to_le_bytes();
        self.h = hi;
        self.l = lo;
    }

    fn get_zf(&self) -> bool {
        self.status.contains(StatusFlags::Z)
    }

    fn get_nf(&self) -> bool {
        self.status.contains(StatusFlags::N)
    }

    fn get_hf(&self) -> bool {
        self.status.contains(StatusFlags::H)
    }

    fn get_cf(&self) -> bool {
        self.status.contains(StatusFlags::C)
    }

    fn get_sp(&self) -> u16 {
        self.stack_pointer
    }

    fn set_sp(&mut self, v: u16) {
        self.stack_pointer = v;
    }

    //* Register related methods *//

    /// get the data at the addr stored in register BC
    fn get_data_at_bc(&self) -> u8 {
        self.mem_read(self.get_bc())
    }

    /// set the data to the addr stored in register BC
    fn set_data_at_bc(&mut self, data: u8) {
        self.mem_write(self.get_bc(), data);
    }

    /// get the data at the addr stored in register DE
    fn get_data_at_de(&self) -> u8 {
        self.mem_read(self.get_de())
    }

    /// set the data to the addr stored in register DE
    fn set_data_at_de(&mut self, data: u8) {
        self.mem_write(self.get_de(), data);
    }

    /// get the data at the addr stored in register HL
    fn get_data_at_hl(&self) -> u8 {
        self.mem_read(self.get_hl())
    }

    /// set the data to the addr stored in register HL
    fn set_data_at_hl(&mut self, data: u8) {
        self.mem_write(self.get_hl(), data);
    }
}

mod test {

    mod load_store_move_8_bit {
        #[allow(unused_imports)]
        use super::super::*;

        #[test]
        fn test_ld_addr_bc_a_0x02() {
            let mut cpu = CPU::new();
            cpu.b = 0x00;
            cpu.c = 0x05;
            cpu.a = 0x69;
            cpu.load_and_run(vec![0x02, 0x10]);

            assert_eq!(cpu.get_data_at_bc(), cpu.a);
        }

        #[test]
        fn test_ld_b_u8_0x06() {
            let mut cpu = CPU::new();
            cpu.b = 0x0E;
            let data = 0xEF_u8;
            cpu.load_and_run(vec![0x06, data, 0x10]);

            assert_eq!(cpu.b, data);
        }

        #[test]
        fn test_ld_a_addr_bc_0x0a() {
            let mut cpu = CPU::new();
            cpu.b = 0x00;
            cpu.c = 0x05;
            cpu.set_data_at_bc(0x99);
            cpu.load_and_run(vec![0x0A, 0x10]);

            assert_eq!(cpu.get_data_at_bc(), cpu.a);
        }

        #[test]
        fn test_ld_c_u8_0x0e() {
            let mut cpu = CPU::new();
            cpu.c = 0x0E;
            let data = 0xEF_u8;
            cpu.load_and_run(vec![0x0e, data, 0x10]);

            assert_eq!(cpu.c, data);
        }

        #[test]
        fn test_ld_addr_de_a_0x12() {
            let mut cpu = CPU::new();
            cpu.d = 0x00;
            cpu.e = 0x05;
            cpu.a = 0x69;
            cpu.load_and_run(vec![0x12, 0x10]);

            assert_eq!(cpu.get_data_at_de(), cpu.a);
        }

        #[test]
        fn test_ld_d_u8_0x16() {
            let mut cpu = CPU::new();
            cpu.d = 0x0E;
            let data = 0xEF_u8;
            cpu.load_and_run(vec![0x16, data, 0x10]);

            assert_eq!(cpu.d, data);
        }

        #[test]
        fn test_ld_a_addr_de_0x1a() {
            let mut cpu = CPU::new();
            cpu.d = 0x00;
            cpu.e = 0x05;
            cpu.set_data_at_de(0x99);
            cpu.load_and_run(vec![0x1A, 0x10]);

            assert_eq!(cpu.get_data_at_de(), cpu.a);
        }

        #[test]
        fn test_ld_e_u8_0x1e() {
            let mut cpu = CPU::new();
            cpu.e = 0x0E;
            let data = 0xEF_u8;
            cpu.load_and_run(vec![0x1e, data, 0x10]);

            assert_eq!(cpu.e, data);
        }

        #[test]
        fn test_ld_addr_hl_incr_a_0x22() {
            let mut cpu = CPU::new();
            let data = 0x99;

            cpu.h = 0x00;
            cpu.l = 0x05;
            let old_hl = cpu.get_hl();
            cpu.a = data;
            cpu.load_and_run(vec![0x22, 0x10]);

            assert_eq!(data, cpu.mem_read(old_hl));
            assert_eq!(cpu.get_hl(), 0x0006);
        }

        #[test]
        fn test_ld_a_addr_hl_incr_0x2a() {
            let mut cpu = CPU::new();
            let data = 0x99;

            cpu.h = 0x00;
            cpu.l = 0x05;
            cpu.set_data_at_hl(data);
            cpu.load_and_run(vec![0x2A, 0x10]);

            assert_eq!(data, cpu.a);
            assert_eq!(cpu.get_hl(), 0x0006);
        }

        #[test]
        fn test_ld_addr_hl_decr_a_0x32() {
            let mut cpu = CPU::new();
            let data = 0x99;

            cpu.h = 0x00;
            cpu.l = 0x05;
            let old_hl = cpu.get_hl();
            cpu.a = data;
            cpu.load_and_run(vec![0x32, 0x10]);

            assert_eq!(data, cpu.mem_read(old_hl));
            assert_eq!(cpu.get_hl(), 0x0004);
        }

        #[test]
        fn test_ld_a_addr_hl_decr_0x3a() {
            let mut cpu = CPU::new();
            let data = 0x99;

            cpu.h = 0x00;
            cpu.l = 0x05;
            cpu.set_data_at_hl(data);
            cpu.load_and_run(vec![0x3A, 0x10]);

            assert_eq!(data, cpu.a);
            assert_eq!(cpu.get_hl(), 0x0004);
        }

        #[test]
        fn test_ld_addr_ff00_u8_a_0xe0() {
            let mut cpu = CPU::new();
            let data = 0x99;

            cpu.a = data;
            cpu.load_and_run(vec![0xE0, 0x10, 0x10]);

            assert_eq!(data, cpu.mem_read(0xFF10));
        }

        #[test]
        fn test_ld_a_addr_ff00_u8_0xf0() {
            let mut cpu = CPU::new();
            let data = 0x99;

            cpu.mem_write(0xFF10, data);
            cpu.load_and_run(vec![0xF0, 0x10, 0x10]);

            assert_eq!(data, cpu.a);
        }

        #[test]
        fn test_ld_addr_ff00_c_a_0xe2() {
            let mut cpu = CPU::new();
            let data = 0x99;

            cpu.c = 0x10;
            cpu.a = data;
            cpu.load_and_run(vec![0xE2, 0x10]);

            assert_eq!(data, cpu.mem_read(0xFF10));
        }

        #[test]
        fn test_ld_a_addr_ff00_c_0xf2() {
            let mut cpu = CPU::new();
            let data = 0x99;

            cpu.mem_write(0xFF10, data);
            cpu.c = 0x10;
            cpu.load_and_run(vec![0xF2, 0x10]);

            assert_eq!(data, cpu.a);
        }

        #[test]
        fn test_ld_addr_u16_a_0xea() {
            let mut cpu = CPU::new();
            let data = 0x99;

            cpu.a = data;
            cpu.load_and_run(vec![0xEA, 0x79, 0xAA, 0x10]);

            assert_eq!(data, cpu.mem_read(0xAA79));
        }

        #[test]
        fn test_ld_a_addr_u16_0xfa() {
            let mut cpu = CPU::new();
            let data = 0x99;

            cpu.mem_write(0xAA79, data);
            cpu.load_and_run(vec![0xFA, 0x79, 0xAA, 0x10]);

            assert_eq!(data, cpu.a);
        }
    }
}
