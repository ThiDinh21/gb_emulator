use crate::alu;
use crate::cpu::{Mem, StatusFlags, CPU};
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct Opcode {
    pub code: u16,
    pub mnemonic: &'static str,
    pub bytes: u8,
}

impl Opcode {
    pub fn new(code: u16, mnemonic: &'static str, bytes: u8) -> Self {
        Opcode {
            code,
            mnemonic,
            bytes,
        }
    }
}

lazy_static! {
    pub static ref OPCODES_LIST: Vec<Opcode> = vec![
        Opcode::new(0x0000, "NOP", 1),
        Opcode::new(0x0001, "LD BC,u16", 3),
        Opcode::new(0x0002, "LD (BC),A", 1),
        Opcode::new(0x0003, "INC BC", 1),
        Opcode::new(0x0004, "INC B", 1),
        Opcode::new(0x0005, "DEC B", 1),
        Opcode::new(0x0006, "LD B,u8", 2),
        Opcode::new(0x0007, "RLCA", 1),
        Opcode::new(0x0008, "LD (u16),SP", 3),
        Opcode::new(0x0009, "ADD HL,BC", 1),
        Opcode::new(0x000A, "LD A,(BC)", 1),
        Opcode::new(0x000B, "DEC BC", 1),
        Opcode::new(0x000C, "INC C", 1),
        Opcode::new(0x000D, "DEC C", 1),
        Opcode::new(0x000E, "LD C,u8", 2),
        Opcode::new(0x000F, "RRCA", 1),
        Opcode::new(0x0010, "STOP", 1),
        Opcode::new(0x0011, "LD DE,u16", 3),
        Opcode::new(0x0012, "LD (DE),A", 1),
        Opcode::new(0x0013, "INC DE", 1),
        Opcode::new(0x0014, "INC D", 1),
        Opcode::new(0x0015, "DEC D", 1),
        Opcode::new(0x0016, "LD D,u8", 2),
        Opcode::new(0x0017, "RLA", 1),
        Opcode::new(0x0018, "JR i8", 2),
        Opcode::new(0x0019, "ADD HL,DE", 1),
        Opcode::new(0x001A, "LD A,(DE)", 1),
        Opcode::new(0x001B, "DEC DE", 1),
        Opcode::new(0x001C, "INC E", 1),
        Opcode::new(0x001D, "DEC E", 1),
        Opcode::new(0x001E, "LD E,u8", 2),
        Opcode::new(0x001F, "RRA", 1),
        Opcode::new(0x0020, "JR NZ,i8", 2),
        Opcode::new(0x0021, "LD HL,u16", 3),
        Opcode::new(0x0022, "LD (HL+),A", 1),
        Opcode::new(0x0023, "INC HL", 1),
        Opcode::new(0x0024, "INC H", 1),
        Opcode::new(0x0025, "DEC H", 1),
        Opcode::new(0x0026, "LD H,u8", 2),
        Opcode::new(0x0027, "DAA", 1),
        Opcode::new(0x0028, "JR Z,i8", 2),
        Opcode::new(0x0029, "ADD HL,HL", 1),
        Opcode::new(0x002A, "LD A,(HL+)", 1),
        Opcode::new(0x002B, "DEC HL", 1),
        Opcode::new(0x002C, "INC L", 1),
        Opcode::new(0x002D, "DEC L", 1),
        Opcode::new(0x002E, "LD L,u8", 2),
        Opcode::new(0x002F, "CPL", 1),
        Opcode::new(0x0030, "JR NC,i8", 2),
        Opcode::new(0x0031, "LD SP,u16", 3),
        Opcode::new(0x0032, "LD (HL-),A", 1),
        Opcode::new(0x0033, "INC SP", 1),
        Opcode::new(0x0034, "INC (HL)", 1),
        Opcode::new(0x0035, "DEC (HL)", 1),
        Opcode::new(0x0036, "LD (HL),u8", 2),
        Opcode::new(0x0037, "SCF", 1),
        Opcode::new(0x0038, "JR C,i8", 2),
        Opcode::new(0x0039, "ADD HL,SP", 1),
        Opcode::new(0x003A, "LD A,(HL-)", 1),
        Opcode::new(0x003B, "DEC SP", 1),
        Opcode::new(0x003C, "INC A", 1),
        Opcode::new(0x003D, "DEC A", 1),
        Opcode::new(0x003E, "LD A,u8", 2),
        Opcode::new(0x003F, "CCF", 1),
        Opcode::new(0x0040, "LD B,B", 1),
        Opcode::new(0x0041, "LD B,C", 1),
        Opcode::new(0x0042, "LD B,D", 1),
        Opcode::new(0x0043, "LD B,E", 1),
        Opcode::new(0x0044, "LD B,H", 1),
        Opcode::new(0x0045, "LD B,L", 1),
        Opcode::new(0x0046, "LD B,(HL)", 1),
        Opcode::new(0x0047, "LD B,A", 1),
        Opcode::new(0x0048, "LD C,B", 1),
        Opcode::new(0x0049, "LD C,C", 1),
        Opcode::new(0x004A, "LD C,D", 1),
        Opcode::new(0x004B, "LD C,E", 1),
        Opcode::new(0x004C, "LD C,H", 1),
        Opcode::new(0x004D, "LD C,L", 1),
        Opcode::new(0x004E, "LD C,(HL)", 1),
        Opcode::new(0x004F, "LD C,A", 1),
        Opcode::new(0x0050, "LD D,B", 1),
        Opcode::new(0x0051, "LD D,C", 1),
        Opcode::new(0x0052, "LD D,D", 1),
        Opcode::new(0x0053, "LD D,E", 1),
        Opcode::new(0x0054, "LD D,H", 1),
        Opcode::new(0x0055, "LD D,L", 1),
        Opcode::new(0x0056, "LD D,(HL)", 1),
        Opcode::new(0x0057, "LD D,A", 1),
        Opcode::new(0x0058, "LD E,B", 1),
        Opcode::new(0x0059, "LD E,C", 1),
        Opcode::new(0x005A, "LD E,D", 1),
        Opcode::new(0x005B, "LD E,E", 1),
        Opcode::new(0x005C, "LD E,H", 1),
        Opcode::new(0x005D, "LD E,L", 1),
        Opcode::new(0x005E, "LD E,(HL)", 1),
        Opcode::new(0x005F, "LD E,A", 1),
        Opcode::new(0x0060, "LD H,B", 1),
        Opcode::new(0x0061, "LD H,C", 1),
        Opcode::new(0x0062, "LD H,D", 1),
        Opcode::new(0x0063, "LD H,E", 1),
        Opcode::new(0x0064, "LD H,H", 1),
        Opcode::new(0x0065, "LD H,L", 1),
        Opcode::new(0x0066, "LD H,(HL)", 1),
        Opcode::new(0x0067, "LD H,A", 1),
        Opcode::new(0x0068, "LD L,B", 1),
        Opcode::new(0x0069, "LD L,C", 1),
        Opcode::new(0x006A, "LD L,D", 1),
        Opcode::new(0x006B, "LD L,E", 1),
        Opcode::new(0x006C, "LD L,H", 1),
        Opcode::new(0x006D, "LD L,L", 1),
        Opcode::new(0x006E, "LD L,(HL)", 1),
        Opcode::new(0x006F, "LD L,A", 1),
        Opcode::new(0x0070, "LD (HL),B", 1),
        Opcode::new(0x0071, "LD (HL),C", 1),
        Opcode::new(0x0072, "LD (HL),D", 1),
        Opcode::new(0x0073, "LD (HL),E", 1),
        Opcode::new(0x0074, "LD (HL),H", 1),
        Opcode::new(0x0075, "LD (HL),L", 1),
        Opcode::new(0x0076, "HALT", 1),
        Opcode::new(0x0077, "LD (HL),A", 1),
        Opcode::new(0x0078, "LD A,B", 1),
        Opcode::new(0x0079, "LD A,C", 1),
        Opcode::new(0x007A, "LD A,D", 1),
        Opcode::new(0x007B, "LD A,E", 1),
        Opcode::new(0x007C, "LD A,H", 1),
        Opcode::new(0x007D, "LD A,L", 1),
        Opcode::new(0x007E, "LD A,(HL)", 1),
        Opcode::new(0x007F, "LD A,A", 1),
        Opcode::new(0x0080, "ADD A,B", 1),
        Opcode::new(0x0081, "ADD A,C", 1),
        Opcode::new(0x0082, "ADD A,D", 1),
        Opcode::new(0x0083, "ADD A,E", 1),
        Opcode::new(0x0084, "ADD A,H", 1),
        Opcode::new(0x0085, "ADD A,L", 1),
        Opcode::new(0x0086, "ADD A,(HL)", 1),
        Opcode::new(0x0087, "ADD A,A", 1),
        Opcode::new(0x0088, "ADC A,B", 1),
        Opcode::new(0x0089, "ADC A,C", 1),
        Opcode::new(0x008A, "ADC A,D", 1),
        Opcode::new(0x008B, "ADC A,E", 1),
        Opcode::new(0x008C, "ADC A,H", 1),
        Opcode::new(0x008D, "ADC A,L", 1),
        Opcode::new(0x008E, "ADC A,(HL)", 1),
        Opcode::new(0x008F, "ADC A,A", 1),
        Opcode::new(0x0090, "SUB A,B", 1),
        Opcode::new(0x0091, "SUB A,C", 1),
        Opcode::new(0x0092, "SUB A,D", 1),
        Opcode::new(0x0093, "SUB A,E", 1),
        Opcode::new(0x0094, "SUB A,H", 1),
        Opcode::new(0x0095, "SUB A,L", 1),
        Opcode::new(0x0096, "SUB A,(HL)", 1),
        Opcode::new(0x0097, "SUB A,A", 1),
        Opcode::new(0x0098, "SBC A,B", 1),
        Opcode::new(0x0099, "SBC A,C", 1),
        Opcode::new(0x009A, "SBC A,D", 1),
        Opcode::new(0x009B, "SBC A,E", 1),
        Opcode::new(0x009C, "SBC A,H", 1),
        Opcode::new(0x009D, "SBC A,L", 1),
        Opcode::new(0x009E, "SBC A,(HL)", 1),
        Opcode::new(0x009F, "SBC A,A", 1),
        Opcode::new(0x00A0, "AND A,B", 1),
        Opcode::new(0x00A1, "AND A,C", 1),
        Opcode::new(0x00A2, "AND A,D", 1),
        Opcode::new(0x00A3, "AND A,E", 1),
        Opcode::new(0x00A4, "AND A,H", 1),
        Opcode::new(0x00A5, "AND A,L", 1),
        Opcode::new(0x00A6, "AND A,(HL)", 1),
        Opcode::new(0x00A7, "AND A,A", 1),
        Opcode::new(0x00A8, "XOR A,B", 1),
        Opcode::new(0x00A9, "XOR A,C", 1),
        Opcode::new(0x00AA, "XOR A,D", 1),
        Opcode::new(0x00AB, "XOR A,E", 1),
        Opcode::new(0x00AC, "XOR A,H", 1),
        Opcode::new(0x00AD, "XOR A,L", 1),
        Opcode::new(0x00AE, "XOR A,(HL)", 1),
        Opcode::new(0x00AF, "XOR A,A", 1),
        Opcode::new(0x00B0, "OR A,B", 1),
        Opcode::new(0x00B1, "OR A,C", 1),
        Opcode::new(0x00B2, "OR A,D", 1),
        Opcode::new(0x00B3, "OR A,E", 1),
        Opcode::new(0x00B4, "OR A,H", 1),
        Opcode::new(0x00B5, "OR A,L", 1),
        Opcode::new(0x00B6, "OR A,(HL)", 1),
        Opcode::new(0x00B7, "OR A,A", 1),
        Opcode::new(0x00B8, "CP A,B", 1),
        Opcode::new(0x00B9, "CP A,C", 1),
        Opcode::new(0x00BA, "CP A,D", 1),
        Opcode::new(0x00BB, "CP A,E", 1),
        Opcode::new(0x00BC, "CP A,H", 1),
        Opcode::new(0x00BD, "CP A,L", 1),
        Opcode::new(0x00BE, "CP A,(HL)", 1),
        Opcode::new(0x00BF, "CP A,A", 1),
        Opcode::new(0x00C0, "RET NZ", 1),
        Opcode::new(0x00C1, "POP BC", 1),
        Opcode::new(0x00C2, "JP NZ,u16", 3),
        Opcode::new(0x00C3, "JP u16", 3),
        Opcode::new(0x00C4, "CALL NZ,u16", 3),
        Opcode::new(0x00C5, "PUSH BC", 1),
        Opcode::new(0x00C6, "ADD A,u8", 2),
        Opcode::new(0x00C7, "RST 00h", 1),
        Opcode::new(0x00C8, "RET Z", 1),
        Opcode::new(0x00C9, "RET", 1),
        Opcode::new(0x00CA, "JP Z,u16", 3),
        Opcode::new(0x00CB, "PREFIX CB", 1),
        Opcode::new(0x00CC, "CALL Z,u16", 3),
        Opcode::new(0x00CD, "CALL u16", 3),
        Opcode::new(0x00CE, "ADC A,u8", 2),
        Opcode::new(0x00CF, "RST 08h", 1),
        Opcode::new(0x00D0, "RET NC", 1),
        Opcode::new(0x00D1, "POP DE", 1),
        Opcode::new(0x00D2, "JP NC,u16", 3),
        Opcode::new(0x00D4, "CALL NC,u16", 3),
        Opcode::new(0x00D5, "PUSH DE", 1),
        Opcode::new(0x00D6, "SUB A,u8", 2),
        Opcode::new(0x00D7, "RST 10h", 1),
        Opcode::new(0x00D8, "RET C", 1),
        Opcode::new(0x00D9, "RETI", 1),
        Opcode::new(0x00DA, "JP C,u16", 3),
        Opcode::new(0x00DC, "CALL C,u16", 3),
        Opcode::new(0x00DE, "SBC A,u8", 2),
        Opcode::new(0x00DF, "RST 18h", 1),
        Opcode::new(0x00E0, "LD (FF00+u8),A", 2),
        Opcode::new(0x00E1, "POP HL", 1),
        Opcode::new(0x00E2, "LD (FF00+C),A", 1),
        Opcode::new(0x00E5, "PUSH HL", 1),
        Opcode::new(0x00E6, "AND A,u8", 2),
        Opcode::new(0x00E7, "RST 20h", 1),
        Opcode::new(0x00E8, "ADD SP,i8", 2),
        Opcode::new(0x00E9, "JP HL", 1),
        Opcode::new(0x00EA, "LD (u16),A", 3),
        Opcode::new(0x00EE, "XOR A,u8", 2),
        Opcode::new(0x00EF, "RST 28h", 1),
        Opcode::new(0x00F0, "LD A,(FF00+u8)", 2),
        Opcode::new(0x00F1, "POP AF", 1),
        Opcode::new(0x00F2, "LD A,(FF00+C)", 1),
        Opcode::new(0x00F3, "DI", 1),
        Opcode::new(0x00F5, "PUSH AF", 1),
        Opcode::new(0x00F6, "OR A,u8", 2),
        Opcode::new(0x00F7, "RST 30h", 1),
        Opcode::new(0x00F8, "LD HL,SP+i8", 2),
        Opcode::new(0x00F9, "LD SP,HL", 1),
        Opcode::new(0x00FA, "LD A,(u16)", 3),
        Opcode::new(0x00FB, "EI", 1),
        Opcode::new(0x00FE, "CP A,u8", 2),
        Opcode::new(0x00FF, "RST 38h", 1),
        Opcode::new(0xCB00, "RLC B", 2),
        Opcode::new(0xCB01, "RLC C", 2),
        Opcode::new(0xCB02, "RLC D", 2),
        Opcode::new(0xCB03, "RLC E", 2),
        Opcode::new(0xCB04, "RLC H", 2),
        Opcode::new(0xCB05, "RLC L", 2),
        Opcode::new(0xCB06, "RLC (HL)", 2),
        Opcode::new(0xCB07, "RLC A", 2),
        Opcode::new(0xCB08, "RRC B", 2),
        Opcode::new(0xCB09, "RRC C", 2),
        Opcode::new(0xCB0A, "RRC D", 2),
        Opcode::new(0xCB0B, "RRC E", 2),
        Opcode::new(0xCB0C, "RRC H", 2),
        Opcode::new(0xCB0D, "RRC L", 2),
        Opcode::new(0xCB0E, "RRC (HL)", 2),
        Opcode::new(0xCB0F, "RRC A", 2),
        Opcode::new(0xCB10, "RL B", 2),
        Opcode::new(0xCB11, "RL C", 2),
        Opcode::new(0xCB12, "RL D", 2),
        Opcode::new(0xCB13, "RL E", 2),
        Opcode::new(0xCB14, "RL H", 2),
        Opcode::new(0xCB15, "RL L", 2),
        Opcode::new(0xCB16, "RL (HL)", 2),
        Opcode::new(0xCB17, "RL A", 2),
        Opcode::new(0xCB18, "RR B", 2),
        Opcode::new(0xCB19, "RR C", 2),
        Opcode::new(0xCB1A, "RR D", 2),
        Opcode::new(0xCB1B, "RR E", 2),
        Opcode::new(0xCB1C, "RR H", 2),
        Opcode::new(0xCB1D, "RR L", 2),
        Opcode::new(0xCB1E, "RR (HL)", 2),
        Opcode::new(0xCB1F, "RR A", 2),
        Opcode::new(0xCB20, "SLA B", 2),
        Opcode::new(0xCB21, "SLA C", 2),
        Opcode::new(0xCB22, "SLA D", 2),
        Opcode::new(0xCB23, "SLA E", 2),
        Opcode::new(0xCB24, "SLA H", 2),
        Opcode::new(0xCB25, "SLA L", 2),
        Opcode::new(0xCB26, "SLA (HL)", 2),
        Opcode::new(0xCB27, "SLA A", 2),
        Opcode::new(0xCB28, "SRA B", 2),
        Opcode::new(0xCB29, "SRA C", 2),
        Opcode::new(0xCB2A, "SRA D", 2),
        Opcode::new(0xCB2B, "SRA E", 2),
        Opcode::new(0xCB2C, "SRA H", 2),
        Opcode::new(0xCB2D, "SRA L", 2),
        Opcode::new(0xCB2E, "SRA (HL)", 2),
        Opcode::new(0xCB2F, "SRA A", 2),
        Opcode::new(0xCB30, "SWAP B", 2),
        Opcode::new(0xCB31, "SWAP C", 2),
        Opcode::new(0xCB32, "SWAP D", 2),
        Opcode::new(0xCB33, "SWAP E", 2),
        Opcode::new(0xCB34, "SWAP H", 2),
        Opcode::new(0xCB35, "SWAP L", 2),
        Opcode::new(0xCB36, "SWAP (HL)", 2),
        Opcode::new(0xCB37, "SWAP A", 2),
        Opcode::new(0xCB38, "SRL B", 2),
        Opcode::new(0xCB39, "SRL C", 2),
        Opcode::new(0xCB3A, "SRL D", 2),
        Opcode::new(0xCB3B, "SRL E", 2),
        Opcode::new(0xCB3C, "SRL H", 2),
        Opcode::new(0xCB3D, "SRL L", 2),
        Opcode::new(0xCB3E, "SRL (HL)", 2),
        Opcode::new(0xCB3F, "SRL A", 2),
        Opcode::new(0xCB40, "BIT 0,B", 2),
        Opcode::new(0xCB41, "BIT 0,C", 2),
        Opcode::new(0xCB42, "BIT 0,D", 2),
        Opcode::new(0xCB43, "BIT 0,E", 2),
        Opcode::new(0xCB44, "BIT 0,H", 2),
        Opcode::new(0xCB45, "BIT 0,L", 2),
        Opcode::new(0xCB46, "BIT 0,(HL)", 2),
        Opcode::new(0xCB47, "BIT 0,A", 2),
        Opcode::new(0xCB48, "BIT 1,B", 2),
        Opcode::new(0xCB49, "BIT 1,C", 2),
        Opcode::new(0xCB4A, "BIT 1,D", 2),
        Opcode::new(0xCB4B, "BIT 1,E", 2),
        Opcode::new(0xCB4C, "BIT 1,H", 2),
        Opcode::new(0xCB4D, "BIT 1,L", 2),
        Opcode::new(0xCB4E, "BIT 1,(HL)", 2),
        Opcode::new(0xCB4F, "BIT 1,A", 2),
        Opcode::new(0xCB50, "BIT 2,B", 2),
        Opcode::new(0xCB51, "BIT 2,C", 2),
        Opcode::new(0xCB52, "BIT 2,D", 2),
        Opcode::new(0xCB53, "BIT 2,E", 2),
        Opcode::new(0xCB54, "BIT 2,H", 2),
        Opcode::new(0xCB55, "BIT 2,L", 2),
        Opcode::new(0xCB56, "BIT 2,(HL)", 2),
        Opcode::new(0xCB57, "BIT 2,A", 2),
        Opcode::new(0xCB58, "BIT 3,B", 2),
        Opcode::new(0xCB59, "BIT 3,C", 2),
        Opcode::new(0xCB5A, "BIT 3,D", 2),
        Opcode::new(0xCB5B, "BIT 3,E", 2),
        Opcode::new(0xCB5C, "BIT 3,H", 2),
        Opcode::new(0xCB5D, "BIT 3,L", 2),
        Opcode::new(0xCB5E, "BIT 3,(HL)", 2),
        Opcode::new(0xCB5F, "BIT 3,A", 2),
        Opcode::new(0xCB60, "BIT 4,B", 2),
        Opcode::new(0xCB61, "BIT 4,C", 2),
        Opcode::new(0xCB62, "BIT 4,D", 2),
        Opcode::new(0xCB63, "BIT 4,E", 2),
        Opcode::new(0xCB64, "BIT 4,H", 2),
        Opcode::new(0xCB65, "BIT 4,L", 2),
        Opcode::new(0xCB66, "BIT 4,(HL)", 2),
        Opcode::new(0xCB67, "BIT 4,A", 2),
        Opcode::new(0xCB68, "BIT 5,B", 2),
        Opcode::new(0xCB69, "BIT 5,C", 2),
        Opcode::new(0xCB6A, "BIT 5,D", 2),
        Opcode::new(0xCB6B, "BIT 5,E", 2),
        Opcode::new(0xCB6C, "BIT 5,H", 2),
        Opcode::new(0xCB6D, "BIT 5,L", 2),
        Opcode::new(0xCB6E, "BIT 5,(HL)", 2),
        Opcode::new(0xCB6F, "BIT 5,A", 2),
        Opcode::new(0xCB70, "BIT 6,B", 2),
        Opcode::new(0xCB71, "BIT 6,C", 2),
        Opcode::new(0xCB72, "BIT 6,D", 2),
        Opcode::new(0xCB73, "BIT 6,E", 2),
        Opcode::new(0xCB74, "BIT 6,H", 2),
        Opcode::new(0xCB75, "BIT 6,L", 2),
        Opcode::new(0xCB76, "BIT 6,(HL)", 2),
        Opcode::new(0xCB77, "BIT 6,A", 2),
        Opcode::new(0xCB78, "BIT 7,B", 2),
        Opcode::new(0xCB79, "BIT 7,C", 2),
        Opcode::new(0xCB7A, "BIT 7,D", 2),
        Opcode::new(0xCB7B, "BIT 7,E", 2),
        Opcode::new(0xCB7C, "BIT 7,H", 2),
        Opcode::new(0xCB7D, "BIT 7,L", 2),
        Opcode::new(0xCB7E, "BIT 7,(HL)", 2),
        Opcode::new(0xCB7F, "BIT 7,A", 2),
        Opcode::new(0xCB80, "RES 0,B", 2),
        Opcode::new(0xCB81, "RES 0,C", 2),
        Opcode::new(0xCB82, "RES 0,D", 2),
        Opcode::new(0xCB83, "RES 0,E", 2),
        Opcode::new(0xCB84, "RES 0,H", 2),
        Opcode::new(0xCB85, "RES 0,L", 2),
        Opcode::new(0xCB86, "RES 0,(HL)", 2),
        Opcode::new(0xCB87, "RES 0,A", 2),
        Opcode::new(0xCB88, "RES 1,B", 2),
        Opcode::new(0xCB89, "RES 1,C", 2),
        Opcode::new(0xCB8A, "RES 1,D", 2),
        Opcode::new(0xCB8B, "RES 1,E", 2),
        Opcode::new(0xCB8C, "RES 1,H", 2),
        Opcode::new(0xCB8D, "RES 1,L", 2),
        Opcode::new(0xCB8E, "RES 1,(HL)", 2),
        Opcode::new(0xCB8F, "RES 1,A", 2),
        Opcode::new(0xCB90, "RES 2,B", 2),
        Opcode::new(0xCB91, "RES 2,C", 2),
        Opcode::new(0xCB92, "RES 2,D", 2),
        Opcode::new(0xCB93, "RES 2,E", 2),
        Opcode::new(0xCB94, "RES 2,H", 2),
        Opcode::new(0xCB95, "RES 2,L", 2),
        Opcode::new(0xCB96, "RES 2,(HL)", 2),
        Opcode::new(0xCB97, "RES 2,A", 2),
        Opcode::new(0xCB98, "RES 3,B", 2),
        Opcode::new(0xCB99, "RES 3,C", 2),
        Opcode::new(0xCB9A, "RES 3,D", 2),
        Opcode::new(0xCB9B, "RES 3,E", 2),
        Opcode::new(0xCB9C, "RES 3,H", 2),
        Opcode::new(0xCB9D, "RES 3,L", 2),
        Opcode::new(0xCB9E, "RES 3,(HL)", 2),
        Opcode::new(0xCB9F, "RES 3,A", 2),
        Opcode::new(0xCBA0, "RES 4,B", 2),
        Opcode::new(0xCBA1, "RES 4,C", 2),
        Opcode::new(0xCBA2, "RES 4,D", 2),
        Opcode::new(0xCBA3, "RES 4,E", 2),
        Opcode::new(0xCBA4, "RES 4,H", 2),
        Opcode::new(0xCBA5, "RES 4,L", 2),
        Opcode::new(0xCBA6, "RES 4,(HL)", 2),
        Opcode::new(0xCBA7, "RES 4,A", 2),
        Opcode::new(0xCBA8, "RES 5,B", 2),
        Opcode::new(0xCBA9, "RES 5,C", 2),
        Opcode::new(0xCBAA, "RES 5,D", 2),
        Opcode::new(0xCBAB, "RES 5,E", 2),
        Opcode::new(0xCBAC, "RES 5,H", 2),
        Opcode::new(0xCBAD, "RES 5,L", 2),
        Opcode::new(0xCBAE, "RES 5,(HL)", 2),
        Opcode::new(0xCBAF, "RES 5,A", 2),
        Opcode::new(0xCBB0, "RES 6,B", 2),
        Opcode::new(0xCBB1, "RES 6,C", 2),
        Opcode::new(0xCBB2, "RES 6,D", 2),
        Opcode::new(0xCBB3, "RES 6,E", 2),
        Opcode::new(0xCBB4, "RES 6,H", 2),
        Opcode::new(0xCBB5, "RES 6,L", 2),
        Opcode::new(0xCBB6, "RES 6,(HL)", 2),
        Opcode::new(0xCBB7, "RES 6,A", 2),
        Opcode::new(0xCBB8, "RES 7,B", 2),
        Opcode::new(0xCBB9, "RES 7,C", 2),
        Opcode::new(0xCBBA, "RES 7,D", 2),
        Opcode::new(0xCBBB, "RES 7,E", 2),
        Opcode::new(0xCBBC, "RES 7,H", 2),
        Opcode::new(0xCBBD, "RES 7,L", 2),
        Opcode::new(0xCBBE, "RES 7,(HL)", 2),
        Opcode::new(0xCBBF, "RES 7,A", 2),
        Opcode::new(0xCBC0, "SET 0,B", 2),
        Opcode::new(0xCBC1, "SET 0,C", 2),
        Opcode::new(0xCBC2, "SET 0,D", 2),
        Opcode::new(0xCBC3, "SET 0,E", 2),
        Opcode::new(0xCBC4, "SET 0,H", 2),
        Opcode::new(0xCBC5, "SET 0,L", 2),
        Opcode::new(0xCBC6, "SET 0,(HL)", 2),
        Opcode::new(0xCBC7, "SET 0,A", 2),
        Opcode::new(0xCBC8, "SET 1,B", 2),
        Opcode::new(0xCBC9, "SET 1,C", 2),
        Opcode::new(0xCBCA, "SET 1,D", 2),
        Opcode::new(0xCBCB, "SET 1,E", 2),
        Opcode::new(0xCBCC, "SET 1,H", 2),
        Opcode::new(0xCBCD, "SET 1,L", 2),
        Opcode::new(0xCBCE, "SET 1,(HL)", 2),
        Opcode::new(0xCBCF, "SET 1,A", 2),
        Opcode::new(0xCBD0, "SET 2,B", 2),
        Opcode::new(0xCBD1, "SET 2,C", 2),
        Opcode::new(0xCBD2, "SET 2,D", 2),
        Opcode::new(0xCBD3, "SET 2,E", 2),
        Opcode::new(0xCBD4, "SET 2,H", 2),
        Opcode::new(0xCBD5, "SET 2,L", 2),
        Opcode::new(0xCBD6, "SET 2,(HL)", 2),
        Opcode::new(0xCBD7, "SET 2,A", 2),
        Opcode::new(0xCBD8, "SET 3,B", 2),
        Opcode::new(0xCBD9, "SET 3,C", 2),
        Opcode::new(0xCBDA, "SET 3,D", 2),
        Opcode::new(0xCBDB, "SET 3,E", 2),
        Opcode::new(0xCBDC, "SET 3,H", 2),
        Opcode::new(0xCBDD, "SET 3,L", 2),
        Opcode::new(0xCBDE, "SET 3,(HL)", 2),
        Opcode::new(0xCBDF, "SET 3,A", 2),
        Opcode::new(0xCBE0, "SET 4,B", 2),
        Opcode::new(0xCBE1, "SET 4,C", 2),
        Opcode::new(0xCBE2, "SET 4,D", 2),
        Opcode::new(0xCBE3, "SET 4,E", 2),
        Opcode::new(0xCBE4, "SET 4,H", 2),
        Opcode::new(0xCBE5, "SET 4,L", 2),
        Opcode::new(0xCBE6, "SET 4,(HL)", 2),
        Opcode::new(0xCBE7, "SET 4,A", 2),
        Opcode::new(0xCBE8, "SET 5,B", 2),
        Opcode::new(0xCBE9, "SET 5,C", 2),
        Opcode::new(0xCBEA, "SET 5,D", 2),
        Opcode::new(0xCBEB, "SET 5,E", 2),
        Opcode::new(0xCBEC, "SET 5,H", 2),
        Opcode::new(0xCBED, "SET 5,L", 2),
        Opcode::new(0xCBEE, "SET 5,(HL)", 2),
        Opcode::new(0xCBEF, "SET 5,A", 2),
        Opcode::new(0xCBF0, "SET 6,B", 2),
        Opcode::new(0xCBF1, "SET 6,C", 2),
        Opcode::new(0xCBF2, "SET 6,D", 2),
        Opcode::new(0xCBF3, "SET 6,E", 2),
        Opcode::new(0xCBF4, "SET 6,H", 2),
        Opcode::new(0xCBF5, "SET 6,L", 2),
        Opcode::new(0xCBF6, "SET 6,(HL)", 2),
        Opcode::new(0xCBF7, "SET 6,A", 2),
        Opcode::new(0xCBF8, "SET 7,B", 2),
        Opcode::new(0xCBF9, "SET 7,C", 2),
        Opcode::new(0xCBFA, "SET 7,D", 2),
        Opcode::new(0xCBFB, "SET 7,E", 2),
        Opcode::new(0xCBFC, "SET 7,H", 2),
        Opcode::new(0xCBFD, "SET 7,L", 2),
        Opcode::new(0xCBFE, "SET 7,(HL)", 2),
        Opcode::new(0xCBFF, "SET 7,A", 2),
    ];
    pub static ref CPU_OPCODES: HashMap<u16, &'static Opcode> = {
        let mut map = HashMap::new();
        for opcode in OPCODES_LIST.iter() {
            map.insert(opcode.code, opcode);
        }
        map
    };
}

