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

    pub fn run(&mut self) {
        let ref all_opcodes = *OPCODES_MAP;

        loop {
            let code = &self.mem_read(self.program_counter);
            self.program_counter += 1;
            let pc_state = self.program_counter;

            let opcode = all_opcodes
                .get(code)
                .expect(&format!("Opcode {:x} is not recognized", code));

            match opcode.code {
                //* 8-bit Load/Store/Move */
                // LD (BC),A
                0x02 => self.set_data_at_bc(self.a),
                // LD B,u8
                0x06 => {
                    let data = self.mem_read(self.program_counter);
                    self.b = data;
                }
                // LD A,(BC)
                0x0A => {
                    let data = self.mem_read(self.get_bc());
                    self.a = data;
                }
                // LD C,u8
                0x0E => {
                    let data = self.mem_read(self.program_counter);
                    self.c = data;
                }
                _ => todo!(""),
            }

            if self.program_counter == pc_state {
                self.program_counter += opcode.bytes as u16 - 1;
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

    /// get the data at the addr stored in register BC
    fn get_data_at_bc(&self) -> u8 {
        self.mem_read(self.get_bc())
    }

    /// set the data to the addr stored in register BC
    fn set_data_at_bc(&mut self, data: u8) {
        self.mem_write(self.get_bc(), data);
    }
}

mod test {
    use super::*;

    // #[test]
    // fn test_ld_addr_bc_a()
}
