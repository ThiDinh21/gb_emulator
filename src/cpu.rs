use crate::opcodes::CPU_OPCODES;
use bitflags::bitflags;

const STACK_BOTTOM: u16 = 0x0100;
const STACK_PTR_RESET: u8 = 0xFD;

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

pub trait Mem {
    fn mem_read_u8(&self, addr: u16) -> u8;
    fn mem_write_u8(&mut self, addr: u16, data: u8);

    fn mem_read_u16(&self, addr: u16) -> u16 {
        let lo = self.mem_read_u8(addr);
        let hi = self.mem_read_u8(addr.wrapping_add(1));
        u16::from_le_bytes([lo, hi])
    }

    fn mem_write_u16(&mut self, addr: u16, data: u16) {
        let [lo, hi] = data.to_le_bytes();
        self.mem_write_u8(addr, lo);
        self.mem_write_u8(addr.wrapping_add(1), hi);
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
    fn mem_read_u8(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write_u8(&mut self, addr: u16, data: u8) {
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
            self.mem_write_u8(i, program[i as usize]);
        }

        self.run();
    }

    fn run(&mut self) {
        let ref all_opcodes = *CPU_OPCODES;

        loop {
            let code = self.fetch_opcode();

            self.program_counter += 1;
            let pc_state = self.program_counter;

            let opcode = all_opcodes
                .get(&code)
                .expect(&format!("Opcode {:x} is not recognized", code));

            let time = self.decode(opcode);

            if self.program_counter == pc_state {
                self.program_counter += opcode.bytes as u16 - 1;
            }
        }
    }

    pub fn fetch_opcode(&mut self) -> u16 {
        let op = self.mem_read_u8(self.program_counter);

        if op != 0xCB {
            op as u16
        } else {
            0xCB_u16 << 8 | op as u16
        }
    }

    //* Getters and Setters *//
    pub fn get_a(&self) -> u8 {
        self.a
    }

    pub fn get_b(&self) -> u8 {
        self.a
    }

    pub fn get_c(&self) -> u8 {
        self.a
    }

    pub fn get_d(&self) -> u8 {
        self.a
    }

    pub fn get_e(&self) -> u8 {
        self.a
    }

    pub fn get_h(&self) -> u8 {
        self.a
    }

    pub fn get_l(&self) -> u8 {
        self.a
    }

    pub fn get_af(&self) -> u16 {
        todo!()
    }

    /// get the register BC
    /// B hi, C lo
    pub fn get_bc(&self) -> u16 {
        u16::from_le_bytes([self.c, self.b])
    }

    /// get the register DE
    /// D hi, E lo
    pub fn get_de(&self) -> u16 {
        u16::from_le_bytes([self.e, self.d])
    }

    /// get the register HL
    /// H hi, L lo
    pub fn get_hl(&self) -> u16 {
        u16::from_le_bytes([self.l, self.h])
    }

    pub fn get_zf(&self) -> bool {
        self.status.contains(StatusFlags::Z)
    }

    pub fn get_nf(&self) -> bool {
        self.status.contains(StatusFlags::N)
    }

    pub fn get_hf(&self) -> bool {
        self.status.contains(StatusFlags::H)
    }

    pub fn get_cf(&self) -> bool {
        self.status.contains(StatusFlags::C)
    }

    pub fn get_sp(&self) -> u16 {
        self.stack_pointer
    }

    pub fn set_a(&mut self, v: u8) {
        self.a = v;
    }

    pub fn set_b(&mut self, v: u8) {
        self.b = v;
    }
    pub fn set_c(&mut self, v: u8) {
        self.c = v;
    }
    pub fn set_d(&mut self, v: u8) {
        self.d = v;
    }
    pub fn set_e(&mut self, v: u8) {
        self.e = v;
    }
    pub fn set_h(&mut self, v: u8) {
        self.h = v;
    }
    pub fn set_l(&mut self, v: u8) {
        self.l = v;
    }

    /// set the register BC with data
    /// B hi, C lo
    pub fn set_bc(&mut self, data: u16) {
        let [lo, hi] = data.to_le_bytes();
        self.b = hi;
        self.c = lo;
    }

    /// set the register DE with data
    /// D hi, E lo
    pub fn set_de(&mut self, data: u16) {
        let [lo, hi] = data.to_le_bytes();
        self.d = hi;
        self.e = lo;
    }

    /// set the register HL with data
    /// H hi, L lo
    pub fn set_hl(&mut self, data: u16) {
        let [lo, hi] = data.to_le_bytes();
        self.h = hi;
        self.l = lo;
    }

    pub fn set_af(&mut self, v: u16) {
        todo!()
    }

    pub fn set_sp(&mut self, v: u16) {
        self.stack_pointer = v;
    }

    //* Register related methods *//

    /// get the data at the addr stored in register BC
    fn get_data_at_bc(&self) -> u8 {
        self.mem_read_u8(self.get_bc())
    }

    /// set the data to the addr stored in register BC
    fn set_data_at_bc(&mut self, data: u8) {
        self.mem_write_u8(self.get_bc(), data);
    }

    /// get the data at the addr stored in register DE
    fn get_data_at_de(&self) -> u8 {
        self.mem_read_u8(self.get_de())
    }

    /// set the data to the addr stored in register DE
    fn set_data_at_de(&mut self, data: u8) {
        self.mem_write_u8(self.get_de(), data);
    }

    /// get the data at the addr stored in register HL
    fn get_data_at_hl(&self) -> u8 {
        self.mem_read_u8(self.get_hl())
    }

    /// set the data to the addr stored in register HL
    fn set_data_at_hl(&mut self, data: u8) {
        self.mem_write_u8(self.get_hl(), data);
    }

    pub fn enable_interrupt(&mut self) {
        todo!();
    }

    pub fn disable_interrupt(&mut self) {
        todo!();
    }

    pub fn cpu_jr(&mut self) {
        todo!();
    }

    pub fn halt(&mut self) {
        todo!();
    }

    pub fn stop(&mut self) {
        todo!();
    }

    //* Stack methods *//
    pub fn stack_push(&mut self, data: u16) {
        self.stack_pointer -= 2;
        self.validate_sp();
        self.mem_write_u16(self.stack_pointer, data);
    }

    pub fn stack_pop(&mut self) -> u16 {
        let res = self.mem_read_u16(self.stack_pointer);

        self.stack_pointer += 2;
        self.validate_sp();

        res
    }

    fn validate_sp(&self) {
        if self.get_sp() > 0xFFFE {
            panic!("Stack underflow");
        } else if self.get_sp() < 0xFF80 {
            panic!("Stack overflow");
        }
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

            assert_eq!(data, cpu.mem_read_u8(old_hl));
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

            assert_eq!(data, cpu.mem_read_u8(old_hl));
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

            assert_eq!(data, cpu.mem_read_u8(0xFF10));
        }

        #[test]
        fn test_ld_a_addr_ff00_u8_0xf0() {
            let mut cpu = CPU::new();
            let data = 0x99;

            cpu.mem_write_u8(0xFF10, data);
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

            assert_eq!(data, cpu.mem_read_u8(0xFF10));
        }

        #[test]
        fn test_ld_a_addr_ff00_c_0xf2() {
            let mut cpu = CPU::new();
            let data = 0x99;

            cpu.mem_write_u8(0xFF10, data);
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

            assert_eq!(data, cpu.mem_read_u8(0xAA79));
        }

        #[test]
        fn test_ld_a_addr_u16_0xfa() {
            let mut cpu = CPU::new();
            let data = 0x99;

            cpu.mem_write_u8(0xAA79, data);
            cpu.load_and_run(vec![0xFA, 0x79, 0xAA, 0x10]);

            assert_eq!(data, cpu.a);
        }
    }
}
