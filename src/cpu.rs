use crate::opcodes::OPCODES_MAP;

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
    pub status: u8,
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
            status: 0,
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
                    self.l = self.l.wrapping_add(1);
                }
                // LD H,u8
                0x26 => {
                    self.h = self.mem_read(self.program_counter);
                }
                // LD A,(HL+)
                0x2A => {
                    self.a = self.get_data_at_hl();
                    self.l = self.l.wrapping_add(1);
                }
                // LD L,u8
                0x2E => {
                    self.l = self.mem_read(self.program_counter);
                }

                //* control/branch *//
                // STOP
                0x10 => return,
                _ => todo!(""),
            }

            if self.program_counter == pc_state {
                self.program_counter += opcode.bytes as u16 - 1;
            }
        }
    }

    //* Register related methods *//

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

    /// get the data at the addr stored in register BC
    fn get_data_at_bc(&self) -> u8 {
        self.mem_read(self.get_bc())
    }

    /// set the data to the addr stored in register BC
    fn set_data_at_bc(&mut self, data: u8) {
        self.mem_write(self.get_bc(), data);
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

    /// get the data at the addr stored in register DE
    fn get_data_at_de(&self) -> u8 {
        self.mem_read(self.get_de())
    }

    /// set the data to the addr stored in register DE
    fn set_data_at_de(&mut self, data: u8) {
        self.mem_write(self.get_de(), data);
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
        fn test_la_a_addr_bc_0x0a() {
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
        fn test_la_a_addr_de_0x1a() {
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
    }
}