impl CPU {
    /// NOP
    #[allow(unused_variables)]
    fn op_0000(&mut self, op_size: u8) -> u8 {
        4
    }

    /// LD BC,u16
    #[allow(unused_variables)]
    fn op_0001(&mut self, op_size: u8) -> u8 {
        let res = self.mem_read_u16(self.program_counter);
        self.set_bc(res);

        12
    }

    /// LD (BC),A
    #[allow(unused_variables)]
    fn op_0002(&mut self, op_size: u8) -> u8 {
        let res = self.get_a();
        self.mem_write_u8(self.get_bc(), res);

        8
    }

    /// INC BC
    #[allow(unused_variables)]
    fn op_0003(&mut self, op_size: u8) -> u8 {
        let (res, z, h, _) = alu::add_u16(self.get_bc(), 1, false);
        self.set_bc(res);

        8
    }

    /// INC B
    #[allow(unused_variables)]
    fn op_0004(&mut self, op_size: u8) -> u8 {
        let (res, z, h, _) = alu::add_u8(self.get_b(), 1, false);
        self.set_b(res);

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.set(StatusFlags::H, h);

        4
    }

    /// DEC B
    #[allow(unused_variables)]
    fn op_0005(&mut self, op_size: u8) -> u8 {
        let (res, z, h, _) = alu::sub_u8(self.get_b(), 1, false);
        self.set_b(res);

        self.status.set(StatusFlags::Z, z);
        self.status.insert(StatusFlags::N);
        self.status.set(StatusFlags::H, h);

        4
    }

    /// LD B,u8
    #[allow(unused_variables)]
    fn op_0006(&mut self, op_size: u8) -> u8 {
        let res = self.mem_read_u8(self.program_counter);
        self.set_b(res);

        8
    }

