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
        Opcode::new(0x02, "LD (BC),A", 1, 8),
        Opcode::new(0x06, "LD B,u8", 2, 8),
        Opcode::new(0x0A, "LD A,(BC)", 1, 8),
        Opcode::new(0x0E, "LD C,u8", 2, 8),

        Opcode::new(0x12, "LD (DE),A", 1, 8),
        Opcode::new(0x16, "LD D,u8", 2, 8),
        Opcode::new(0x1A, "LD A,(DE)", 1, 8),
        Opcode::new(0x1E, "LD E,u8", 2, 8),

        Opcode::new(0x22, "LD (HL+),A", 1, 8),
        Opcode::new(0x26, "LD H,u8", 2, 8),
        Opcode::new(0x2A, "LD A,(HL+)", 1, 8),
        Opcode::new(0x2E, "LD L,u8", 2, 8),

        Opcode::new(0x32, "LD (HL-),A", 1, 8),
        Opcode::new(0x36, "LD (HL),u8", 2, 8),
        Opcode::new(0x3A, "LD A,(HL-)", 1, 8),
        Opcode::new(0x3E, "LD A,u8", 2, 8),

        Opcode::new(0x40, "LD B,B", 1, 4),
        Opcode::new(0x41, "LD B,C", 1, 4),
        Opcode::new(0x42, "LD B,D", 1, 4),
        Opcode::new(0x43, "LD B,E", 1, 4),
        Opcode::new(0x44, "LD B,H", 1, 4),
        Opcode::new(0x45, "LD B,L", 1, 4),
        Opcode::new(0x46, "LD B,(HL)", 1, 4),
        Opcode::new(0x47, "LD B,A", 1, 4),

        Opcode::new(0x48, "LD C,B", 1, 4),
        Opcode::new(0x49, "LD C,C", 1, 4),
        Opcode::new(0x4A, "LD C,D", 1, 4),
        Opcode::new(0x4B, "LD C,E", 1, 4),
        Opcode::new(0x4C, "LD C,H", 1, 4),
        Opcode::new(0x4D, "LD C,L", 1, 4),
        Opcode::new(0x4E, "LD C,(HL)", 1, 4),
        Opcode::new(0x4F, "LD C,A", 1, 4),

        Opcode::new(0x50, "LD D,B", 1, 4),
        Opcode::new(0x51, "LD D,C", 1, 4),
        Opcode::new(0x52, "LD D,D", 1, 4),
        Opcode::new(0x53, "LD D,E", 1, 4),
        Opcode::new(0x54, "LD D,H", 1, 4),
        Opcode::new(0x55, "LD D,L", 1, 4),
        Opcode::new(0x56, "LD D,(HL)", 1, 4),
        Opcode::new(0x57, "LD D,A", 1, 4),

        Opcode::new(0x58, "LD E,B", 1, 4),
        Opcode::new(0x59, "LD E,C", 1, 4),
        Opcode::new(0x5A, "LD E,D", 1, 4),
        Opcode::new(0x5B, "LD E,E", 1, 4),
        Opcode::new(0x5C, "LD E,H", 1, 4),
        Opcode::new(0x5D, "LD E,L", 1, 4),
        Opcode::new(0x5E, "LD E,(HL)", 1, 4),
        Opcode::new(0x5F, "LD E,A", 1, 4),

        // Opcode::new(, , , ),

        // control/branch
        Opcode::new(0x10, "STOP", 1, 4),
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
