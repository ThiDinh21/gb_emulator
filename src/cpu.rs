use crate::opcodes::{Opcode, CPU_OPCODES_MAP};

pub struct CPU {
    pub program_counter: u16,
    pub status: u8,
    pub register_a: u8,
    pub register_b: u8,
    pub register_c: u8,
    pub register_d: u8,
    pub register_e: u8,
    pub register_h: u8,
    pub register_l: u8,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_b: 0,
            register_c: 0,
            register_d: 0,
            register_e: 0,
            register_h: 0,
            register_l: 0,
            status: 0,
            program_counter: 0,
        }
    }

    pub fn run(&mut self, program: Vec<u8>) {
        let ref all_opcodes = *CPU_OPCODES_MAP;
        loop {
            let opcode = program[self.program_counter as usize];
            self.program_counter += 1;

            match opcode {
                _ => todo!(),
            }
        }
    }
}