    /// RLCA
    #[allow(unused_variables)]
    fn op_0007(&mut self, op_size: u8) -> u8 {
        let c = self.a & 0x80 != 0;
        self.a = self.a.rotate_left(1);

        self.status.remove(StatusFlags::Z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// LD (u16),SP
    #[allow(unused_variables)]
    fn op_0008(&mut self, op_size: u8) -> u8 {
        let res = self.get_sp();
        self.mem_write_u16(self.mem_read_u16(self.program_counter), res);

        20
    }

    /// ADD HL,BC
    #[allow(unused_variables)]
    fn op_0009(&mut self, op_size: u8) -> u8 {
        let x = self.get_hl();
        let y = self.get_bc();
        let (res, z, h, c) = alu::add_u16(x, y, false);
        self.set_hl(res);

        self.status.remove(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// LD A,(BC)
    #[allow(unused_variables)]
    fn op_000a(&mut self, op_size: u8) -> u8 {
        let res = self.mem_read_u8(self.get_bc());
        self.set_a(res);

        8
    }

    /// DEC BC
    #[allow(unused_variables)]
    fn op_000b(&mut self, op_size: u8) -> u8 {
        let res = self.get_bc().wrapping_sub(1);
        self.set_bc(res);

        8
    }

    /// INC C
    #[allow(unused_variables)]
    fn op_000c(&mut self, op_size: u8) -> u8 {
        let (res, z, h, _) = alu::add_u8(self.get_c(), 1, false);
        self.set_c(res);

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.set(StatusFlags::H, h);

        4
    }

    /// DEC C
    #[allow(unused_variables)]
    fn op_000d(&mut self, op_size: u8) -> u8 {
        let (res, z, h, _) = alu::sub_u8(self.get_c(), 1, false);
        self.set_c(res);

        self.status.set(StatusFlags::Z, z);
        self.status.insert(StatusFlags::N);
        self.status.set(StatusFlags::H, h);

        4
    }

    /// LD C,u8
    #[allow(unused_variables)]
    fn op_000e(&mut self, op_size: u8) -> u8 {
        let res = self.mem_read_u8(self.program_counter);
        self.set_c(res);

        8
    }

    /// RRCA
    #[allow(unused_variables)]
    fn op_000f(&mut self, op_size: u8) -> u8 {
        let c = self.a & 0x01 != 0;
        self.a = self.a.rotate_right(1);

        self.status.remove(StatusFlags::Z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// STOP
    #[allow(unused_variables)]
    fn op_0010(&mut self, op_size: u8) -> u8 {
        self.stop();

        4
    }

    /// LD DE,u16
    #[allow(unused_variables)]
    fn op_0011(&mut self, op_size: u8) -> u8 {
        let res = self.mem_read_u16(self.program_counter);
        self.set_de(res);

        12
    }

    /// LD (DE),A
    #[allow(unused_variables)]
    fn op_0012(&mut self, op_size: u8) -> u8 {
        let res = self.get_a();
        self.mem_write_u8(self.get_de(), res);

        8
    }

    /// INC DE
    #[allow(unused_variables)]
    fn op_0013(&mut self, op_size: u8) -> u8 {
        let (res, z, h, _) = alu::add_u16(self.get_de(), 1, false);
        self.set_de(res);

        8
    }

    /// INC D
    #[allow(unused_variables)]
    fn op_0014(&mut self, op_size: u8) -> u8 {
        let (res, z, h, _) = alu::add_u8(self.get_d(), 1, false);
        self.set_d(res);

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.set(StatusFlags::H, h);

        4
    }

    /// DEC D
    #[allow(unused_variables)]
    fn op_0015(&mut self, op_size: u8) -> u8 {
        let (res, z, h, _) = alu::sub_u8(self.get_d(), 1, false);
        self.set_d(res);

        self.status.set(StatusFlags::Z, z);
        self.status.insert(StatusFlags::N);
        self.status.set(StatusFlags::H, h);

        4
    }

    /// LD D,u8
    #[allow(unused_variables)]
    fn op_0016(&mut self, op_size: u8) -> u8 {
        let res = self.mem_read_u8(self.program_counter);
        self.set_d(res);

        8
    }

    /// RLA
    #[allow(unused_variables)]
    fn op_0017(&mut self, op_size: u8) -> u8 {
        let c = self.a & 0x80 != 0;
        self.a = self.a.wrapping_shl(1);
        self.a |= if self.get_cf() { 1 } else { 0 };

        self.status.remove(StatusFlags::Z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// JR i8
    #[allow(unused_variables)]
    fn op_0018(&mut self, op_size: u8) -> u8 {
        self.cpu_jr();

        12
    }

    /// ADD HL,DE
    #[allow(unused_variables)]
    fn op_0019(&mut self, op_size: u8) -> u8 {
        let x = self.get_hl();
        let y = self.get_de();
        let (res, z, h, c) = alu::add_u16(x, y, false);
        self.set_hl(res);

        self.status.remove(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// LD A,(DE)
    #[allow(unused_variables)]
    fn op_001a(&mut self, op_size: u8) -> u8 {
        let res = self.mem_read_u8(self.get_de());
        self.set_a(res);

        8
    }

    /// DEC DE
    #[allow(unused_variables)]
    fn op_001b(&mut self, op_size: u8) -> u8 {
        let res = self.get_de().wrapping_sub(1);
        self.set_de(res);

        8
    }

    /// INC E
    #[allow(unused_variables)]
    fn op_001c(&mut self, op_size: u8) -> u8 {
        let (res, z, h, _) = alu::add_u8(self.get_e(), 1, false);
        self.set_e(res);

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.set(StatusFlags::H, h);

        4
    }

    /// DEC E
    #[allow(unused_variables)]
    fn op_001d(&mut self, op_size: u8) -> u8 {
        let (res, z, h, _) = alu::sub_u8(self.get_e(), 1, false);
        self.set_e(res);

        self.status.set(StatusFlags::Z, z);
        self.status.insert(StatusFlags::N);
        self.status.set(StatusFlags::H, h);

        4
    }

    /// LD E,u8
    #[allow(unused_variables)]
    fn op_001e(&mut self, op_size: u8) -> u8 {
        let res = self.mem_read_u8(self.program_counter);
        self.set_e(res);

        8
    }

    /// RRA
    #[allow(unused_variables)]
    fn op_001f(&mut self, op_size: u8) -> u8 {
        let c = self.a & 0x01 != 0;
        self.a = self.a.wrapping_shr(1);
        self.a |= if self.get_cf() { 0x80 } else { 0 };

        self.status.remove(StatusFlags::Z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// JR NZ,i8
    #[allow(unused_variables)]
    fn op_0020(&mut self, op_size: u8) -> u8 {
        let flg = !self.status.contains(StatusFlags::Z);
        if flg {
            self.cpu_jr();
            return 8;
        }

        12
    }

    /// LD HL,u16
    #[allow(unused_variables)]
    fn op_0021(&mut self, op_size: u8) -> u8 {
        let res = self.mem_read_u16(self.program_counter);
        self.set_hl(res);

        12
    }

    /// LD (HL+),A
    #[allow(unused_variables)]
    fn op_0022(&mut self, op_size: u8) -> u8 {
        let res = self.get_a();
        self.mem_write_u8(self.get_hl(), res);
        self.set_hl(self.get_hl().wrapping_add(1));

        8
    }

    /// INC HL
    #[allow(unused_variables)]
    fn op_0023(&mut self, op_size: u8) -> u8 {
        let (res, z, h, _) = alu::add_u16(self.get_hl(), 1, false);
        self.set_hl(res);

        8
    }

    /// INC H
    #[allow(unused_variables)]
    fn op_0024(&mut self, op_size: u8) -> u8 {
        let (res, z, h, _) = alu::add_u8(self.get_h(), 1, false);
        self.set_h(res);

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.set(StatusFlags::H, h);

        4
    }

    /// DEC H
    #[allow(unused_variables)]
    fn op_0025(&mut self, op_size: u8) -> u8 {
        let (res, z, h, _) = alu::sub_u8(self.get_h(), 1, false);
        self.set_h(res);

        self.status.set(StatusFlags::Z, z);
        self.status.insert(StatusFlags::N);
        self.status.set(StatusFlags::H, h);

        4
    }

    /// LD H,u8
    #[allow(unused_variables)]
    fn op_0026(&mut self, op_size: u8) -> u8 {
        let res = self.mem_read_u8(self.program_counter);
        self.set_h(res);

        8
    }

    /// DAA
    #[allow(unused_variables)]
    fn op_0027(&mut self, op_size: u8) -> u8 {
        let mut adj = 0;

        let v = self.a as usize;

        if self.get_hf() || (!self.get_nf() && (v & 0x0F > 0x09)) {
            adj |= 0x06;
        }

        let c = if self.get_cf() || (!self.get_nf() && v > 0x99) {
            adj |= 0x60;
            true
        } else {
            false
        };

        let res = if self.get_nf() { v - adj } else { v + adj };
        let res = (res & 0xFF) as u8;
        let z = res == 0;

        self.set_a(res);

        self.status.set(StatusFlags::Z, z);

        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// JR Z,i8
    #[allow(unused_variables)]
    fn op_0028(&mut self, op_size: u8) -> u8 {
        let flg = self.status.contains(StatusFlags::Z);
        if flg {
            self.cpu_jr();
            return 8;
        }

        12
    }

    /// ADD HL,HL
    #[allow(unused_variables)]
    fn op_0029(&mut self, op_size: u8) -> u8 {
        let x = self.get_hl();
        let y = self.get_hl();
        let (res, z, h, c) = alu::add_u16(x, y, false);
        self.set_hl(res);

        self.status.remove(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// LD A,(HL+)
    #[allow(unused_variables)]
    fn op_002a(&mut self, op_size: u8) -> u8 {
        let res = self.mem_read_u8(self.get_hl());
        self.set_a(res);
        self.set_hl(self.get_hl().wrapping_add(1));

        8
    }

    /// DEC HL
    #[allow(unused_variables)]
    fn op_002b(&mut self, op_size: u8) -> u8 {
        let res = self.get_hl().wrapping_sub(1);
        self.set_hl(res);

        8
    }

    /// INC L
    #[allow(unused_variables)]
    fn op_002c(&mut self, op_size: u8) -> u8 {
        let (res, z, h, _) = alu::add_u8(self.get_l(), 1, false);
        self.set_l(res);

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.set(StatusFlags::H, h);

        4
    }

    /// DEC L
    #[allow(unused_variables)]
    fn op_002d(&mut self, op_size: u8) -> u8 {
        let (res, z, h, _) = alu::sub_u8(self.get_l(), 1, false);
        self.set_l(res);

        self.status.set(StatusFlags::Z, z);
        self.status.insert(StatusFlags::N);
        self.status.set(StatusFlags::H, h);

        4
    }

    /// LD L,u8
    #[allow(unused_variables)]
    fn op_002e(&mut self, op_size: u8) -> u8 {
        let res = self.mem_read_u8(self.program_counter);
        self.set_l(res);

        8
    }

    /// CPL
    #[allow(unused_variables)]
    fn op_002f(&mut self, op_size: u8) -> u8 {
        self.a = !self.a;

        self.status.insert(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        4
    }

    /// JR NC,i8
    #[allow(unused_variables)]
    fn op_0030(&mut self, op_size: u8) -> u8 {
        let flg = !self.status.contains(StatusFlags::C);
        if flg {
            self.cpu_jr();
            return 8;
        }

        12
    }

    /// LD SP,u16
    #[allow(unused_variables)]
    fn op_0031(&mut self, op_size: u8) -> u8 {
        let res = self.mem_read_u16(self.program_counter);
        self.set_sp(res);

        12
    }

    /// LD (HL-),A
    #[allow(unused_variables)]
    fn op_0032(&mut self, op_size: u8) -> u8 {
        let res = self.get_a();
        self.mem_write_u8(self.get_hl(), res);
        self.set_hl(self.get_hl().wrapping_sub(1));

        8
    }

    /// INC SP
    #[allow(unused_variables)]
    fn op_0033(&mut self, op_size: u8) -> u8 {
        let (res, z, h, _) = alu::add_u16(self.get_sp(), 1, false);
        self.set_sp(res);

        8
    }

    /// INC (HL)
    #[allow(unused_variables)]
    fn op_0034(&mut self, op_size: u8) -> u8 {
        let (res, z, h, _) = alu::add_u8(self.mem_read_u8(self.get_hl()), 1, false);
        self.mem_write_u8(self.get_hl(), res);

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.set(StatusFlags::H, h);

        12
    }

    /// DEC (HL)
    #[allow(unused_variables)]
    fn op_0035(&mut self, op_size: u8) -> u8 {
        let (res, z, h, _) = alu::sub_u8(self.mem_read_u8(self.get_hl()), 1, false);
        self.mem_write_u8(self.get_hl(), res);

        self.status.set(StatusFlags::Z, z);
        self.status.insert(StatusFlags::N);
        self.status.set(StatusFlags::H, h);

        12
    }

    /// LD (HL),u8
    #[allow(unused_variables)]
    fn op_0036(&mut self, op_size: u8) -> u8 {
        let res = self.mem_read_u8(self.program_counter);
        self.mem_write_u8(self.get_hl(), res);

        12
    }

    /// SCF
    #[allow(unused_variables)]
    fn op_0037(&mut self, op_size: u8) -> u8 {
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.insert(StatusFlags::C);

        4
    }

    /// JR C,i8
    #[allow(unused_variables)]
    fn op_0038(&mut self, op_size: u8) -> u8 {
        let flg = self.status.contains(StatusFlags::C);
        if flg {
            self.cpu_jr();
            return 8;
        }

        12
    }

    /// ADD HL,SP
    #[allow(unused_variables)]
    fn op_0039(&mut self, op_size: u8) -> u8 {
        let x = self.get_hl();
        let y = self.get_sp();
        let (res, z, h, c) = alu::add_u16(x, y, false);
        self.set_hl(res);

        self.status.remove(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// LD A,(HL-)
    #[allow(unused_variables)]
    fn op_003a(&mut self, op_size: u8) -> u8 {
        let res = self.mem_read_u8(self.get_hl());
        self.set_a(res);
        self.set_hl(self.get_hl().wrapping_sub(1));

        8
    }

    /// DEC SP
    #[allow(unused_variables)]
    fn op_003b(&mut self, op_size: u8) -> u8 {
        let res = self.get_sp().wrapping_sub(1);
        self.set_sp(res);

        8
    }

    /// INC A
    #[allow(unused_variables)]
    fn op_003c(&mut self, op_size: u8) -> u8 {
        let (res, z, h, _) = alu::add_u8(self.get_a(), 1, false);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.set(StatusFlags::H, h);

        4
    }

    /// DEC A
    #[allow(unused_variables)]
    fn op_003d(&mut self, op_size: u8) -> u8 {
        let (res, z, h, _) = alu::sub_u8(self.get_a(), 1, false);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.insert(StatusFlags::N);
        self.status.set(StatusFlags::H, h);

        4
    }

    /// LD A,u8
    #[allow(unused_variables)]
    fn op_003e(&mut self, op_size: u8) -> u8 {
        let res = self.mem_read_u8(self.program_counter);
        self.set_a(res);

        8
    }

    /// CCF
    #[allow(unused_variables)]
    fn op_003f(&mut self, op_size: u8) -> u8 {
        let c = !self.get_cf();

        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// LD B,B
    #[allow(unused_variables)]
    fn op_0040(&mut self, op_size: u8) -> u8 {
        let res = self.get_b();
        self.set_b(res);

        4
    }

    /// LD B,C
    #[allow(unused_variables)]
    fn op_0041(&mut self, op_size: u8) -> u8 {
        let res = self.get_c();
        self.set_b(res);

        4
    }

    /// LD B,D
    #[allow(unused_variables)]
    fn op_0042(&mut self, op_size: u8) -> u8 {
        let res = self.get_d();
        self.set_b(res);

        4
    }

    /// LD B,E
    #[allow(unused_variables)]
    fn op_0043(&mut self, op_size: u8) -> u8 {
        let res = self.get_e();
        self.set_b(res);

        4
    }

    /// LD B,H
    #[allow(unused_variables)]
    fn op_0044(&mut self, op_size: u8) -> u8 {
        let res = self.get_h();
        self.set_b(res);

        4
    }

    /// LD B,L
    #[allow(unused_variables)]
    fn op_0045(&mut self, op_size: u8) -> u8 {
        let res = self.get_l();
        self.set_b(res);

        4
    }

    /// LD B,(HL)
    #[allow(unused_variables)]
    fn op_0046(&mut self, op_size: u8) -> u8 {
        let res = self.mem_read_u8(self.get_hl());
        self.set_b(res);

        8
    }

    /// LD B,A
    #[allow(unused_variables)]
    fn op_0047(&mut self, op_size: u8) -> u8 {
        let res = self.get_a();
        self.set_b(res);

        4
    }

    /// LD C,B
    #[allow(unused_variables)]
    fn op_0048(&mut self, op_size: u8) -> u8 {
        let res = self.get_b();
        self.set_c(res);

        4
    }

    /// LD C,C
    #[allow(unused_variables)]
    fn op_0049(&mut self, op_size: u8) -> u8 {
        let res = self.get_c();
        self.set_c(res);

        4
    }

    /// LD C,D
    #[allow(unused_variables)]
    fn op_004a(&mut self, op_size: u8) -> u8 {
        let res = self.get_d();
        self.set_c(res);

        4
    }

    /// LD C,E
    #[allow(unused_variables)]
    fn op_004b(&mut self, op_size: u8) -> u8 {
        let res = self.get_e();
        self.set_c(res);

        4
    }

    /// LD C,H
    #[allow(unused_variables)]
    fn op_004c(&mut self, op_size: u8) -> u8 {
        let res = self.get_h();
        self.set_c(res);

        4
    }

    /// LD C,L
    #[allow(unused_variables)]
    fn op_004d(&mut self, op_size: u8) -> u8 {
        let res = self.get_l();
        self.set_c(res);

        4
    }

    /// LD C,(HL)
    #[allow(unused_variables)]
    fn op_004e(&mut self, op_size: u8) -> u8 {
        let res = self.mem_read_u8(self.get_hl());
        self.set_c(res);

        8
    }

    /// LD C,A
    #[allow(unused_variables)]
    fn op_004f(&mut self, op_size: u8) -> u8 {
        let res = self.get_a();
        self.set_c(res);

        4
    }

    /// LD D,B
    #[allow(unused_variables)]
    fn op_0050(&mut self, op_size: u8) -> u8 {
        let res = self.get_b();
        self.set_d(res);

        4
    }

    /// LD D,C
    #[allow(unused_variables)]
    fn op_0051(&mut self, op_size: u8) -> u8 {
        let res = self.get_c();
        self.set_d(res);

        4
    }

    /// LD D,D
    #[allow(unused_variables)]
    fn op_0052(&mut self, op_size: u8) -> u8 {
        let res = self.get_d();
        self.set_d(res);

        4
    }

    /// LD D,E
    #[allow(unused_variables)]
    fn op_0053(&mut self, op_size: u8) -> u8 {
        let res = self.get_e();
        self.set_d(res);

        4
    }

    /// LD D,H
    #[allow(unused_variables)]
    fn op_0054(&mut self, op_size: u8) -> u8 {
        let res = self.get_h();
        self.set_d(res);

        4
    }

    /// LD D,L
    #[allow(unused_variables)]
    fn op_0055(&mut self, op_size: u8) -> u8 {
        let res = self.get_l();
        self.set_d(res);

        4
    }

    /// LD D,(HL)
    #[allow(unused_variables)]
    fn op_0056(&mut self, op_size: u8) -> u8 {
        let res = self.mem_read_u8(self.get_hl());
        self.set_d(res);

        8
    }

    /// LD D,A
    #[allow(unused_variables)]
    fn op_0057(&mut self, op_size: u8) -> u8 {
        let res = self.get_a();
        self.set_d(res);

        4
    }

    /// LD E,B
    #[allow(unused_variables)]
    fn op_0058(&mut self, op_size: u8) -> u8 {
        let res = self.get_b();
        self.set_e(res);

        4
    }

    /// LD E,C
    #[allow(unused_variables)]
    fn op_0059(&mut self, op_size: u8) -> u8 {
        let res = self.get_c();
        self.set_e(res);

        4
    }

    /// LD E,D
    #[allow(unused_variables)]
    fn op_005a(&mut self, op_size: u8) -> u8 {
        let res = self.get_d();
        self.set_e(res);

        4
    }

    /// LD E,E
    #[allow(unused_variables)]
    fn op_005b(&mut self, op_size: u8) -> u8 {
        let res = self.get_e();
        self.set_e(res);

        4
    }

    /// LD E,H
    #[allow(unused_variables)]
    fn op_005c(&mut self, op_size: u8) -> u8 {
        let res = self.get_h();
        self.set_e(res);

        4
    }

    /// LD E,L
    #[allow(unused_variables)]
    fn op_005d(&mut self, op_size: u8) -> u8 {
        let res = self.get_l();
        self.set_e(res);

        4
    }

    /// LD E,(HL)
    #[allow(unused_variables)]
    fn op_005e(&mut self, op_size: u8) -> u8 {
        let res = self.mem_read_u8(self.get_hl());
        self.set_e(res);

        8
    }

    /// LD E,A
    #[allow(unused_variables)]
    fn op_005f(&mut self, op_size: u8) -> u8 {
        let res = self.get_a();
        self.set_e(res);

        4
    }

    /// LD H,B
    #[allow(unused_variables)]
    fn op_0060(&mut self, op_size: u8) -> u8 {
        let res = self.get_b();
        self.set_h(res);

        4
    }

    /// LD H,C
    #[allow(unused_variables)]
    fn op_0061(&mut self, op_size: u8) -> u8 {
        let res = self.get_c();
        self.set_h(res);

        4
    }

    /// LD H,D
    #[allow(unused_variables)]
    fn op_0062(&mut self, op_size: u8) -> u8 {
        let res = self.get_d();
        self.set_h(res);

        4
    }

    /// LD H,E
    #[allow(unused_variables)]
    fn op_0063(&mut self, op_size: u8) -> u8 {
        let res = self.get_e();
        self.set_h(res);

        4
    }

    /// LD H,H
    #[allow(unused_variables)]
    fn op_0064(&mut self, op_size: u8) -> u8 {
        let res = self.get_h();
        self.set_h(res);

        4
    }

    /// LD H,L
    #[allow(unused_variables)]
    fn op_0065(&mut self, op_size: u8) -> u8 {
        let res = self.get_l();
        self.set_h(res);

        4
    }

    /// LD H,(HL)
    #[allow(unused_variables)]
    fn op_0066(&mut self, op_size: u8) -> u8 {
        let res = self.mem_read_u8(self.get_hl());
        self.set_h(res);

        8
    }

    /// LD H,A
    #[allow(unused_variables)]
    fn op_0067(&mut self, op_size: u8) -> u8 {
        let res = self.get_a();
        self.set_h(res);

        4
    }

    /// LD L,B
    #[allow(unused_variables)]
    fn op_0068(&mut self, op_size: u8) -> u8 {
        let res = self.get_b();
        self.set_l(res);

        4
    }

    /// LD L,C
    #[allow(unused_variables)]
    fn op_0069(&mut self, op_size: u8) -> u8 {
        let res = self.get_c();
        self.set_l(res);

        4
    }

    /// LD L,D
    #[allow(unused_variables)]
    fn op_006a(&mut self, op_size: u8) -> u8 {
        let res = self.get_d();
        self.set_l(res);

        4
    }

    /// LD L,E
    #[allow(unused_variables)]
    fn op_006b(&mut self, op_size: u8) -> u8 {
        let res = self.get_e();
        self.set_l(res);

        4
    }

    /// LD L,H
    #[allow(unused_variables)]
    fn op_006c(&mut self, op_size: u8) -> u8 {
        let res = self.get_h();
        self.set_l(res);

        4
    }

    /// LD L,L
    #[allow(unused_variables)]
    fn op_006d(&mut self, op_size: u8) -> u8 {
        let res = self.get_l();
        self.set_l(res);

        4
    }

    /// LD L,(HL)
    #[allow(unused_variables)]
    fn op_006e(&mut self, op_size: u8) -> u8 {
        let res = self.mem_read_u8(self.get_hl());
        self.set_l(res);

        8
    }

    /// LD L,A
    #[allow(unused_variables)]
    fn op_006f(&mut self, op_size: u8) -> u8 {
        let res = self.get_a();
        self.set_l(res);

        4
    }

    /// LD (HL),B
    #[allow(unused_variables)]
    fn op_0070(&mut self, op_size: u8) -> u8 {
        let res = self.get_b();
        self.mem_write_u8(self.get_hl(), res);

        8
    }

    /// LD (HL),C
    #[allow(unused_variables)]
    fn op_0071(&mut self, op_size: u8) -> u8 {
        let res = self.get_c();
        self.mem_write_u8(self.get_hl(), res);

        8
    }

    /// LD (HL),D
    #[allow(unused_variables)]
    fn op_0072(&mut self, op_size: u8) -> u8 {
        let res = self.get_d();
        self.mem_write_u8(self.get_hl(), res);

        8
    }

    /// LD (HL),E
    #[allow(unused_variables)]
    fn op_0073(&mut self, op_size: u8) -> u8 {
        let res = self.get_e();
        self.mem_write_u8(self.get_hl(), res);

        8
    }

    /// LD (HL),H
    #[allow(unused_variables)]
    fn op_0074(&mut self, op_size: u8) -> u8 {
        let res = self.get_h();
        self.mem_write_u8(self.get_hl(), res);

        8
    }

    /// LD (HL),L
    #[allow(unused_variables)]
    fn op_0075(&mut self, op_size: u8) -> u8 {
        let res = self.get_l();
        self.mem_write_u8(self.get_hl(), res);

        8
    }

    /// HALT
    #[allow(unused_variables)]
    fn op_0076(&mut self, op_size: u8) -> u8 {
        self.halt();

        4
    }

    /// LD (HL),A
    #[allow(unused_variables)]
    fn op_0077(&mut self, op_size: u8) -> u8 {
        let res = self.get_a();
        self.mem_write_u8(self.get_hl(), res);

        8
    }

    /// LD A,B
    #[allow(unused_variables)]
    fn op_0078(&mut self, op_size: u8) -> u8 {
        let res = self.get_b();
        self.set_a(res);

        4
    }

    /// LD A,C
    #[allow(unused_variables)]
    fn op_0079(&mut self, op_size: u8) -> u8 {
        let res = self.get_c();
        self.set_a(res);

        4
    }

    /// LD A,D
    #[allow(unused_variables)]
    fn op_007a(&mut self, op_size: u8) -> u8 {
        let res = self.get_d();
        self.set_a(res);

        4
    }

    /// LD A,E
    #[allow(unused_variables)]
    fn op_007b(&mut self, op_size: u8) -> u8 {
        let res = self.get_e();
        self.set_a(res);

        4
    }

    /// LD A,H
    #[allow(unused_variables)]
    fn op_007c(&mut self, op_size: u8) -> u8 {
        let res = self.get_h();
        self.set_a(res);

        4
    }

    /// LD A,L
    #[allow(unused_variables)]
    fn op_007d(&mut self, op_size: u8) -> u8 {
        let res = self.get_l();
        self.set_a(res);

        4
    }

    /// LD A,(HL)
    #[allow(unused_variables)]
    fn op_007e(&mut self, op_size: u8) -> u8 {
        let res = self.mem_read_u8(self.get_hl());
        self.set_a(res);

        8
    }

    /// LD A,A
    #[allow(unused_variables)]
    fn op_007f(&mut self, op_size: u8) -> u8 {
        let res = self.get_a();
        self.set_a(res);

        4
    }

    /// ADD A,B
    #[allow(unused_variables)]
    fn op_0080(&mut self, op_size: u8) -> u8 {
        let x = self.get_a();
        let y = self.get_b();
        let (res, z, h, c) = alu::add_u8(x, y, false);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// ADD A,C
    #[allow(unused_variables)]
    fn op_0081(&mut self, op_size: u8) -> u8 {
        let x = self.get_a();
        let y = self.get_c();
        let (res, z, h, c) = alu::add_u8(x, y, false);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// ADD A,D
    #[allow(unused_variables)]
    fn op_0082(&mut self, op_size: u8) -> u8 {
        let x = self.get_a();
        let y = self.get_d();
        let (res, z, h, c) = alu::add_u8(x, y, false);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// ADD A,E
    #[allow(unused_variables)]
    fn op_0083(&mut self, op_size: u8) -> u8 {
        let x = self.get_a();
        let y = self.get_e();
        let (res, z, h, c) = alu::add_u8(x, y, false);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// ADD A,H
    #[allow(unused_variables)]
    fn op_0084(&mut self, op_size: u8) -> u8 {
        let x = self.get_a();
        let y = self.get_h();
        let (res, z, h, c) = alu::add_u8(x, y, false);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// ADD A,L
    #[allow(unused_variables)]
    fn op_0085(&mut self, op_size: u8) -> u8 {
        let x = self.get_a();
        let y = self.get_l();
        let (res, z, h, c) = alu::add_u8(x, y, false);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// ADD A,(HL)
    #[allow(unused_variables)]
    fn op_0086(&mut self, op_size: u8) -> u8 {
        let x = self.get_a();
        let y = self.mem_read_u8(self.get_hl());
        let (res, z, h, c) = alu::add_u8(x, y, false);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// ADD A,A
    #[allow(unused_variables)]
    fn op_0087(&mut self, op_size: u8) -> u8 {
        let x = self.get_a();
        let y = self.get_a();
        let (res, z, h, c) = alu::add_u8(x, y, false);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// ADC A,B
    #[allow(unused_variables)]
    fn op_0088(&mut self, op_size: u8) -> u8 {
        let carry = self.status.contains(StatusFlags::C);
        let (res, z, h, c) = alu::add_u8(self.get_a(), self.get_b(), carry);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// ADC A,C
    #[allow(unused_variables)]
    fn op_0089(&mut self, op_size: u8) -> u8 {
        let carry = self.status.contains(StatusFlags::C);
        let (res, z, h, c) = alu::add_u8(self.get_a(), self.get_c(), carry);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// ADC A,D
    #[allow(unused_variables)]
    fn op_008a(&mut self, op_size: u8) -> u8 {
        let carry = self.status.contains(StatusFlags::C);
        let (res, z, h, c) = alu::add_u8(self.get_a(), self.get_d(), carry);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// ADC A,E
    #[allow(unused_variables)]
    fn op_008b(&mut self, op_size: u8) -> u8 {
        let carry = self.status.contains(StatusFlags::C);
        let (res, z, h, c) = alu::add_u8(self.get_a(), self.get_e(), carry);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// ADC A,H
    #[allow(unused_variables)]
    fn op_008c(&mut self, op_size: u8) -> u8 {
        let carry = self.status.contains(StatusFlags::C);
        let (res, z, h, c) = alu::add_u8(self.get_a(), self.get_h(), carry);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// ADC A,L
    #[allow(unused_variables)]
    fn op_008d(&mut self, op_size: u8) -> u8 {
        let carry = self.status.contains(StatusFlags::C);
        let (res, z, h, c) = alu::add_u8(self.get_a(), self.get_l(), carry);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// ADC A,(HL)
    #[allow(unused_variables)]
    fn op_008e(&mut self, op_size: u8) -> u8 {
        let carry = self.status.contains(StatusFlags::C);
        let (res, z, h, c) = alu::add_u8(self.get_a(), self.mem_read_u8(self.get_hl()), carry);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// ADC A,A
    #[allow(unused_variables)]
    fn op_008f(&mut self, op_size: u8) -> u8 {
        let carry = self.status.contains(StatusFlags::C);
        let (res, z, h, c) = alu::add_u8(self.get_a(), self.get_a(), carry);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// SUB A,B
    #[allow(unused_variables)]
    fn op_0090(&mut self, op_size: u8) -> u8 {
        let (res, z, h, c) = alu::sub_u8(self.get_a(), self.get_b(), false);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.insert(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// SUB A,C
    #[allow(unused_variables)]
    fn op_0091(&mut self, op_size: u8) -> u8 {
        let (res, z, h, c) = alu::sub_u8(self.get_a(), self.get_c(), false);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.insert(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// SUB A,D
    #[allow(unused_variables)]
    fn op_0092(&mut self, op_size: u8) -> u8 {
        let (res, z, h, c) = alu::sub_u8(self.get_a(), self.get_d(), false);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.insert(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// SUB A,E
    #[allow(unused_variables)]
    fn op_0093(&mut self, op_size: u8) -> u8 {
        let (res, z, h, c) = alu::sub_u8(self.get_a(), self.get_e(), false);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.insert(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// SUB A,H
    #[allow(unused_variables)]
    fn op_0094(&mut self, op_size: u8) -> u8 {
        let (res, z, h, c) = alu::sub_u8(self.get_a(), self.get_h(), false);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.insert(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// SUB A,L
    #[allow(unused_variables)]
    fn op_0095(&mut self, op_size: u8) -> u8 {
        let (res, z, h, c) = alu::sub_u8(self.get_a(), self.get_l(), false);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.insert(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// SUB A,(HL)
    #[allow(unused_variables)]
    fn op_0096(&mut self, op_size: u8) -> u8 {
        let (res, z, h, c) = alu::sub_u8(self.get_a(), self.mem_read_u8(self.get_hl()), false);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.insert(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// SUB A,A
    #[allow(unused_variables)]
    fn op_0097(&mut self, op_size: u8) -> u8 {
        let (res, z, h, c) = alu::sub_u8(self.get_a(), self.get_a(), false);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.insert(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// SBC A,B
    #[allow(unused_variables)]
    fn op_0098(&mut self, op_size: u8) -> u8 {
        let carry = self.status.contains(StatusFlags::C);
        let (res, z, h, c) = alu::sub_u8(self.get_a(), self.get_b(), carry);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.insert(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// SBC A,C
    #[allow(unused_variables)]
    fn op_0099(&mut self, op_size: u8) -> u8 {
        let carry = self.status.contains(StatusFlags::C);
        let (res, z, h, c) = alu::sub_u8(self.get_a(), self.get_c(), carry);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.insert(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// SBC A,D
    #[allow(unused_variables)]
    fn op_009a(&mut self, op_size: u8) -> u8 {
        let carry = self.status.contains(StatusFlags::C);
        let (res, z, h, c) = alu::sub_u8(self.get_a(), self.get_d(), carry);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.insert(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// SBC A,E
    #[allow(unused_variables)]
    fn op_009b(&mut self, op_size: u8) -> u8 {
        let carry = self.status.contains(StatusFlags::C);
        let (res, z, h, c) = alu::sub_u8(self.get_a(), self.get_e(), carry);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.insert(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// SBC A,H
    #[allow(unused_variables)]
    fn op_009c(&mut self, op_size: u8) -> u8 {
        let carry = self.status.contains(StatusFlags::C);
        let (res, z, h, c) = alu::sub_u8(self.get_a(), self.get_h(), carry);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.insert(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// SBC A,L
    #[allow(unused_variables)]
    fn op_009d(&mut self, op_size: u8) -> u8 {
        let carry = self.status.contains(StatusFlags::C);
        let (res, z, h, c) = alu::sub_u8(self.get_a(), self.get_l(), carry);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.insert(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// SBC A,(HL)
    #[allow(unused_variables)]
    fn op_009e(&mut self, op_size: u8) -> u8 {
        let carry = self.status.contains(StatusFlags::C);
        let (res, z, h, c) = alu::sub_u8(self.get_a(), self.mem_read_u8(self.get_hl()), carry);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.insert(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// SBC A,A
    #[allow(unused_variables)]
    fn op_009f(&mut self, op_size: u8) -> u8 {
        let carry = self.status.contains(StatusFlags::C);
        let (res, z, h, c) = alu::sub_u8(self.get_a(), self.get_a(), carry);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.insert(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// AND A,B
    #[allow(unused_variables)]
    fn op_00a0(&mut self, op_size: u8) -> u8 {
        self.a = self.a & self.get_b();
        let z = self.a == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);
        self.status.remove(StatusFlags::C);

        4
    }

    /// AND A,C
    #[allow(unused_variables)]
    fn op_00a1(&mut self, op_size: u8) -> u8 {
        self.a = self.a & self.get_c();
        let z = self.a == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);
        self.status.remove(StatusFlags::C);

        4
    }

    /// AND A,D
    #[allow(unused_variables)]
    fn op_00a2(&mut self, op_size: u8) -> u8 {
        self.a = self.a & self.get_d();
        let z = self.a == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);
        self.status.remove(StatusFlags::C);

        4
    }

    /// AND A,E
    #[allow(unused_variables)]
    fn op_00a3(&mut self, op_size: u8) -> u8 {
        self.a = self.a & self.get_e();
        let z = self.a == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);
        self.status.remove(StatusFlags::C);

        4
    }

    /// AND A,H
    #[allow(unused_variables)]
    fn op_00a4(&mut self, op_size: u8) -> u8 {
        self.a = self.a & self.get_h();
        let z = self.a == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);
        self.status.remove(StatusFlags::C);

        4
    }

    /// AND A,L
    #[allow(unused_variables)]
    fn op_00a5(&mut self, op_size: u8) -> u8 {
        self.a = self.a & self.get_l();
        let z = self.a == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);
        self.status.remove(StatusFlags::C);

        4
    }

    /// AND A,(HL)
    #[allow(unused_variables)]
    fn op_00a6(&mut self, op_size: u8) -> u8 {
        self.a = self.a & self.mem_read_u8(self.get_hl());
        let z = self.a == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);
        self.status.remove(StatusFlags::C);

        8
    }

    /// AND A,A
    #[allow(unused_variables)]
    fn op_00a7(&mut self, op_size: u8) -> u8 {
        self.a = self.a & self.get_a();
        let z = self.a == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);
        self.status.remove(StatusFlags::C);

        4
    }

    /// XOR A,B
    #[allow(unused_variables)]
    fn op_00a8(&mut self, op_size: u8) -> u8 {
        self.a = self.a ^ self.get_b();
        let z = self.a == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.remove(StatusFlags::C);

        4
    }

    /// XOR A,C
    #[allow(unused_variables)]
    fn op_00a9(&mut self, op_size: u8) -> u8 {
        self.a = self.a ^ self.get_c();
        let z = self.a == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.remove(StatusFlags::C);

        4
    }

    /// XOR A,D
    #[allow(unused_variables)]
    fn op_00aa(&mut self, op_size: u8) -> u8 {
        self.a = self.a ^ self.get_d();
        let z = self.a == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.remove(StatusFlags::C);

        4
    }

    /// XOR A,E
    #[allow(unused_variables)]
    fn op_00ab(&mut self, op_size: u8) -> u8 {
        self.a = self.a ^ self.get_e();
        let z = self.a == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.remove(StatusFlags::C);

        4
    }

    /// XOR A,H
    #[allow(unused_variables)]
    fn op_00ac(&mut self, op_size: u8) -> u8 {
        self.a = self.a ^ self.get_h();
        let z = self.a == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.remove(StatusFlags::C);

        4
    }

    /// XOR A,L
    #[allow(unused_variables)]
    fn op_00ad(&mut self, op_size: u8) -> u8 {
        self.a = self.a ^ self.get_l();
        let z = self.a == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.remove(StatusFlags::C);

        4
    }

    /// XOR A,(HL)
    #[allow(unused_variables)]
    fn op_00ae(&mut self, op_size: u8) -> u8 {
        self.a = self.a ^ self.mem_read_u8(self.get_hl());
        let z = self.a == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.remove(StatusFlags::C);

        8
    }

    /// XOR A,A
    #[allow(unused_variables)]
    fn op_00af(&mut self, op_size: u8) -> u8 {
        self.a = self.a ^ self.get_a();
        let z = self.a == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.remove(StatusFlags::C);

        4
    }

    /// OR A,B
    #[allow(unused_variables)]
    fn op_00b0(&mut self, op_size: u8) -> u8 {
        self.a = self.a | self.get_b();
        let z = self.a == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.remove(StatusFlags::C);

        4
    }

    /// OR A,C
    #[allow(unused_variables)]
    fn op_00b1(&mut self, op_size: u8) -> u8 {
        self.a = self.a | self.get_c();
        let z = self.a == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.remove(StatusFlags::C);

        4
    }

    /// OR A,D
    #[allow(unused_variables)]
    fn op_00b2(&mut self, op_size: u8) -> u8 {
        self.a = self.a | self.get_d();
        let z = self.a == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.remove(StatusFlags::C);

        4
    }

    /// OR A,E
    #[allow(unused_variables)]
    fn op_00b3(&mut self, op_size: u8) -> u8 {
        self.a = self.a | self.get_e();
        let z = self.a == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.remove(StatusFlags::C);

        4
    }

    /// OR A,H
    #[allow(unused_variables)]
    fn op_00b4(&mut self, op_size: u8) -> u8 {
        self.a = self.a | self.get_h();
        let z = self.a == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.remove(StatusFlags::C);

        4
    }

    /// OR A,L
    #[allow(unused_variables)]
    fn op_00b5(&mut self, op_size: u8) -> u8 {
        self.a = self.a | self.get_l();
        let z = self.a == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.remove(StatusFlags::C);

        4
    }

    /// OR A,(HL)
    #[allow(unused_variables)]
    fn op_00b6(&mut self, op_size: u8) -> u8 {
        self.a = self.a | self.mem_read_u8(self.get_hl());
        let z = self.a == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.remove(StatusFlags::C);

        8
    }

    /// OR A,A
    #[allow(unused_variables)]
    fn op_00b7(&mut self, op_size: u8) -> u8 {
        self.a = self.a | self.get_a();
        let z = self.a == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.remove(StatusFlags::C);

        4
    }

    /// CP A,B
    #[allow(unused_variables)]
    fn op_00b8(&mut self, op_size: u8) -> u8 {
        let x = self.get_b();
        let (res, _, h, _) = alu::sub_u8(self.a, x, false);
        let z = res == 0;
        let c = self.a < x;

        self.status.set(StatusFlags::Z, z);
        self.status.insert(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// CP A,C
    #[allow(unused_variables)]
    fn op_00b9(&mut self, op_size: u8) -> u8 {
        let x = self.get_c();
        let (res, _, h, _) = alu::sub_u8(self.a, x, false);
        let z = res == 0;
        let c = self.a < x;

        self.status.set(StatusFlags::Z, z);
        self.status.insert(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// CP A,D
    #[allow(unused_variables)]
    fn op_00ba(&mut self, op_size: u8) -> u8 {
        let x = self.get_d();
        let (res, _, h, _) = alu::sub_u8(self.a, x, false);
        let z = res == 0;
        let c = self.a < x;

        self.status.set(StatusFlags::Z, z);
        self.status.insert(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// CP A,E
    #[allow(unused_variables)]
    fn op_00bb(&mut self, op_size: u8) -> u8 {
        let x = self.get_e();
        let (res, _, h, _) = alu::sub_u8(self.a, x, false);
        let z = res == 0;
        let c = self.a < x;

        self.status.set(StatusFlags::Z, z);
        self.status.insert(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// CP A,H
    #[allow(unused_variables)]
    fn op_00bc(&mut self, op_size: u8) -> u8 {
        let x = self.get_h();
        let (res, _, h, _) = alu::sub_u8(self.a, x, false);
        let z = res == 0;
        let c = self.a < x;

        self.status.set(StatusFlags::Z, z);
        self.status.insert(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// CP A,L
    #[allow(unused_variables)]
    fn op_00bd(&mut self, op_size: u8) -> u8 {
        let x = self.get_l();
        let (res, _, h, _) = alu::sub_u8(self.a, x, false);
        let z = res == 0;
        let c = self.a < x;

        self.status.set(StatusFlags::Z, z);
        self.status.insert(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// CP A,(HL)
    #[allow(unused_variables)]
    fn op_00be(&mut self, op_size: u8) -> u8 {
        let x = self.mem_read_u8(self.get_hl());
        let (res, _, h, _) = alu::sub_u8(self.a, x, false);
        let z = res == 0;
        let c = self.a < x;

        self.status.set(StatusFlags::Z, z);
        self.status.insert(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// CP A,A
    #[allow(unused_variables)]
    fn op_00bf(&mut self, op_size: u8) -> u8 {
        let x = self.get_a();
        let (res, _, h, _) = alu::sub_u8(self.a, x, false);
        let z = res == 0;
        let c = self.a < x;

        self.status.set(StatusFlags::Z, z);
        self.status.insert(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        4
    }

    /// RET NZ
    #[allow(unused_variables)]
    fn op_00c0(&mut self, op_size: u8) -> u8 {
        let flg = !self.status.contains(StatusFlags::Z);
        if flg {
            self.program_counter = self.stack_pop();
            return 8;
        }

        20
    }

    /// POP BC
    #[allow(unused_variables)]
    fn op_00c1(&mut self, op_size: u8) -> u8 {
        let res = self.stack_pop();
        self.set_bc(res);

        12
    }

    /// JP NZ,u16
    #[allow(unused_variables)]
    fn op_00c2(&mut self, op_size: u8) -> u8 {
        let flg = !self.status.contains(StatusFlags::Z);
        if flg {
            let pc = self.mem_read_u16(self.program_counter);
            self.program_counter = pc;
            return 12;
        }

        16
    }

    /// JP u16
    #[allow(unused_variables)]
    fn op_00c3(&mut self, op_size: u8) -> u8 {
        self.program_counter = self.mem_read_u16(self.program_counter);

        16
    }

    /// CALL NZ,u16
    #[allow(unused_variables)]
    fn op_00c4(&mut self, op_size: u8) -> u8 {
        let flg = !self.status.contains(StatusFlags::Z);
        if flg {
            self.stack_push(self.program_counter.wrapping_add(2));
            self.program_counter = self.mem_read_u16(self.program_counter);
            return 12;
        }

        24
    }

    /// PUSH BC
    #[allow(unused_variables)]
    fn op_00c5(&mut self, op_size: u8) -> u8 {
        self.stack_push(self.get_bc());

        16
    }

    /// ADD A,u8
    #[allow(unused_variables)]
    fn op_00c6(&mut self, op_size: u8) -> u8 {
        let x = self.get_a();
        let y = self.mem_read_u8(self.program_counter);
        let (res, z, h, c) = alu::add_u8(x, y, false);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// RST 00h
    #[allow(unused_variables)]
    fn op_00c7(&mut self, op_size: u8) -> u8 {
        self.stack_push(self.program_counter);
        self.program_counter = 0x00;

        16
    }

    /// RET Z
    #[allow(unused_variables)]
    fn op_00c8(&mut self, op_size: u8) -> u8 {
        let flg = self.status.contains(StatusFlags::Z);
        if flg {
            self.program_counter = self.stack_pop();
            return 8;
        }

        20
    }

    /// RET
    #[allow(unused_variables)]
    fn op_00c9(&mut self, op_size: u8) -> u8 {
        self.program_counter = self.stack_pop();

        16
    }

    /// JP Z,u16
    #[allow(unused_variables)]
    fn op_00ca(&mut self, op_size: u8) -> u8 {
        let flg = self.status.contains(StatusFlags::Z);
        if flg {
            let pc = self.mem_read_u16(self.program_counter);
            self.program_counter = pc;
            return 12;
        }

        16
    }

    /// PREFIX CB
    #[allow(unused_variables)]
    fn op_00cb(&mut self, op_size: u8) -> u8 {
        4
    }

    /// CALL Z,u16
    #[allow(unused_variables)]
    fn op_00cc(&mut self, op_size: u8) -> u8 {
        let flg = self.status.contains(StatusFlags::Z);
        if flg {
            self.stack_push(self.program_counter.wrapping_add(2));
            self.program_counter = self.mem_read_u16(self.program_counter);
            return 12;
        }

        24
    }

    /// CALL u16
    #[allow(unused_variables)]
    fn op_00cd(&mut self, op_size: u8) -> u8 {
        self.stack_push(self.program_counter.wrapping_add(2));
        self.program_counter = self.mem_read_u16(self.program_counter);

        24
    }

    /// ADC A,u8
    #[allow(unused_variables)]
    fn op_00ce(&mut self, op_size: u8) -> u8 {
        let carry = self.status.contains(StatusFlags::C);
        let (res, z, h, c) =
            alu::add_u8(self.get_a(), self.mem_read_u8(self.program_counter), carry);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// RST 08h
    #[allow(unused_variables)]
    fn op_00cf(&mut self, op_size: u8) -> u8 {
        self.stack_push(self.program_counter);
        self.program_counter = 0x08;

        16
    }

    /// RET NC
    #[allow(unused_variables)]
    fn op_00d0(&mut self, op_size: u8) -> u8 {
        let flg = !self.status.contains(StatusFlags::C);
        if flg {
            self.program_counter = self.stack_pop();
            return 8;
        }

        20
    }

    /// POP DE
    #[allow(unused_variables)]
    fn op_00d1(&mut self, op_size: u8) -> u8 {
        let res = self.stack_pop();
        self.set_de(res);

        12
    }

    /// JP NC,u16
    #[allow(unused_variables)]
    fn op_00d2(&mut self, op_size: u8) -> u8 {
        let flg = !self.status.contains(StatusFlags::C);
        if flg {
            let pc = self.mem_read_u16(self.program_counter);
            self.program_counter = pc;
            return 12;
        }

        16
    }

    /// CALL NC,u16
    #[allow(unused_variables)]
    fn op_00d4(&mut self, op_size: u8) -> u8 {
        let flg = !self.status.contains(StatusFlags::C);
        if flg {
            self.stack_push(self.program_counter.wrapping_add(2));
            self.program_counter = self.mem_read_u16(self.program_counter);
            return 12;
        }

        24
    }

    /// PUSH DE
    #[allow(unused_variables)]
    fn op_00d5(&mut self, op_size: u8) -> u8 {
        self.stack_push(self.get_de());

        16
    }

    /// SUB A,u8
    #[allow(unused_variables)]
    fn op_00d6(&mut self, op_size: u8) -> u8 {
        let (res, z, h, c) =
            alu::sub_u8(self.get_a(), self.mem_read_u8(self.program_counter), false);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.insert(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// RST 10h
    #[allow(unused_variables)]
    fn op_00d7(&mut self, op_size: u8) -> u8 {
        self.stack_push(self.program_counter);
        self.program_counter = 0x10;

        16
    }

    /// RET C
    #[allow(unused_variables)]
    fn op_00d8(&mut self, op_size: u8) -> u8 {
        let flg = self.status.contains(StatusFlags::C);
        if flg {
            self.program_counter = self.stack_pop();
            return 8;
        }

        20
    }

    /// RETI
    #[allow(unused_variables)]
    fn op_00d9(&mut self, op_size: u8) -> u8 {
        self.program_counter = self.stack_pop();
        self.enable_interrupt();

        16
    }

    /// JP C,u16
    #[allow(unused_variables)]
    fn op_00da(&mut self, op_size: u8) -> u8 {
        let flg = self.status.contains(StatusFlags::C);
        if flg {
            let pc = self.mem_read_u16(self.program_counter);
            self.program_counter = pc;
            return 12;
        }

        16
    }

    /// CALL C,u16
    #[allow(unused_variables)]
    fn op_00dc(&mut self, op_size: u8) -> u8 {
        let flg = self.status.contains(StatusFlags::C);
        if flg {
            self.stack_push(self.program_counter.wrapping_add(2));
            self.program_counter = self.mem_read_u16(self.program_counter);
            return 12;
        }

        24
    }

    /// SBC A,u8
    #[allow(unused_variables)]
    fn op_00de(&mut self, op_size: u8) -> u8 {
        let carry = self.status.contains(StatusFlags::C);
        let (res, z, h, c) =
            alu::sub_u8(self.get_a(), self.mem_read_u8(self.program_counter), carry);
        self.set_a(res);

        self.status.set(StatusFlags::Z, z);
        self.status.insert(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// RST 18h
    #[allow(unused_variables)]
    fn op_00df(&mut self, op_size: u8) -> u8 {
        self.stack_push(self.program_counter);
        self.program_counter = 0x18;

        16
    }

    /// LD (FF00+u8),A
    #[allow(unused_variables)]
    fn op_00e0(&mut self, op_size: u8) -> u8 {
        let res = self.get_a();
        self.mem_write_u8(
            0xFF00 + (self.mem_read_u8(self.program_counter) as u16),
            res,
        );

        12
    }

    /// POP HL
    #[allow(unused_variables)]
    fn op_00e1(&mut self, op_size: u8) -> u8 {
        let res = self.stack_pop();
        self.set_hl(res);

        12
    }

    /// LD (FF00+C),A
    #[allow(unused_variables)]
    fn op_00e2(&mut self, op_size: u8) -> u8 {
        let res = self.get_a();
        self.mem_write_u8(0xFF00 + (self.get_c() as u16), res);

        8
    }

    /// PUSH HL
    #[allow(unused_variables)]
    fn op_00e5(&mut self, op_size: u8) -> u8 {
        self.stack_push(self.get_hl());

        16
    }

    /// AND A,u8
    #[allow(unused_variables)]
    fn op_00e6(&mut self, op_size: u8) -> u8 {
        self.a = self.a & self.mem_read_u8(self.program_counter);
        let z = self.a == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);
        self.status.remove(StatusFlags::C);

        8
    }

    /// RST 20h
    #[allow(unused_variables)]
    fn op_00e7(&mut self, op_size: u8) -> u8 {
        self.stack_push(self.program_counter);
        self.program_counter = 0x20;

        16
    }

    /// ADD SP,i8
    #[allow(unused_variables)]
    fn op_00e8(&mut self, op_size: u8) -> u8 {
        let x = self.get_sp();
        let y = self.mem_read_u8(self.program_counter) as u16;
        let (res, z, h, c) = alu::add_u16(x, y, false);
        self.set_sp(res);

        self.status.remove(StatusFlags::Z);
        self.status.remove(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        16
    }

    /// JP HL
    #[allow(unused_variables)]
    fn op_00e9(&mut self, op_size: u8) -> u8 {
        self.program_counter = self.get_hl();

        4
    }

    /// LD (u16),A
    #[allow(unused_variables)]
    fn op_00ea(&mut self, op_size: u8) -> u8 {
        let res = self.get_a();
        self.mem_write_u8(self.mem_read_u16(self.program_counter), res);

        16
    }

    /// XOR A,u8
    #[allow(unused_variables)]
    fn op_00ee(&mut self, op_size: u8) -> u8 {
        self.a = self.a ^ self.mem_read_u8(self.program_counter);
        let z = self.a == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.remove(StatusFlags::C);

        8
    }

    /// RST 28h
    #[allow(unused_variables)]
    fn op_00ef(&mut self, op_size: u8) -> u8 {
        self.stack_push(self.program_counter);
        self.program_counter = 0x28;

        16
    }

    /// LD A,(FF00+u8)
    #[allow(unused_variables)]
    fn op_00f0(&mut self, op_size: u8) -> u8 {
        let res = self.mem_read_u8(0xFF00 + (self.mem_read_u8(self.program_counter) as u16));
        self.set_a(res);

        12
    }

    /// POP AF
    #[allow(unused_variables)]
    fn op_00f1(&mut self, op_size: u8) -> u8 {
        let res = self.stack_pop();
        self.set_af(res);

        12
    }

    /// LD A,(FF00+C)
    #[allow(unused_variables)]
    fn op_00f2(&mut self, op_size: u8) -> u8 {
        let res = self.mem_read_u8(0xFF00 + (self.get_c() as u16));
        self.set_a(res);

        8
    }

    /// DI
    #[allow(unused_variables)]
    fn op_00f3(&mut self, op_size: u8) -> u8 {
        self.disable_interrupt();

        4
    }

    /// PUSH AF
    #[allow(unused_variables)]
    fn op_00f5(&mut self, op_size: u8) -> u8 {
        self.stack_push(self.get_af());

        16
    }

    /// OR A,u8
    #[allow(unused_variables)]
    fn op_00f6(&mut self, op_size: u8) -> u8 {
        self.a = self.a | self.mem_read_u8(self.program_counter);
        let z = self.a == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.remove(StatusFlags::C);

        8
    }

    /// RST 30h
    #[allow(unused_variables)]
    fn op_00f7(&mut self, op_size: u8) -> u8 {
        self.stack_push(self.program_counter);
        self.program_counter = 0x30;

        16
    }

    /// LD HL,SP+i8
    #[allow(unused_variables)]
    fn op_00f8(&mut self, op_size: u8) -> u8 {
        // Yo MAMA
        let (res, _, h, c) = alu::add_u16(
            self.get_sp(),
            self.mem_read_u8(self.program_counter) as u16,
            false,
        );
        self.set_hl(res);

        self.status.remove(StatusFlags::Z);
        self.status.remove(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        12
    }

    /// LD SP,HL
    #[allow(unused_variables)]
    fn op_00f9(&mut self, op_size: u8) -> u8 {
        let res = self.get_hl();
        self.set_sp(res);

        8
    }

    /// LD A,(u16)
    #[allow(unused_variables)]
    fn op_00fa(&mut self, op_size: u8) -> u8 {
        let res = self.mem_read_u8(self.mem_read_u16(self.program_counter));
        self.set_a(res);

        16
    }

    /// EI
    #[allow(unused_variables)]
    fn op_00fb(&mut self, op_size: u8) -> u8 {
        self.enable_interrupt();

        4
    }

    /// CP A,u8
    #[allow(unused_variables)]
    fn op_00fe(&mut self, op_size: u8) -> u8 {
        let x = self.mem_read_u8(self.program_counter);
        let (res, _, h, _) = alu::sub_u8(self.a, x, false);
        let z = res == 0;
        let c = self.a < x;

        self.status.set(StatusFlags::Z, z);
        self.status.insert(StatusFlags::N);
        self.status.set(StatusFlags::H, h);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// RST 38h
    #[allow(unused_variables)]
    fn op_00ff(&mut self, op_size: u8) -> u8 {
        self.stack_push(self.program_counter);
        self.program_counter = 0x38;

        16
    }

    /// RLC B
    #[allow(unused_variables)]
    fn op_cb00(&mut self, op_size: u8) -> u8 {
        let v = self.get_b();
        let res = v.rotate_left(1);
        self.set_b(res);

        let z = res == 0;
        let c = v & 0x80 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// RLC C
    #[allow(unused_variables)]
    fn op_cb01(&mut self, op_size: u8) -> u8 {
        let v = self.get_c();
        let res = v.rotate_left(1);
        self.set_c(res);

        let z = res == 0;
        let c = v & 0x80 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// RLC D
    #[allow(unused_variables)]
    fn op_cb02(&mut self, op_size: u8) -> u8 {
        let v = self.get_d();
        let res = v.rotate_left(1);
        self.set_d(res);

        let z = res == 0;
        let c = v & 0x80 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// RLC E
    #[allow(unused_variables)]
    fn op_cb03(&mut self, op_size: u8) -> u8 {
        let v = self.get_e();
        let res = v.rotate_left(1);
        self.set_e(res);

        let z = res == 0;
        let c = v & 0x80 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// RLC H
    #[allow(unused_variables)]
    fn op_cb04(&mut self, op_size: u8) -> u8 {
        let v = self.get_h();
        let res = v.rotate_left(1);
        self.set_h(res);

        let z = res == 0;
        let c = v & 0x80 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// RLC L
    #[allow(unused_variables)]
    fn op_cb05(&mut self, op_size: u8) -> u8 {
        let v = self.get_l();
        let res = v.rotate_left(1);
        self.set_l(res);

        let z = res == 0;
        let c = v & 0x80 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// RLC (HL)
    #[allow(unused_variables)]
    fn op_cb06(&mut self, op_size: u8) -> u8 {
        let v = self.mem_read_u8(self.get_hl());
        let res = v.rotate_left(1);
        self.mem_write_u8(self.get_hl(), res);

        let z = res == 0;
        let c = v & 0x80 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        16
    }

    /// RLC A
    #[allow(unused_variables)]
    fn op_cb07(&mut self, op_size: u8) -> u8 {
        let v = self.get_a();
        let res = v.rotate_left(1);
        self.set_a(res);

        let z = res == 0;
        let c = v & 0x80 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// RRC B
    #[allow(unused_variables)]
    fn op_cb08(&mut self, op_size: u8) -> u8 {
        let v = self.get_b();
        let res = v.rotate_right(1);
        self.set_b(res);

        let z = res == 0;
        let c = v & 0x01 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// RRC C
    #[allow(unused_variables)]
    fn op_cb09(&mut self, op_size: u8) -> u8 {
        let v = self.get_c();
        let res = v.rotate_right(1);
        self.set_c(res);

        let z = res == 0;
        let c = v & 0x01 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// RRC D
    #[allow(unused_variables)]
    fn op_cb0a(&mut self, op_size: u8) -> u8 {
        let v = self.get_d();
        let res = v.rotate_right(1);
        self.set_d(res);

        let z = res == 0;
        let c = v & 0x01 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// RRC E
    #[allow(unused_variables)]
    fn op_cb0b(&mut self, op_size: u8) -> u8 {
        let v = self.get_e();
        let res = v.rotate_right(1);
        self.set_e(res);

        let z = res == 0;
        let c = v & 0x01 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// RRC H
    #[allow(unused_variables)]
    fn op_cb0c(&mut self, op_size: u8) -> u8 {
        let v = self.get_h();
        let res = v.rotate_right(1);
        self.set_h(res);

        let z = res == 0;
        let c = v & 0x01 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// RRC L
    #[allow(unused_variables)]
    fn op_cb0d(&mut self, op_size: u8) -> u8 {
        let v = self.get_l();
        let res = v.rotate_right(1);
        self.set_l(res);

        let z = res == 0;
        let c = v & 0x01 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// RRC (HL)
    #[allow(unused_variables)]
    fn op_cb0e(&mut self, op_size: u8) -> u8 {
        let v = self.mem_read_u8(self.get_hl());
        let res = v.rotate_right(1);
        self.mem_write_u8(self.get_hl(), res);

        let z = res == 0;
        let c = v & 0x01 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        16
    }

    /// RRC A
    #[allow(unused_variables)]
    fn op_cb0f(&mut self, op_size: u8) -> u8 {
        let v = self.get_a();
        let res = v.rotate_right(1);
        self.set_a(res);

        let z = res == 0;
        let c = v & 0x01 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// RL B
    #[allow(unused_variables)]
    fn op_cb10(&mut self, op_size: u8) -> u8 {
        let v = self.get_b();

        let mut res = v.wrapping_shl(1);
        res |= if self.get_cf() { 1 } else { 0 };

        self.set_b(res);

        let z = res == 0;
        let c = v & 0x80 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// RL C
    #[allow(unused_variables)]
    fn op_cb11(&mut self, op_size: u8) -> u8 {
        let v = self.get_c();

        let mut res = v.wrapping_shl(1);
        res |= if self.get_cf() { 1 } else { 0 };

        self.set_c(res);

        let z = res == 0;
        let c = v & 0x80 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// RL D
    #[allow(unused_variables)]
    fn op_cb12(&mut self, op_size: u8) -> u8 {
        let v = self.get_d();

        let mut res = v.wrapping_shl(1);
        res |= if self.get_cf() { 1 } else { 0 };

        self.set_d(res);

        let z = res == 0;
        let c = v & 0x80 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// RL E
    #[allow(unused_variables)]
    fn op_cb13(&mut self, op_size: u8) -> u8 {
        let v = self.get_e();

        let mut res = v.wrapping_shl(1);
        res |= if self.get_cf() { 1 } else { 0 };

        self.set_e(res);

        let z = res == 0;
        let c = v & 0x80 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// RL H
    #[allow(unused_variables)]
    fn op_cb14(&mut self, op_size: u8) -> u8 {
        let v = self.get_h();

        let mut res = v.wrapping_shl(1);
        res |= if self.get_cf() { 1 } else { 0 };

        self.set_h(res);

        let z = res == 0;
        let c = v & 0x80 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// RL L
    #[allow(unused_variables)]
    fn op_cb15(&mut self, op_size: u8) -> u8 {
        let v = self.get_l();

        let mut res = v.wrapping_shl(1);
        res |= if self.get_cf() { 1 } else { 0 };

        self.set_l(res);

        let z = res == 0;
        let c = v & 0x80 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// RL (HL)
    #[allow(unused_variables)]
    fn op_cb16(&mut self, op_size: u8) -> u8 {
        let v = self.mem_read_u8(self.get_hl());

        let mut res = v.wrapping_shl(1);
        res |= if self.get_cf() { 1 } else { 0 };

        self.mem_write_u8(self.get_hl(), res);

        let z = res == 0;
        let c = v & 0x80 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        16
    }

    /// RL A
    #[allow(unused_variables)]
    fn op_cb17(&mut self, op_size: u8) -> u8 {
        let v = self.get_a();

        let mut res = v.wrapping_shl(1);
        res |= if self.get_cf() { 1 } else { 0 };

        self.set_a(res);

        let z = res == 0;
        let c = v & 0x80 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// RR B
    #[allow(unused_variables)]
    fn op_cb18(&mut self, op_size: u8) -> u8 {
        let v = self.get_b();

        let mut res = v.wrapping_shr(1);
        res |= if self.get_cf() { 0x80 } else { 0 };

        self.set_b(res);

        let z = res == 0;
        let c = v & 0x01 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// RR C
    #[allow(unused_variables)]
    fn op_cb19(&mut self, op_size: u8) -> u8 {
        let v = self.get_c();

        let mut res = v.wrapping_shr(1);
        res |= if self.get_cf() { 0x80 } else { 0 };

        self.set_c(res);

        let z = res == 0;
        let c = v & 0x01 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// RR D
    #[allow(unused_variables)]
    fn op_cb1a(&mut self, op_size: u8) -> u8 {
        let v = self.get_d();

        let mut res = v.wrapping_shr(1);
        res |= if self.get_cf() { 0x80 } else { 0 };

        self.set_d(res);

        let z = res == 0;
        let c = v & 0x01 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// RR E
    #[allow(unused_variables)]
    fn op_cb1b(&mut self, op_size: u8) -> u8 {
        let v = self.get_e();

        let mut res = v.wrapping_shr(1);
        res |= if self.get_cf() { 0x80 } else { 0 };

        self.set_e(res);

        let z = res == 0;
        let c = v & 0x01 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// RR H
    #[allow(unused_variables)]
    fn op_cb1c(&mut self, op_size: u8) -> u8 {
        let v = self.get_h();

        let mut res = v.wrapping_shr(1);
        res |= if self.get_cf() { 0x80 } else { 0 };

        self.set_h(res);

        let z = res == 0;
        let c = v & 0x01 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// RR L
    #[allow(unused_variables)]
    fn op_cb1d(&mut self, op_size: u8) -> u8 {
        let v = self.get_l();

        let mut res = v.wrapping_shr(1);
        res |= if self.get_cf() { 0x80 } else { 0 };

        self.set_l(res);

        let z = res == 0;
        let c = v & 0x01 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// RR (HL)
    #[allow(unused_variables)]
    fn op_cb1e(&mut self, op_size: u8) -> u8 {
        let v = self.mem_read_u8(self.get_hl());

        let mut res = v.wrapping_shr(1);
        res |= if self.get_cf() { 0x80 } else { 0 };

        self.mem_write_u8(self.get_hl(), res);

        let z = res == 0;
        let c = v & 0x01 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        16
    }

    /// RR A
    #[allow(unused_variables)]
    fn op_cb1f(&mut self, op_size: u8) -> u8 {
        let v = self.get_a();

        let mut res = v.wrapping_shr(1);
        res |= if self.get_cf() { 0x80 } else { 0 };

        self.set_a(res);

        let z = res == 0;
        let c = v & 0x01 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// SLA B
    #[allow(unused_variables)]
    fn op_cb20(&mut self, op_size: u8) -> u8 {
        let v = self.get_b();
        let res = v.wrapping_shl(1);
        self.set_b(res);

        let z = v == 0;
        let c = v & 0x80 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// SLA C
    #[allow(unused_variables)]
    fn op_cb21(&mut self, op_size: u8) -> u8 {
        let v = self.get_c();
        let res = v.wrapping_shl(1);
        self.set_c(res);

        let z = v == 0;
        let c = v & 0x80 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// SLA D
    #[allow(unused_variables)]
    fn op_cb22(&mut self, op_size: u8) -> u8 {
        let v = self.get_d();
        let res = v.wrapping_shl(1);
        self.set_d(res);

        let z = v == 0;
        let c = v & 0x80 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// SLA E
    #[allow(unused_variables)]
    fn op_cb23(&mut self, op_size: u8) -> u8 {
        let v = self.get_e();
        let res = v.wrapping_shl(1);
        self.set_e(res);

        let z = v == 0;
        let c = v & 0x80 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// SLA H
    #[allow(unused_variables)]
    fn op_cb24(&mut self, op_size: u8) -> u8 {
        let v = self.get_h();
        let res = v.wrapping_shl(1);
        self.set_h(res);

        let z = v == 0;
        let c = v & 0x80 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// SLA L
    #[allow(unused_variables)]
    fn op_cb25(&mut self, op_size: u8) -> u8 {
        let v = self.get_l();
        let res = v.wrapping_shl(1);
        self.set_l(res);

        let z = v == 0;
        let c = v & 0x80 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// SLA (HL)
    #[allow(unused_variables)]
    fn op_cb26(&mut self, op_size: u8) -> u8 {
        let v = self.mem_read_u8(self.get_hl());
        let res = v.wrapping_shl(1);
        self.mem_write_u8(self.get_hl(), res);

        let z = v == 0;
        let c = v & 0x80 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        16
    }

    /// SLA A
    #[allow(unused_variables)]
    fn op_cb27(&mut self, op_size: u8) -> u8 {
        let v = self.get_a();
        let res = v.wrapping_shl(1);
        self.set_a(res);

        let z = v == 0;
        let c = v & 0x80 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// SRA B
    #[allow(unused_variables)]
    fn op_cb28(&mut self, op_size: u8) -> u8 {
        let v = self.get_b();
        let msb = v & 0x80;
        let res = v.wrapping_shr(1);
        let res = res | msb;
        self.set_b(res);

        let z = res == 0;
        let c = v & 0x01 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// SRA C
    #[allow(unused_variables)]
    fn op_cb29(&mut self, op_size: u8) -> u8 {
        let v = self.get_c();
        let msb = v & 0x80;
        let res = v.wrapping_shr(1);
        let res = res | msb;
        self.set_c(res);

        let z = res == 0;
        let c = v & 0x01 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// SRA D
    #[allow(unused_variables)]
    fn op_cb2a(&mut self, op_size: u8) -> u8 {
        let v = self.get_d();
        let msb = v & 0x80;
        let res = v.wrapping_shr(1);
        let res = res | msb;
        self.set_d(res);

        let z = res == 0;
        let c = v & 0x01 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// SRA E
    #[allow(unused_variables)]
    fn op_cb2b(&mut self, op_size: u8) -> u8 {
        let v = self.get_e();
        let msb = v & 0x80;
        let res = v.wrapping_shr(1);
        let res = res | msb;
        self.set_e(res);

        let z = res == 0;
        let c = v & 0x01 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// SRA H
    #[allow(unused_variables)]
    fn op_cb2c(&mut self, op_size: u8) -> u8 {
        let v = self.get_h();
        let msb = v & 0x80;
        let res = v.wrapping_shr(1);
        let res = res | msb;
        self.set_h(res);

        let z = res == 0;
        let c = v & 0x01 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// SRA L
    #[allow(unused_variables)]
    fn op_cb2d(&mut self, op_size: u8) -> u8 {
        let v = self.get_l();
        let msb = v & 0x80;
        let res = v.wrapping_shr(1);
        let res = res | msb;
        self.set_l(res);

        let z = res == 0;
        let c = v & 0x01 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// SRA (HL)
    #[allow(unused_variables)]
    fn op_cb2e(&mut self, op_size: u8) -> u8 {
        let v = self.mem_read_u8(self.get_hl());
        let msb = v & 0x80;
        let res = v.wrapping_shr(1);
        let res = res | msb;
        self.mem_write_u8(self.get_hl(), res);

        let z = res == 0;
        let c = v & 0x01 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        16
    }

    /// SRA A
    #[allow(unused_variables)]
    fn op_cb2f(&mut self, op_size: u8) -> u8 {
        let v = self.get_a();
        let msb = v & 0x80;
        let res = v.wrapping_shr(1);
        let res = res | msb;
        self.set_a(res);

        let z = res == 0;
        let c = v & 0x01 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// SWAP B
    #[allow(unused_variables)]
    fn op_cb30(&mut self, op_size: u8) -> u8 {
        let mut res = self.get_b();
        let most_sig_nib = (res & 0b0000_1111) << 4;
        let least_sig_nib = (res & 0b1111_0000) >> 4;

        res = most_sig_nib | least_sig_nib;
        self.set_b(res);
        let z = res == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.remove(StatusFlags::C);

        8
    }

    /// SWAP C
    #[allow(unused_variables)]
    fn op_cb31(&mut self, op_size: u8) -> u8 {
        let mut res = self.get_c();
        let most_sig_nib = (res & 0b0000_1111) << 4;
        let least_sig_nib = (res & 0b1111_0000) >> 4;

        res = most_sig_nib | least_sig_nib;
        self.set_c(res);
        let z = res == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.remove(StatusFlags::C);

        8
    }

    /// SWAP D
    #[allow(unused_variables)]
    fn op_cb32(&mut self, op_size: u8) -> u8 {
        let mut res = self.get_d();
        let most_sig_nib = (res & 0b0000_1111) << 4;
        let least_sig_nib = (res & 0b1111_0000) >> 4;

        res = most_sig_nib | least_sig_nib;
        self.set_d(res);
        let z = res == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.remove(StatusFlags::C);

        8
    }

    /// SWAP E
    #[allow(unused_variables)]
    fn op_cb33(&mut self, op_size: u8) -> u8 {
        let mut res = self.get_e();
        let most_sig_nib = (res & 0b0000_1111) << 4;
        let least_sig_nib = (res & 0b1111_0000) >> 4;

        res = most_sig_nib | least_sig_nib;
        self.set_e(res);
        let z = res == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.remove(StatusFlags::C);

        8
    }

    /// SWAP H
    #[allow(unused_variables)]
    fn op_cb34(&mut self, op_size: u8) -> u8 {
        let mut res = self.get_h();
        let most_sig_nib = (res & 0b0000_1111) << 4;
        let least_sig_nib = (res & 0b1111_0000) >> 4;

        res = most_sig_nib | least_sig_nib;
        self.set_h(res);
        let z = res == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.remove(StatusFlags::C);

        8
    }

    /// SWAP L
    #[allow(unused_variables)]
    fn op_cb35(&mut self, op_size: u8) -> u8 {
        let mut res = self.get_l();
        let most_sig_nib = (res & 0b0000_1111) << 4;
        let least_sig_nib = (res & 0b1111_0000) >> 4;

        res = most_sig_nib | least_sig_nib;
        self.set_l(res);
        let z = res == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.remove(StatusFlags::C);

        8
    }

    /// SWAP (HL)
    #[allow(unused_variables)]
    fn op_cb36(&mut self, op_size: u8) -> u8 {
        let mut res = self.mem_read_u8(self.get_hl());
        let most_sig_nib = (res & 0b0000_1111) << 4;
        let least_sig_nib = (res & 0b1111_0000) >> 4;

        res = most_sig_nib | least_sig_nib;
        self.mem_write_u8(self.get_hl(), res);
        let z = res == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.remove(StatusFlags::C);

        16
    }

    /// SWAP A
    #[allow(unused_variables)]
    fn op_cb37(&mut self, op_size: u8) -> u8 {
        let mut res = self.get_a();
        let most_sig_nib = (res & 0b0000_1111) << 4;
        let least_sig_nib = (res & 0b1111_0000) >> 4;

        res = most_sig_nib | least_sig_nib;
        self.set_a(res);
        let z = res == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.remove(StatusFlags::C);

        8
    }

    /// SRL B
    #[allow(unused_variables)]
    fn op_cb38(&mut self, op_size: u8) -> u8 {
        let v = self.get_b();
        let res = v.wrapping_shr(1);
        self.set_b(res);

        let z = res == 0;
        let c = v & 0x01 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// SRL C
    #[allow(unused_variables)]
    fn op_cb39(&mut self, op_size: u8) -> u8 {
        let v = self.get_c();
        let res = v.wrapping_shr(1);
        self.set_c(res);

        let z = res == 0;
        let c = v & 0x01 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// SRL D
    #[allow(unused_variables)]
    fn op_cb3a(&mut self, op_size: u8) -> u8 {
        let v = self.get_d();
        let res = v.wrapping_shr(1);
        self.set_d(res);

        let z = res == 0;
        let c = v & 0x01 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// SRL E
    #[allow(unused_variables)]
    fn op_cb3b(&mut self, op_size: u8) -> u8 {
        let v = self.get_e();
        let res = v.wrapping_shr(1);
        self.set_e(res);

        let z = res == 0;
        let c = v & 0x01 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// SRL H
    #[allow(unused_variables)]
    fn op_cb3c(&mut self, op_size: u8) -> u8 {
        let v = self.get_h();
        let res = v.wrapping_shr(1);
        self.set_h(res);

        let z = res == 0;
        let c = v & 0x01 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// SRL L
    #[allow(unused_variables)]
    fn op_cb3d(&mut self, op_size: u8) -> u8 {
        let v = self.get_l();
        let res = v.wrapping_shr(1);
        self.set_l(res);

        let z = res == 0;
        let c = v & 0x01 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// SRL (HL)
    #[allow(unused_variables)]
    fn op_cb3e(&mut self, op_size: u8) -> u8 {
        let v = self.mem_read_u8(self.get_hl());
        let res = v.wrapping_shr(1);
        self.mem_write_u8(self.get_hl(), res);

        let z = res == 0;
        let c = v & 0x01 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        16
    }

    /// SRL A
    #[allow(unused_variables)]
    fn op_cb3f(&mut self, op_size: u8) -> u8 {
        let v = self.get_a();
        let res = v.wrapping_shr(1);
        self.set_a(res);

        let z = res == 0;
        let c = v & 0x01 != 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.remove(StatusFlags::H);
        self.status.set(StatusFlags::C, c);

        8
    }

    /// BIT 0,B
    #[allow(unused_variables)]
    fn op_cb40(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 0;
        let v = self.get_b();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 0,C
    #[allow(unused_variables)]
    fn op_cb41(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 0;
        let v = self.get_c();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 0,D
    #[allow(unused_variables)]
    fn op_cb42(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 0;
        let v = self.get_d();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 0,E
    #[allow(unused_variables)]
    fn op_cb43(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 0;
        let v = self.get_e();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 0,H
    #[allow(unused_variables)]
    fn op_cb44(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 0;
        let v = self.get_h();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 0,L
    #[allow(unused_variables)]
    fn op_cb45(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 0;
        let v = self.get_l();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 0,(HL)
    #[allow(unused_variables)]
    fn op_cb46(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 0;
        let v = self.mem_read_u8(self.get_hl());
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        12
    }

    /// BIT 0,A
    #[allow(unused_variables)]
    fn op_cb47(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 0;
        let v = self.get_a();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 1,B
    #[allow(unused_variables)]
    fn op_cb48(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 1;
        let v = self.get_b();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 1,C
    #[allow(unused_variables)]
    fn op_cb49(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 1;
        let v = self.get_c();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 1,D
    #[allow(unused_variables)]
    fn op_cb4a(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 1;
        let v = self.get_d();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 1,E
    #[allow(unused_variables)]
    fn op_cb4b(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 1;
        let v = self.get_e();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 1,H
    #[allow(unused_variables)]
    fn op_cb4c(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 1;
        let v = self.get_h();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 1,L
    #[allow(unused_variables)]
    fn op_cb4d(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 1;
        let v = self.get_l();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 1,(HL)
    #[allow(unused_variables)]
    fn op_cb4e(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 1;
        let v = self.mem_read_u8(self.get_hl());
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        12
    }

    /// BIT 1,A
    #[allow(unused_variables)]
    fn op_cb4f(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 1;
        let v = self.get_a();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 2,B
    #[allow(unused_variables)]
    fn op_cb50(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 2;
        let v = self.get_b();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 2,C
    #[allow(unused_variables)]
    fn op_cb51(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 2;
        let v = self.get_c();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 2,D
    #[allow(unused_variables)]
    fn op_cb52(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 2;
        let v = self.get_d();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 2,E
    #[allow(unused_variables)]
    fn op_cb53(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 2;
        let v = self.get_e();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 2,H
    #[allow(unused_variables)]
    fn op_cb54(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 2;
        let v = self.get_h();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 2,L
    #[allow(unused_variables)]
    fn op_cb55(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 2;
        let v = self.get_l();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 2,(HL)
    #[allow(unused_variables)]
    fn op_cb56(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 2;
        let v = self.mem_read_u8(self.get_hl());
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        12
    }

    /// BIT 2,A
    #[allow(unused_variables)]
    fn op_cb57(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 2;
        let v = self.get_a();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 3,B
    #[allow(unused_variables)]
    fn op_cb58(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 3;
        let v = self.get_b();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 3,C
    #[allow(unused_variables)]
    fn op_cb59(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 3;
        let v = self.get_c();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 3,D
    #[allow(unused_variables)]
    fn op_cb5a(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 3;
        let v = self.get_d();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 3,E
    #[allow(unused_variables)]
    fn op_cb5b(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 3;
        let v = self.get_e();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 3,H
    #[allow(unused_variables)]
    fn op_cb5c(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 3;
        let v = self.get_h();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 3,L
    #[allow(unused_variables)]
    fn op_cb5d(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 3;
        let v = self.get_l();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 3,(HL)
    #[allow(unused_variables)]
    fn op_cb5e(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 3;
        let v = self.mem_read_u8(self.get_hl());
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        12
    }

    /// BIT 3,A
    #[allow(unused_variables)]
    fn op_cb5f(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 3;
        let v = self.get_a();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 4,B
    #[allow(unused_variables)]
    fn op_cb60(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 4;
        let v = self.get_b();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 4,C
    #[allow(unused_variables)]
    fn op_cb61(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 4;
        let v = self.get_c();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 4,D
    #[allow(unused_variables)]
    fn op_cb62(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 4;
        let v = self.get_d();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 4,E
    #[allow(unused_variables)]
    fn op_cb63(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 4;
        let v = self.get_e();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 4,H
    #[allow(unused_variables)]
    fn op_cb64(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 4;
        let v = self.get_h();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 4,L
    #[allow(unused_variables)]
    fn op_cb65(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 4;
        let v = self.get_l();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 4,(HL)
    #[allow(unused_variables)]
    fn op_cb66(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 4;
        let v = self.mem_read_u8(self.get_hl());
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        12
    }

    /// BIT 4,A
    #[allow(unused_variables)]
    fn op_cb67(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 4;
        let v = self.get_a();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 5,B
    #[allow(unused_variables)]
    fn op_cb68(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 5;
        let v = self.get_b();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 5,C
    #[allow(unused_variables)]
    fn op_cb69(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 5;
        let v = self.get_c();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 5,D
    #[allow(unused_variables)]
    fn op_cb6a(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 5;
        let v = self.get_d();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 5,E
    #[allow(unused_variables)]
    fn op_cb6b(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 5;
        let v = self.get_e();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 5,H
    #[allow(unused_variables)]
    fn op_cb6c(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 5;
        let v = self.get_h();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 5,L
    #[allow(unused_variables)]
    fn op_cb6d(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 5;
        let v = self.get_l();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 5,(HL)
    #[allow(unused_variables)]
    fn op_cb6e(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 5;
        let v = self.mem_read_u8(self.get_hl());
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        12
    }

    /// BIT 5,A
    #[allow(unused_variables)]
    fn op_cb6f(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 5;
        let v = self.get_a();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 6,B
    #[allow(unused_variables)]
    fn op_cb70(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 6;
        let v = self.get_b();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 6,C
    #[allow(unused_variables)]
    fn op_cb71(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 6;
        let v = self.get_c();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 6,D
    #[allow(unused_variables)]
    fn op_cb72(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 6;
        let v = self.get_d();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 6,E
    #[allow(unused_variables)]
    fn op_cb73(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 6;
        let v = self.get_e();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 6,H
    #[allow(unused_variables)]
    fn op_cb74(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 6;
        let v = self.get_h();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 6,L
    #[allow(unused_variables)]
    fn op_cb75(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 6;
        let v = self.get_l();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 6,(HL)
    #[allow(unused_variables)]
    fn op_cb76(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 6;
        let v = self.mem_read_u8(self.get_hl());
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        12
    }

    /// BIT 6,A
    #[allow(unused_variables)]
    fn op_cb77(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 6;
        let v = self.get_a();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 7,B
    #[allow(unused_variables)]
    fn op_cb78(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 7;
        let v = self.get_b();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 7,C
    #[allow(unused_variables)]
    fn op_cb79(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 7;
        let v = self.get_c();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 7,D
    #[allow(unused_variables)]
    fn op_cb7a(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 7;
        let v = self.get_d();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 7,E
    #[allow(unused_variables)]
    fn op_cb7b(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 7;
        let v = self.get_e();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 7,H
    #[allow(unused_variables)]
    fn op_cb7c(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 7;
        let v = self.get_h();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 7,L
    #[allow(unused_variables)]
    fn op_cb7d(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 7;
        let v = self.get_l();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// BIT 7,(HL)
    #[allow(unused_variables)]
    fn op_cb7e(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 7;
        let v = self.mem_read_u8(self.get_hl());
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        12
    }

    /// BIT 7,A
    #[allow(unused_variables)]
    fn op_cb7f(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 7;
        let v = self.get_a();
        let z = (v & test_bit) == 0;

        self.status.set(StatusFlags::Z, z);
        self.status.remove(StatusFlags::N);
        self.status.insert(StatusFlags::H);

        8
    }

    /// RES 0,B
    #[allow(unused_variables)]
    fn op_cb80(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 0);
        let v = self.get_b();
        self.set_b(v & test_bit);

        8
    }

    /// RES 0,C
    #[allow(unused_variables)]
    fn op_cb81(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 0);
        let v = self.get_c();
        self.set_c(v & test_bit);

        8
    }

    /// RES 0,D
    #[allow(unused_variables)]
    fn op_cb82(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 0);
        let v = self.get_d();
        self.set_d(v & test_bit);

        8
    }

    /// RES 0,E
    #[allow(unused_variables)]
    fn op_cb83(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 0);
        let v = self.get_e();
        self.set_e(v & test_bit);

        8
    }

    /// RES 0,H
    #[allow(unused_variables)]
    fn op_cb84(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 0);
        let v = self.get_h();
        self.set_h(v & test_bit);

        8
    }

    /// RES 0,L
    #[allow(unused_variables)]
    fn op_cb85(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 0);
        let v = self.get_l();
        self.set_l(v & test_bit);

        8
    }

    /// RES 0,(HL)
    #[allow(unused_variables)]
    fn op_cb86(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 0);
        let v = self.mem_read_u8(self.get_hl());
        self.mem_write_u8(self.get_hl(), v & test_bit);

        16
    }

    /// RES 0,A
    #[allow(unused_variables)]
    fn op_cb87(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 0);
        let v = self.get_a();
        self.set_a(v & test_bit);

        8
    }

    /// RES 1,B
    #[allow(unused_variables)]
    fn op_cb88(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 1);
        let v = self.get_b();
        self.set_b(v & test_bit);

        8
    }

    /// RES 1,C
    #[allow(unused_variables)]
    fn op_cb89(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 1);
        let v = self.get_c();
        self.set_c(v & test_bit);

        8
    }

    /// RES 1,D
    #[allow(unused_variables)]
    fn op_cb8a(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 1);
        let v = self.get_d();
        self.set_d(v & test_bit);

        8
    }

    /// RES 1,E
    #[allow(unused_variables)]
    fn op_cb8b(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 1);
        let v = self.get_e();
        self.set_e(v & test_bit);

        8
    }

    /// RES 1,H
    #[allow(unused_variables)]
    fn op_cb8c(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 1);
        let v = self.get_h();
        self.set_h(v & test_bit);

        8
    }

    /// RES 1,L
    #[allow(unused_variables)]
    fn op_cb8d(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 1);
        let v = self.get_l();
        self.set_l(v & test_bit);

        8
    }

    /// RES 1,(HL)
    #[allow(unused_variables)]
    fn op_cb8e(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 1);
        let v = self.mem_read_u8(self.get_hl());
        self.mem_write_u8(self.get_hl(), v & test_bit);

        16
    }

    /// RES 1,A
    #[allow(unused_variables)]
    fn op_cb8f(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 1);
        let v = self.get_a();
        self.set_a(v & test_bit);

        8
    }

    /// RES 2,B
    #[allow(unused_variables)]
    fn op_cb90(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 2);
        let v = self.get_b();
        self.set_b(v & test_bit);

        8
    }

    /// RES 2,C
    #[allow(unused_variables)]
    fn op_cb91(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 2);
        let v = self.get_c();
        self.set_c(v & test_bit);

        8
    }

    /// RES 2,D
    #[allow(unused_variables)]
    fn op_cb92(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 2);
        let v = self.get_d();
        self.set_d(v & test_bit);

        8
    }

    /// RES 2,E
    #[allow(unused_variables)]
    fn op_cb93(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 2);
        let v = self.get_e();
        self.set_e(v & test_bit);

        8
    }

    /// RES 2,H
    #[allow(unused_variables)]
    fn op_cb94(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 2);
        let v = self.get_h();
        self.set_h(v & test_bit);

        8
    }

    /// RES 2,L
    #[allow(unused_variables)]
    fn op_cb95(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 2);
        let v = self.get_l();
        self.set_l(v & test_bit);

        8
    }

    /// RES 2,(HL)
    #[allow(unused_variables)]
    fn op_cb96(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 2);
        let v = self.mem_read_u8(self.get_hl());
        self.mem_write_u8(self.get_hl(), v & test_bit);

        16
    }

    /// RES 2,A
    #[allow(unused_variables)]
    fn op_cb97(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 2);
        let v = self.get_a();
        self.set_a(v & test_bit);

        8
    }

    /// RES 3,B
    #[allow(unused_variables)]
    fn op_cb98(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 3);
        let v = self.get_b();
        self.set_b(v & test_bit);

        8
    }

    /// RES 3,C
    #[allow(unused_variables)]
    fn op_cb99(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 3);
        let v = self.get_c();
        self.set_c(v & test_bit);

        8
    }

    /// RES 3,D
    #[allow(unused_variables)]
    fn op_cb9a(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 3);
        let v = self.get_d();
        self.set_d(v & test_bit);

        8
    }

    /// RES 3,E
    #[allow(unused_variables)]
    fn op_cb9b(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 3);
        let v = self.get_e();
        self.set_e(v & test_bit);

        8
    }

    /// RES 3,H
    #[allow(unused_variables)]
    fn op_cb9c(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 3);
        let v = self.get_h();
        self.set_h(v & test_bit);

        8
    }

    /// RES 3,L
    #[allow(unused_variables)]
    fn op_cb9d(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 3);
        let v = self.get_l();
        self.set_l(v & test_bit);

        8
    }

    /// RES 3,(HL)
    #[allow(unused_variables)]
    fn op_cb9e(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 3);
        let v = self.mem_read_u8(self.get_hl());
        self.mem_write_u8(self.get_hl(), v & test_bit);

        16
    }

    /// RES 3,A
    #[allow(unused_variables)]
    fn op_cb9f(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 3);
        let v = self.get_a();
        self.set_a(v & test_bit);

        8
    }

    /// RES 4,B
    #[allow(unused_variables)]
    fn op_cba0(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 4);
        let v = self.get_b();
        self.set_b(v & test_bit);

        8
    }

    /// RES 4,C
    #[allow(unused_variables)]
    fn op_cba1(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 4);
        let v = self.get_c();
        self.set_c(v & test_bit);

        8
    }

    /// RES 4,D
    #[allow(unused_variables)]
    fn op_cba2(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 4);
        let v = self.get_d();
        self.set_d(v & test_bit);

        8
    }

    /// RES 4,E
    #[allow(unused_variables)]
    fn op_cba3(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 4);
        let v = self.get_e();
        self.set_e(v & test_bit);

        8
    }

    /// RES 4,H
    #[allow(unused_variables)]
    fn op_cba4(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 4);
        let v = self.get_h();
        self.set_h(v & test_bit);

        8
    }

    /// RES 4,L
    #[allow(unused_variables)]
    fn op_cba5(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 4);
        let v = self.get_l();
        self.set_l(v & test_bit);

        8
    }

    /// RES 4,(HL)
    #[allow(unused_variables)]
    fn op_cba6(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 4);
        let v = self.mem_read_u8(self.get_hl());
        self.mem_write_u8(self.get_hl(), v & test_bit);

        16
    }

    /// RES 4,A
    #[allow(unused_variables)]
    fn op_cba7(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 4);
        let v = self.get_a();
        self.set_a(v & test_bit);

        8
    }

    /// RES 5,B
    #[allow(unused_variables)]
    fn op_cba8(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 5);
        let v = self.get_b();
        self.set_b(v & test_bit);

        8
    }

    /// RES 5,C
    #[allow(unused_variables)]
    fn op_cba9(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 5);
        let v = self.get_c();
        self.set_c(v & test_bit);

        8
    }

    /// RES 5,D
    #[allow(unused_variables)]
    fn op_cbaa(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 5);
        let v = self.get_d();
        self.set_d(v & test_bit);

        8
    }

    /// RES 5,E
    #[allow(unused_variables)]
    fn op_cbab(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 5);
        let v = self.get_e();
        self.set_e(v & test_bit);

        8
    }

    /// RES 5,H
    #[allow(unused_variables)]
    fn op_cbac(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 5);
        let v = self.get_h();
        self.set_h(v & test_bit);

        8
    }

    /// RES 5,L
    #[allow(unused_variables)]
    fn op_cbad(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 5);
        let v = self.get_l();
        self.set_l(v & test_bit);

        8
    }

    /// RES 5,(HL)
    #[allow(unused_variables)]
    fn op_cbae(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 5);
        let v = self.mem_read_u8(self.get_hl());
        self.mem_write_u8(self.get_hl(), v & test_bit);

        16
    }

    /// RES 5,A
    #[allow(unused_variables)]
    fn op_cbaf(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 5);
        let v = self.get_a();
        self.set_a(v & test_bit);

        8
    }

    /// RES 6,B
    #[allow(unused_variables)]
    fn op_cbb0(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 6);
        let v = self.get_b();
        self.set_b(v & test_bit);

        8
    }

    /// RES 6,C
    #[allow(unused_variables)]
    fn op_cbb1(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 6);
        let v = self.get_c();
        self.set_c(v & test_bit);

        8
    }

    /// RES 6,D
    #[allow(unused_variables)]
    fn op_cbb2(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 6);
        let v = self.get_d();
        self.set_d(v & test_bit);

        8
    }

    /// RES 6,E
    #[allow(unused_variables)]
    fn op_cbb3(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 6);
        let v = self.get_e();
        self.set_e(v & test_bit);

        8
    }

    /// RES 6,H
    #[allow(unused_variables)]
    fn op_cbb4(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 6);
        let v = self.get_h();
        self.set_h(v & test_bit);

        8
    }

    /// RES 6,L
    #[allow(unused_variables)]
    fn op_cbb5(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 6);
        let v = self.get_l();
        self.set_l(v & test_bit);

        8
    }

    /// RES 6,(HL)
    #[allow(unused_variables)]
    fn op_cbb6(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 6);
        let v = self.mem_read_u8(self.get_hl());
        self.mem_write_u8(self.get_hl(), v & test_bit);

        16
    }

    /// RES 6,A
    #[allow(unused_variables)]
    fn op_cbb7(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 6);
        let v = self.get_a();
        self.set_a(v & test_bit);

        8
    }

    /// RES 7,B
    #[allow(unused_variables)]
    fn op_cbb8(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 7);
        let v = self.get_b();
        self.set_b(v & test_bit);

        8
    }

    /// RES 7,C
    #[allow(unused_variables)]
    fn op_cbb9(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 7);
        let v = self.get_c();
        self.set_c(v & test_bit);

        8
    }

    /// RES 7,D
    #[allow(unused_variables)]
    fn op_cbba(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 7);
        let v = self.get_d();
        self.set_d(v & test_bit);

        8
    }

    /// RES 7,E
    #[allow(unused_variables)]
    fn op_cbbb(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 7);
        let v = self.get_e();
        self.set_e(v & test_bit);

        8
    }

    /// RES 7,H
    #[allow(unused_variables)]
    fn op_cbbc(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 7);
        let v = self.get_h();
        self.set_h(v & test_bit);

        8
    }

    /// RES 7,L
    #[allow(unused_variables)]
    fn op_cbbd(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 7);
        let v = self.get_l();
        self.set_l(v & test_bit);

        8
    }

    /// RES 7,(HL)
    #[allow(unused_variables)]
    fn op_cbbe(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 7);
        let v = self.mem_read_u8(self.get_hl());
        self.mem_write_u8(self.get_hl(), v & test_bit);

        16
    }

    /// RES 7,A
    #[allow(unused_variables)]
    fn op_cbbf(&mut self, op_size: u8) -> u8 {
        let test_bit = !(1 << 7);
        let v = self.get_a();
        self.set_a(v & test_bit);

        8
    }

    /// SET 0,B
    #[allow(unused_variables)]
    fn op_cbc0(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 0;
        let v = self.get_b();
        self.set_b(v | test_bit);

        8
    }

    /// SET 0,C
    #[allow(unused_variables)]
    fn op_cbc1(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 0;
        let v = self.get_c();
        self.set_c(v | test_bit);

        8
    }

    /// SET 0,D
    #[allow(unused_variables)]
    fn op_cbc2(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 0;
        let v = self.get_d();
        self.set_d(v | test_bit);

        8
    }

    /// SET 0,E
    #[allow(unused_variables)]
    fn op_cbc3(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 0;
        let v = self.get_e();
        self.set_e(v | test_bit);

        8
    }

    /// SET 0,H
    #[allow(unused_variables)]
    fn op_cbc4(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 0;
        let v = self.get_h();
        self.set_h(v | test_bit);

        8
    }

    /// SET 0,L
    #[allow(unused_variables)]
    fn op_cbc5(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 0;
        let v = self.get_l();
        self.set_l(v | test_bit);

        8
    }

    /// SET 0,(HL)
    #[allow(unused_variables)]
    fn op_cbc6(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 0;
        let v = self.mem_read_u8(self.get_hl());
        self.mem_write_u8(self.get_hl(), v | test_bit);

        16
    }

    /// SET 0,A
    #[allow(unused_variables)]
    fn op_cbc7(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 0;
        let v = self.get_a();
        self.set_a(v | test_bit);

        8
    }

    /// SET 1,B
    #[allow(unused_variables)]
    fn op_cbc8(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 1;
        let v = self.get_b();
        self.set_b(v | test_bit);

        8
    }

    /// SET 1,C
    #[allow(unused_variables)]
    fn op_cbc9(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 1;
        let v = self.get_c();
        self.set_c(v | test_bit);

        8
    }

    /// SET 1,D
    #[allow(unused_variables)]
    fn op_cbca(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 1;
        let v = self.get_d();
        self.set_d(v | test_bit);

        8
    }

    /// SET 1,E
    #[allow(unused_variables)]
    fn op_cbcb(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 1;
        let v = self.get_e();
        self.set_e(v | test_bit);

        8
    }

    /// SET 1,H
    #[allow(unused_variables)]
    fn op_cbcc(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 1;
        let v = self.get_h();
        self.set_h(v | test_bit);

        8
    }

    /// SET 1,L
    #[allow(unused_variables)]
    fn op_cbcd(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 1;
        let v = self.get_l();
        self.set_l(v | test_bit);

        8
    }

    /// SET 1,(HL)
    #[allow(unused_variables)]
    fn op_cbce(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 1;
        let v = self.mem_read_u8(self.get_hl());
        self.mem_write_u8(self.get_hl(), v | test_bit);

        16
    }

    /// SET 1,A
    #[allow(unused_variables)]
    fn op_cbcf(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 1;
        let v = self.get_a();
        self.set_a(v | test_bit);

        8
    }

    /// SET 2,B
    #[allow(unused_variables)]
    fn op_cbd0(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 2;
        let v = self.get_b();
        self.set_b(v | test_bit);

        8
    }

    /// SET 2,C
    #[allow(unused_variables)]
    fn op_cbd1(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 2;
        let v = self.get_c();
        self.set_c(v | test_bit);

        8
    }

    /// SET 2,D
    #[allow(unused_variables)]
    fn op_cbd2(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 2;
        let v = self.get_d();
        self.set_d(v | test_bit);

        8
    }

    /// SET 2,E
    #[allow(unused_variables)]
    fn op_cbd3(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 2;
        let v = self.get_e();
        self.set_e(v | test_bit);

        8
    }

    /// SET 2,H
    #[allow(unused_variables)]
    fn op_cbd4(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 2;
        let v = self.get_h();
        self.set_h(v | test_bit);

        8
    }

    /// SET 2,L
    #[allow(unused_variables)]
    fn op_cbd5(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 2;
        let v = self.get_l();
        self.set_l(v | test_bit);

        8
    }

    /// SET 2,(HL)
    #[allow(unused_variables)]
    fn op_cbd6(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 2;
        let v = self.mem_read_u8(self.get_hl());
        self.mem_write_u8(self.get_hl(), v | test_bit);

        16
    }

    /// SET 2,A
    #[allow(unused_variables)]
    fn op_cbd7(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 2;
        let v = self.get_a();
        self.set_a(v | test_bit);

        8
    }

    /// SET 3,B
    #[allow(unused_variables)]
    fn op_cbd8(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 3;
        let v = self.get_b();
        self.set_b(v | test_bit);

        8
    }

    /// SET 3,C
    #[allow(unused_variables)]
    fn op_cbd9(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 3;
        let v = self.get_c();
        self.set_c(v | test_bit);

        8
    }

    /// SET 3,D
    #[allow(unused_variables)]
    fn op_cbda(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 3;
        let v = self.get_d();
        self.set_d(v | test_bit);

        8
    }

    /// SET 3,E
    #[allow(unused_variables)]
    fn op_cbdb(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 3;
        let v = self.get_e();
        self.set_e(v | test_bit);

        8
    }

    /// SET 3,H
    #[allow(unused_variables)]
    fn op_cbdc(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 3;
        let v = self.get_h();
        self.set_h(v | test_bit);

        8
    }

    /// SET 3,L
    #[allow(unused_variables)]
    fn op_cbdd(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 3;
        let v = self.get_l();
        self.set_l(v | test_bit);

        8
    }

    /// SET 3,(HL)
    #[allow(unused_variables)]
    fn op_cbde(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 3;
        let v = self.mem_read_u8(self.get_hl());
        self.mem_write_u8(self.get_hl(), v | test_bit);

        16
    }

    /// SET 3,A
    #[allow(unused_variables)]
    fn op_cbdf(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 3;
        let v = self.get_a();
        self.set_a(v | test_bit);

        8
    }

    /// SET 4,B
    #[allow(unused_variables)]
    fn op_cbe0(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 4;
        let v = self.get_b();
        self.set_b(v | test_bit);

        8
    }

    /// SET 4,C
    #[allow(unused_variables)]
    fn op_cbe1(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 4;
        let v = self.get_c();
        self.set_c(v | test_bit);

        8
    }

    /// SET 4,D
    #[allow(unused_variables)]
    fn op_cbe2(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 4;
        let v = self.get_d();
        self.set_d(v | test_bit);

        8
    }

    /// SET 4,E
    #[allow(unused_variables)]
    fn op_cbe3(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 4;
        let v = self.get_e();
        self.set_e(v | test_bit);

        8
    }

    /// SET 4,H
    #[allow(unused_variables)]
    fn op_cbe4(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 4;
        let v = self.get_h();
        self.set_h(v | test_bit);

        8
    }

    /// SET 4,L
    #[allow(unused_variables)]
    fn op_cbe5(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 4;
        let v = self.get_l();
        self.set_l(v | test_bit);

        8
    }

    /// SET 4,(HL)
    #[allow(unused_variables)]
    fn op_cbe6(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 4;
        let v = self.mem_read_u8(self.get_hl());
        self.mem_write_u8(self.get_hl(), v | test_bit);

        16
    }

    /// SET 4,A
    #[allow(unused_variables)]
    fn op_cbe7(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 4;
        let v = self.get_a();
        self.set_a(v | test_bit);

        8
    }

    /// SET 5,B
    #[allow(unused_variables)]
    fn op_cbe8(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 5;
        let v = self.get_b();
        self.set_b(v | test_bit);

        8
    }

    /// SET 5,C
    #[allow(unused_variables)]
    fn op_cbe9(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 5;
        let v = self.get_c();
        self.set_c(v | test_bit);

        8
    }

    /// SET 5,D
    #[allow(unused_variables)]
    fn op_cbea(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 5;
        let v = self.get_d();
        self.set_d(v | test_bit);

        8
    }

    /// SET 5,E
    #[allow(unused_variables)]
    fn op_cbeb(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 5;
        let v = self.get_e();
        self.set_e(v | test_bit);

        8
    }

    /// SET 5,H
    #[allow(unused_variables)]
    fn op_cbec(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 5;
        let v = self.get_h();
        self.set_h(v | test_bit);

        8
    }

    /// SET 5,L
    #[allow(unused_variables)]
    fn op_cbed(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 5;
        let v = self.get_l();
        self.set_l(v | test_bit);

        8
    }

    /// SET 5,(HL)
    #[allow(unused_variables)]
    fn op_cbee(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 5;
        let v = self.mem_read_u8(self.get_hl());
        self.mem_write_u8(self.get_hl(), v | test_bit);

        16
    }

    /// SET 5,A
    #[allow(unused_variables)]
    fn op_cbef(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 5;
        let v = self.get_a();
        self.set_a(v | test_bit);

        8
    }

    /// SET 6,B
    #[allow(unused_variables)]
    fn op_cbf0(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 6;
        let v = self.get_b();
        self.set_b(v | test_bit);

        8
    }

    /// SET 6,C
    #[allow(unused_variables)]
    fn op_cbf1(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 6;
        let v = self.get_c();
        self.set_c(v | test_bit);

        8
    }

    /// SET 6,D
    #[allow(unused_variables)]
    fn op_cbf2(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 6;
        let v = self.get_d();
        self.set_d(v | test_bit);

        8
    }

    /// SET 6,E
    #[allow(unused_variables)]
    fn op_cbf3(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 6;
        let v = self.get_e();
        self.set_e(v | test_bit);

        8
    }

    /// SET 6,H
    #[allow(unused_variables)]
    fn op_cbf4(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 6;
        let v = self.get_h();
        self.set_h(v | test_bit);

        8
    }

    /// SET 6,L
    #[allow(unused_variables)]
    fn op_cbf5(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 6;
        let v = self.get_l();
        self.set_l(v | test_bit);

        8
    }

    /// SET 6,(HL)
    #[allow(unused_variables)]
    fn op_cbf6(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 6;
        let v = self.mem_read_u8(self.get_hl());
        self.mem_write_u8(self.get_hl(), v | test_bit);

        16
    }

    /// SET 6,A
    #[allow(unused_variables)]
    fn op_cbf7(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 6;
        let v = self.get_a();
        self.set_a(v | test_bit);

        8
    }

    /// SET 7,B
    #[allow(unused_variables)]
    fn op_cbf8(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 7;
        let v = self.get_b();
        self.set_b(v | test_bit);

        8
    }

    /// SET 7,C
    #[allow(unused_variables)]
    fn op_cbf9(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 7;
        let v = self.get_c();
        self.set_c(v | test_bit);

        8
    }

    /// SET 7,D
    #[allow(unused_variables)]
    fn op_cbfa(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 7;
        let v = self.get_d();
        self.set_d(v | test_bit);

        8
    }

    /// SET 7,E
    #[allow(unused_variables)]
    fn op_cbfb(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 7;
        let v = self.get_e();
        self.set_e(v | test_bit);

        8
    }

    /// SET 7,H
    #[allow(unused_variables)]
    fn op_cbfc(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 7;
        let v = self.get_h();
        self.set_h(v | test_bit);

        8
    }

    /// SET 7,L
    #[allow(unused_variables)]
    fn op_cbfd(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 7;
        let v = self.get_l();
        self.set_l(v | test_bit);

        8
    }

    /// SET 7,(HL)
    #[allow(unused_variables)]
    fn op_cbfe(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 7;
        let v = self.mem_read_u8(self.get_hl());
        self.mem_write_u8(self.get_hl(), v | test_bit);

        16
    }

    /// SET 7,A
    #[allow(unused_variables)]
    fn op_cbff(&mut self, op_size: u8) -> u8 {
        let test_bit = 1 << 7;
        let v = self.get_a();
        self.set_a(v | test_bit);

        8
    }

    /// decode the opcode and return the cycle
    pub fn decode(&mut self, opcode: &Opcode) -> u8 {
        let op_size = opcode.bytes;

        match opcode.code {
            0x0000 => self.op_0000(op_size),
            0x0001 => self.op_0001(op_size),
            0x0002 => self.op_0002(op_size),
            0x0003 => self.op_0003(op_size),
            0x0004 => self.op_0004(op_size),
            0x0005 => self.op_0005(op_size),
            0x0006 => self.op_0006(op_size),
            0x0007 => self.op_0007(op_size),
            0x0008 => self.op_0008(op_size),
            0x0009 => self.op_0009(op_size),
            0x000A => self.op_000a(op_size),
            0x000B => self.op_000b(op_size),
            0x000C => self.op_000c(op_size),
            0x000D => self.op_000d(op_size),
            0x000E => self.op_000e(op_size),
            0x000F => self.op_000f(op_size),
            0x0010 => self.op_0010(op_size),
            0x0011 => self.op_0011(op_size),
            0x0012 => self.op_0012(op_size),
            0x0013 => self.op_0013(op_size),
            0x0014 => self.op_0014(op_size),
            0x0015 => self.op_0015(op_size),
            0x0016 => self.op_0016(op_size),
            0x0017 => self.op_0017(op_size),
            0x0018 => self.op_0018(op_size),
            0x0019 => self.op_0019(op_size),
            0x001A => self.op_001a(op_size),
            0x001B => self.op_001b(op_size),
            0x001C => self.op_001c(op_size),
            0x001D => self.op_001d(op_size),
            0x001E => self.op_001e(op_size),
            0x001F => self.op_001f(op_size),
            0x0020 => self.op_0020(op_size),
            0x0021 => self.op_0021(op_size),
            0x0022 => self.op_0022(op_size),
            0x0023 => self.op_0023(op_size),
            0x0024 => self.op_0024(op_size),
            0x0025 => self.op_0025(op_size),
            0x0026 => self.op_0026(op_size),
            0x0027 => self.op_0027(op_size),
            0x0028 => self.op_0028(op_size),
            0x0029 => self.op_0029(op_size),
            0x002A => self.op_002a(op_size),
            0x002B => self.op_002b(op_size),
            0x002C => self.op_002c(op_size),
            0x002D => self.op_002d(op_size),
            0x002E => self.op_002e(op_size),
            0x002F => self.op_002f(op_size),
            0x0030 => self.op_0030(op_size),
            0x0031 => self.op_0031(op_size),
            0x0032 => self.op_0032(op_size),
            0x0033 => self.op_0033(op_size),
            0x0034 => self.op_0034(op_size),
            0x0035 => self.op_0035(op_size),
            0x0036 => self.op_0036(op_size),
            0x0037 => self.op_0037(op_size),
            0x0038 => self.op_0038(op_size),
            0x0039 => self.op_0039(op_size),
            0x003A => self.op_003a(op_size),
            0x003B => self.op_003b(op_size),
            0x003C => self.op_003c(op_size),
            0x003D => self.op_003d(op_size),
            0x003E => self.op_003e(op_size),
            0x003F => self.op_003f(op_size),
            0x0040 => self.op_0040(op_size),
            0x0041 => self.op_0041(op_size),
            0x0042 => self.op_0042(op_size),
            0x0043 => self.op_0043(op_size),
            0x0044 => self.op_0044(op_size),
            0x0045 => self.op_0045(op_size),
            0x0046 => self.op_0046(op_size),
            0x0047 => self.op_0047(op_size),
            0x0048 => self.op_0048(op_size),
            0x0049 => self.op_0049(op_size),
            0x004A => self.op_004a(op_size),
            0x004B => self.op_004b(op_size),
            0x004C => self.op_004c(op_size),
            0x004D => self.op_004d(op_size),
            0x004E => self.op_004e(op_size),
            0x004F => self.op_004f(op_size),
            0x0050 => self.op_0050(op_size),
            0x0051 => self.op_0051(op_size),
            0x0052 => self.op_0052(op_size),
            0x0053 => self.op_0053(op_size),
            0x0054 => self.op_0054(op_size),
            0x0055 => self.op_0055(op_size),
            0x0056 => self.op_0056(op_size),
            0x0057 => self.op_0057(op_size),
            0x0058 => self.op_0058(op_size),
            0x0059 => self.op_0059(op_size),
            0x005A => self.op_005a(op_size),
            0x005B => self.op_005b(op_size),
            0x005C => self.op_005c(op_size),
            0x005D => self.op_005d(op_size),
            0x005E => self.op_005e(op_size),
            0x005F => self.op_005f(op_size),
            0x0060 => self.op_0060(op_size),
            0x0061 => self.op_0061(op_size),
            0x0062 => self.op_0062(op_size),
            0x0063 => self.op_0063(op_size),
            0x0064 => self.op_0064(op_size),
            0x0065 => self.op_0065(op_size),
            0x0066 => self.op_0066(op_size),
            0x0067 => self.op_0067(op_size),
            0x0068 => self.op_0068(op_size),
            0x0069 => self.op_0069(op_size),
            0x006A => self.op_006a(op_size),
            0x006B => self.op_006b(op_size),
            0x006C => self.op_006c(op_size),
            0x006D => self.op_006d(op_size),
            0x006E => self.op_006e(op_size),
            0x006F => self.op_006f(op_size),
            0x0070 => self.op_0070(op_size),
            0x0071 => self.op_0071(op_size),
            0x0072 => self.op_0072(op_size),
            0x0073 => self.op_0073(op_size),
            0x0074 => self.op_0074(op_size),
            0x0075 => self.op_0075(op_size),
            0x0076 => self.op_0076(op_size),
            0x0077 => self.op_0077(op_size),
            0x0078 => self.op_0078(op_size),
            0x0079 => self.op_0079(op_size),
            0x007A => self.op_007a(op_size),
            0x007B => self.op_007b(op_size),
            0x007C => self.op_007c(op_size),
            0x007D => self.op_007d(op_size),
            0x007E => self.op_007e(op_size),
            0x007F => self.op_007f(op_size),
            0x0080 => self.op_0080(op_size),
            0x0081 => self.op_0081(op_size),
            0x0082 => self.op_0082(op_size),
            0x0083 => self.op_0083(op_size),
            0x0084 => self.op_0084(op_size),
            0x0085 => self.op_0085(op_size),
            0x0086 => self.op_0086(op_size),
            0x0087 => self.op_0087(op_size),
            0x0088 => self.op_0088(op_size),
            0x0089 => self.op_0089(op_size),
            0x008A => self.op_008a(op_size),
            0x008B => self.op_008b(op_size),
            0x008C => self.op_008c(op_size),
            0x008D => self.op_008d(op_size),
            0x008E => self.op_008e(op_size),
            0x008F => self.op_008f(op_size),
            0x0090 => self.op_0090(op_size),
            0x0091 => self.op_0091(op_size),
            0x0092 => self.op_0092(op_size),
            0x0093 => self.op_0093(op_size),
            0x0094 => self.op_0094(op_size),
            0x0095 => self.op_0095(op_size),
            0x0096 => self.op_0096(op_size),
            0x0097 => self.op_0097(op_size),
            0x0098 => self.op_0098(op_size),
            0x0099 => self.op_0099(op_size),
            0x009A => self.op_009a(op_size),
            0x009B => self.op_009b(op_size),
            0x009C => self.op_009c(op_size),
            0x009D => self.op_009d(op_size),
            0x009E => self.op_009e(op_size),
            0x009F => self.op_009f(op_size),
            0x00A0 => self.op_00a0(op_size),
            0x00A1 => self.op_00a1(op_size),
            0x00A2 => self.op_00a2(op_size),
            0x00A3 => self.op_00a3(op_size),
            0x00A4 => self.op_00a4(op_size),
            0x00A5 => self.op_00a5(op_size),
            0x00A6 => self.op_00a6(op_size),
            0x00A7 => self.op_00a7(op_size),
            0x00A8 => self.op_00a8(op_size),
            0x00A9 => self.op_00a9(op_size),
            0x00AA => self.op_00aa(op_size),
            0x00AB => self.op_00ab(op_size),
            0x00AC => self.op_00ac(op_size),
            0x00AD => self.op_00ad(op_size),
            0x00AE => self.op_00ae(op_size),
            0x00AF => self.op_00af(op_size),
            0x00B0 => self.op_00b0(op_size),
            0x00B1 => self.op_00b1(op_size),
            0x00B2 => self.op_00b2(op_size),
            0x00B3 => self.op_00b3(op_size),
            0x00B4 => self.op_00b4(op_size),
            0x00B5 => self.op_00b5(op_size),
            0x00B6 => self.op_00b6(op_size),
            0x00B7 => self.op_00b7(op_size),
            0x00B8 => self.op_00b8(op_size),
            0x00B9 => self.op_00b9(op_size),
            0x00BA => self.op_00ba(op_size),
            0x00BB => self.op_00bb(op_size),
            0x00BC => self.op_00bc(op_size),
            0x00BD => self.op_00bd(op_size),
            0x00BE => self.op_00be(op_size),
            0x00BF => self.op_00bf(op_size),
            0x00C0 => self.op_00c0(op_size),
            0x00C1 => self.op_00c1(op_size),
            0x00C2 => self.op_00c2(op_size),
            0x00C3 => self.op_00c3(op_size),
            0x00C4 => self.op_00c4(op_size),
            0x00C5 => self.op_00c5(op_size),
            0x00C6 => self.op_00c6(op_size),
            0x00C7 => self.op_00c7(op_size),
            0x00C8 => self.op_00c8(op_size),
            0x00C9 => self.op_00c9(op_size),
            0x00CA => self.op_00ca(op_size),
            0x00CB => self.op_00cb(op_size),
            0x00CC => self.op_00cc(op_size),
            0x00CD => self.op_00cd(op_size),
            0x00CE => self.op_00ce(op_size),
            0x00CF => self.op_00cf(op_size),
            0x00D0 => self.op_00d0(op_size),
            0x00D1 => self.op_00d1(op_size),
            0x00D2 => self.op_00d2(op_size),
            0x00D4 => self.op_00d4(op_size),
            0x00D5 => self.op_00d5(op_size),
            0x00D6 => self.op_00d6(op_size),
            0x00D7 => self.op_00d7(op_size),
            0x00D8 => self.op_00d8(op_size),
            0x00D9 => self.op_00d9(op_size),
            0x00DA => self.op_00da(op_size),
            0x00DC => self.op_00dc(op_size),
            0x00DE => self.op_00de(op_size),
            0x00DF => self.op_00df(op_size),
            0x00E0 => self.op_00e0(op_size),
            0x00E1 => self.op_00e1(op_size),
            0x00E2 => self.op_00e2(op_size),
            0x00E5 => self.op_00e5(op_size),
            0x00E6 => self.op_00e6(op_size),
            0x00E7 => self.op_00e7(op_size),
            0x00E8 => self.op_00e8(op_size),
            0x00E9 => self.op_00e9(op_size),
            0x00EA => self.op_00ea(op_size),
            0x00EE => self.op_00ee(op_size),
            0x00EF => self.op_00ef(op_size),
            0x00F0 => self.op_00f0(op_size),
            0x00F1 => self.op_00f1(op_size),
            0x00F2 => self.op_00f2(op_size),
            0x00F3 => self.op_00f3(op_size),
            0x00F5 => self.op_00f5(op_size),
            0x00F6 => self.op_00f6(op_size),
            0x00F7 => self.op_00f7(op_size),
            0x00F8 => self.op_00f8(op_size),
            0x00F9 => self.op_00f9(op_size),
            0x00FA => self.op_00fa(op_size),
            0x00FB => self.op_00fb(op_size),
            0x00FE => self.op_00fe(op_size),
            0x00FF => self.op_00ff(op_size),
            0xCB00 => self.op_cb00(op_size),
            0xCB01 => self.op_cb01(op_size),
            0xCB02 => self.op_cb02(op_size),
            0xCB03 => self.op_cb03(op_size),
            0xCB04 => self.op_cb04(op_size),
            0xCB05 => self.op_cb05(op_size),
            0xCB06 => self.op_cb06(op_size),
            0xCB07 => self.op_cb07(op_size),
            0xCB08 => self.op_cb08(op_size),
            0xCB09 => self.op_cb09(op_size),
            0xCB0A => self.op_cb0a(op_size),
            0xCB0B => self.op_cb0b(op_size),
            0xCB0C => self.op_cb0c(op_size),
            0xCB0D => self.op_cb0d(op_size),
            0xCB0E => self.op_cb0e(op_size),
            0xCB0F => self.op_cb0f(op_size),
            0xCB10 => self.op_cb10(op_size),
            0xCB11 => self.op_cb11(op_size),
            0xCB12 => self.op_cb12(op_size),
            0xCB13 => self.op_cb13(op_size),
            0xCB14 => self.op_cb14(op_size),
            0xCB15 => self.op_cb15(op_size),
            0xCB16 => self.op_cb16(op_size),
            0xCB17 => self.op_cb17(op_size),
            0xCB18 => self.op_cb18(op_size),
            0xCB19 => self.op_cb19(op_size),
            0xCB1A => self.op_cb1a(op_size),
            0xCB1B => self.op_cb1b(op_size),
            0xCB1C => self.op_cb1c(op_size),
            0xCB1D => self.op_cb1d(op_size),
            0xCB1E => self.op_cb1e(op_size),
            0xCB1F => self.op_cb1f(op_size),
            0xCB20 => self.op_cb20(op_size),
            0xCB21 => self.op_cb21(op_size),
            0xCB22 => self.op_cb22(op_size),
            0xCB23 => self.op_cb23(op_size),
            0xCB24 => self.op_cb24(op_size),
            0xCB25 => self.op_cb25(op_size),
            0xCB26 => self.op_cb26(op_size),
            0xCB27 => self.op_cb27(op_size),
            0xCB28 => self.op_cb28(op_size),
            0xCB29 => self.op_cb29(op_size),
            0xCB2A => self.op_cb2a(op_size),
            0xCB2B => self.op_cb2b(op_size),
            0xCB2C => self.op_cb2c(op_size),
            0xCB2D => self.op_cb2d(op_size),
            0xCB2E => self.op_cb2e(op_size),
            0xCB2F => self.op_cb2f(op_size),
            0xCB30 => self.op_cb30(op_size),
            0xCB31 => self.op_cb31(op_size),
            0xCB32 => self.op_cb32(op_size),
            0xCB33 => self.op_cb33(op_size),
            0xCB34 => self.op_cb34(op_size),
            0xCB35 => self.op_cb35(op_size),
            0xCB36 => self.op_cb36(op_size),
            0xCB37 => self.op_cb37(op_size),
            0xCB38 => self.op_cb38(op_size),
            0xCB39 => self.op_cb39(op_size),
            0xCB3A => self.op_cb3a(op_size),
            0xCB3B => self.op_cb3b(op_size),
            0xCB3C => self.op_cb3c(op_size),
            0xCB3D => self.op_cb3d(op_size),
            0xCB3E => self.op_cb3e(op_size),
            0xCB3F => self.op_cb3f(op_size),
            0xCB40 => self.op_cb40(op_size),
            0xCB41 => self.op_cb41(op_size),
            0xCB42 => self.op_cb42(op_size),
            0xCB43 => self.op_cb43(op_size),
            0xCB44 => self.op_cb44(op_size),
            0xCB45 => self.op_cb45(op_size),
            0xCB46 => self.op_cb46(op_size),
            0xCB47 => self.op_cb47(op_size),
            0xCB48 => self.op_cb48(op_size),
            0xCB49 => self.op_cb49(op_size),
            0xCB4A => self.op_cb4a(op_size),
            0xCB4B => self.op_cb4b(op_size),
            0xCB4C => self.op_cb4c(op_size),
            0xCB4D => self.op_cb4d(op_size),
            0xCB4E => self.op_cb4e(op_size),
            0xCB4F => self.op_cb4f(op_size),
            0xCB50 => self.op_cb50(op_size),
            0xCB51 => self.op_cb51(op_size),
            0xCB52 => self.op_cb52(op_size),
            0xCB53 => self.op_cb53(op_size),
            0xCB54 => self.op_cb54(op_size),
            0xCB55 => self.op_cb55(op_size),
            0xCB56 => self.op_cb56(op_size),
            0xCB57 => self.op_cb57(op_size),
            0xCB58 => self.op_cb58(op_size),
            0xCB59 => self.op_cb59(op_size),
            0xCB5A => self.op_cb5a(op_size),
            0xCB5B => self.op_cb5b(op_size),
            0xCB5C => self.op_cb5c(op_size),
            0xCB5D => self.op_cb5d(op_size),
            0xCB5E => self.op_cb5e(op_size),
            0xCB5F => self.op_cb5f(op_size),
            0xCB60 => self.op_cb60(op_size),
            0xCB61 => self.op_cb61(op_size),
            0xCB62 => self.op_cb62(op_size),
            0xCB63 => self.op_cb63(op_size),
            0xCB64 => self.op_cb64(op_size),
            0xCB65 => self.op_cb65(op_size),
            0xCB66 => self.op_cb66(op_size),
            0xCB67 => self.op_cb67(op_size),
            0xCB68 => self.op_cb68(op_size),
            0xCB69 => self.op_cb69(op_size),
            0xCB6A => self.op_cb6a(op_size),
            0xCB6B => self.op_cb6b(op_size),
            0xCB6C => self.op_cb6c(op_size),
            0xCB6D => self.op_cb6d(op_size),
            0xCB6E => self.op_cb6e(op_size),
            0xCB6F => self.op_cb6f(op_size),
            0xCB70 => self.op_cb70(op_size),
            0xCB71 => self.op_cb71(op_size),
            0xCB72 => self.op_cb72(op_size),
            0xCB73 => self.op_cb73(op_size),
            0xCB74 => self.op_cb74(op_size),
            0xCB75 => self.op_cb75(op_size),
            0xCB76 => self.op_cb76(op_size),
            0xCB77 => self.op_cb77(op_size),
            0xCB78 => self.op_cb78(op_size),
            0xCB79 => self.op_cb79(op_size),
            0xCB7A => self.op_cb7a(op_size),
            0xCB7B => self.op_cb7b(op_size),
            0xCB7C => self.op_cb7c(op_size),
            0xCB7D => self.op_cb7d(op_size),
            0xCB7E => self.op_cb7e(op_size),
            0xCB7F => self.op_cb7f(op_size),
            0xCB80 => self.op_cb80(op_size),
            0xCB81 => self.op_cb81(op_size),
            0xCB82 => self.op_cb82(op_size),
            0xCB83 => self.op_cb83(op_size),
            0xCB84 => self.op_cb84(op_size),
            0xCB85 => self.op_cb85(op_size),
            0xCB86 => self.op_cb86(op_size),
            0xCB87 => self.op_cb87(op_size),
            0xCB88 => self.op_cb88(op_size),
            0xCB89 => self.op_cb89(op_size),
            0xCB8A => self.op_cb8a(op_size),
            0xCB8B => self.op_cb8b(op_size),
            0xCB8C => self.op_cb8c(op_size),
            0xCB8D => self.op_cb8d(op_size),
            0xCB8E => self.op_cb8e(op_size),
            0xCB8F => self.op_cb8f(op_size),
            0xCB90 => self.op_cb90(op_size),
            0xCB91 => self.op_cb91(op_size),
            0xCB92 => self.op_cb92(op_size),
            0xCB93 => self.op_cb93(op_size),
            0xCB94 => self.op_cb94(op_size),
            0xCB95 => self.op_cb95(op_size),
            0xCB96 => self.op_cb96(op_size),
            0xCB97 => self.op_cb97(op_size),
            0xCB98 => self.op_cb98(op_size),
            0xCB99 => self.op_cb99(op_size),
            0xCB9A => self.op_cb9a(op_size),
            0xCB9B => self.op_cb9b(op_size),
            0xCB9C => self.op_cb9c(op_size),
            0xCB9D => self.op_cb9d(op_size),
            0xCB9E => self.op_cb9e(op_size),
            0xCB9F => self.op_cb9f(op_size),
            0xCBA0 => self.op_cba0(op_size),
            0xCBA1 => self.op_cba1(op_size),
            0xCBA2 => self.op_cba2(op_size),
            0xCBA3 => self.op_cba3(op_size),
            0xCBA4 => self.op_cba4(op_size),
            0xCBA5 => self.op_cba5(op_size),
            0xCBA6 => self.op_cba6(op_size),
            0xCBA7 => self.op_cba7(op_size),
            0xCBA8 => self.op_cba8(op_size),
            0xCBA9 => self.op_cba9(op_size),
            0xCBAA => self.op_cbaa(op_size),
            0xCBAB => self.op_cbab(op_size),
            0xCBAC => self.op_cbac(op_size),
            0xCBAD => self.op_cbad(op_size),
            0xCBAE => self.op_cbae(op_size),
            0xCBAF => self.op_cbaf(op_size),
            0xCBB0 => self.op_cbb0(op_size),
            0xCBB1 => self.op_cbb1(op_size),
            0xCBB2 => self.op_cbb2(op_size),
            0xCBB3 => self.op_cbb3(op_size),
            0xCBB4 => self.op_cbb4(op_size),
            0xCBB5 => self.op_cbb5(op_size),
            0xCBB6 => self.op_cbb6(op_size),
            0xCBB7 => self.op_cbb7(op_size),
            0xCBB8 => self.op_cbb8(op_size),
            0xCBB9 => self.op_cbb9(op_size),
            0xCBBA => self.op_cbba(op_size),
            0xCBBB => self.op_cbbb(op_size),
            0xCBBC => self.op_cbbc(op_size),
            0xCBBD => self.op_cbbd(op_size),
            0xCBBE => self.op_cbbe(op_size),
            0xCBBF => self.op_cbbf(op_size),
            0xCBC0 => self.op_cbc0(op_size),
            0xCBC1 => self.op_cbc1(op_size),
            0xCBC2 => self.op_cbc2(op_size),
            0xCBC3 => self.op_cbc3(op_size),
            0xCBC4 => self.op_cbc4(op_size),
            0xCBC5 => self.op_cbc5(op_size),
            0xCBC6 => self.op_cbc6(op_size),
            0xCBC7 => self.op_cbc7(op_size),
            0xCBC8 => self.op_cbc8(op_size),
            0xCBC9 => self.op_cbc9(op_size),
            0xCBCA => self.op_cbca(op_size),
            0xCBCB => self.op_cbcb(op_size),
            0xCBCC => self.op_cbcc(op_size),
            0xCBCD => self.op_cbcd(op_size),
            0xCBCE => self.op_cbce(op_size),
            0xCBCF => self.op_cbcf(op_size),
            0xCBD0 => self.op_cbd0(op_size),
            0xCBD1 => self.op_cbd1(op_size),
            0xCBD2 => self.op_cbd2(op_size),
            0xCBD3 => self.op_cbd3(op_size),
            0xCBD4 => self.op_cbd4(op_size),
            0xCBD5 => self.op_cbd5(op_size),
            0xCBD6 => self.op_cbd6(op_size),
            0xCBD7 => self.op_cbd7(op_size),
            0xCBD8 => self.op_cbd8(op_size),
            0xCBD9 => self.op_cbd9(op_size),
            0xCBDA => self.op_cbda(op_size),
            0xCBDB => self.op_cbdb(op_size),
            0xCBDC => self.op_cbdc(op_size),
            0xCBDD => self.op_cbdd(op_size),
            0xCBDE => self.op_cbde(op_size),
            0xCBDF => self.op_cbdf(op_size),
            0xCBE0 => self.op_cbe0(op_size),
            0xCBE1 => self.op_cbe1(op_size),
            0xCBE2 => self.op_cbe2(op_size),
            0xCBE3 => self.op_cbe3(op_size),
            0xCBE4 => self.op_cbe4(op_size),
            0xCBE5 => self.op_cbe5(op_size),
            0xCBE6 => self.op_cbe6(op_size),
            0xCBE7 => self.op_cbe7(op_size),
            0xCBE8 => self.op_cbe8(op_size),
            0xCBE9 => self.op_cbe9(op_size),
            0xCBEA => self.op_cbea(op_size),
            0xCBEB => self.op_cbeb(op_size),
            0xCBEC => self.op_cbec(op_size),
            0xCBED => self.op_cbed(op_size),
            0xCBEE => self.op_cbee(op_size),
            0xCBEF => self.op_cbef(op_size),
            0xCBF0 => self.op_cbf0(op_size),
            0xCBF1 => self.op_cbf1(op_size),
            0xCBF2 => self.op_cbf2(op_size),
            0xCBF3 => self.op_cbf3(op_size),
            0xCBF4 => self.op_cbf4(op_size),
            0xCBF5 => self.op_cbf5(op_size),
            0xCBF6 => self.op_cbf6(op_size),
            0xCBF7 => self.op_cbf7(op_size),
            0xCBF8 => self.op_cbf8(op_size),
            0xCBF9 => self.op_cbf9(op_size),
            0xCBFA => self.op_cbfa(op_size),
            0xCBFB => self.op_cbfb(op_size),
            0xCBFC => self.op_cbfc(op_size),
            0xCBFD => self.op_cbfd(op_size),
            0xCBFE => self.op_cbfe(op_size),
            0xCBFF => self.op_cbff(op_size),
            _ => panic!("Unable to decode opcode: {}", opcode.code),
        }
    }
}
