use crate::opcodes::{Opcode, OPCODES_MAP};

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
        }
    }

    pub fn run(&mut self, program: Vec<u8>) {
        let ref all_opcodes = *OPCODES_MAP;
        loop {
            let code = &program[self.program_counter as usize];
            self.program_counter += 1;
            let pc_state = self.program_counter;

            let opcode = all_opcodes
                .get(code)
                .expect(&format!("Opcode {:x} is not recognized", code));

            match opcode.code {
                //* 8-bit Load/Store/Move */
                // LD (BC),A
                0x02 => self.set_bc(self.a as u16),
                // LD B,u8
                0x06 => {
                    let data = program[self.program_counter as usize];
                    self.b = data;
                }
                // LD A,(BC)
                // 0x0xA
                _ => todo!(""),
            }

            if self.program_counter == pc_state {
                self.program_counter += opcode.bytes as u16;
            }
        }
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
}
