use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct Opcode {
    pub code: u8,
    pub mnemonic: &'static str,
    pub bytes: u8,
    pub cycles: u8,
}

impl Opcode {
    pub fn new(code: u8, mnemonic: &'static str, bytes: u8, cycles: u8) -> Self {
        Opcode {
            code,
            mnemonic,
            bytes,
            cycles,
        }
    }
}

lazy_static! {
    pub static ref CPU_OPCODES: Vec<Opcode> = vec![
        // 8-bit Load/Store/Move
        Opcode::new(0x02, "LD (BC), A", 1, 8),
        Opcode::new(0x06, "LD B, d8", 2, 8),
        Opcode::new(0x0A, "LD A, (BC)", 1, 8),
        Opcode::new(0x0E, "LD C, d8", 2, 8),
        // Opcode::new(, , , ),
    ];
    pub static ref OPCODES_MAP: HashMap<u8, &'static Opcode> = {
        let mut map = HashMap::new();
        for cpu_op in CPU_OPCODES.iter() {
            map.insert(cpu_op.code, cpu_op);
        }
        map
    };
}
