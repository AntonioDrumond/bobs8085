#[allow(dead_code)]
enum Register {
    A, B, C, D, E, H, L, M,
}

#[derive(Default, Debug)]
#[allow(dead_code, unused_variables)]
pub struct CPU {
    a: u8, // Accumulator
    b: u8, // Pair BC
    c: u8,
    d: u8, // Pair DE
    e: u8,
    h: u8, // Pair HL
    l: u8,
    sp: u16, // Stack Pointer
    pc: u16, // Program Counter
    // Flags:
    s: bool,  // Sign
    z: bool,  // Zero
    ac: bool, // Auxiliary Carry
    p: bool,  // Parity
    cy: bool, // Carry
}

#[allow(dead_code, unused_variables)]
impl CPU {

    fn get_reg(&self, reg: Register) -> u8 {
        match reg {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::H => self.h,
            Register::L => self.l,
            Register::M => todo!("Memory access"),
            // _ => unreachable!("No register found"),
        }
    }

    fn set_reg(&mut self, reg: Register, value: u8) {
        match reg {
            Register::A => self.a = value,
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::D => self.d = value,
            Register::E => self.e = value,
            Register::H => self.h = value,
            Register::L => self.l = value,
            Register::M => todo!("Memory access"),
            // _ => unreachable!("No register found"),
        }
    }

    fn reg_from_bits(bits: u8) -> Register {
        let tail = bits & 0x07;
        match tail {
            0b111 => Register::A,
            0b000 => Register::B,
            0b001 => Register::C,
            0b010 => Register::D,
            0b011 => Register::E,
            0b100 => Register::H,
            0b101 => Register::L,
            0b110 => Register::M,
            _ => unreachable!("No possible register"),
        }
    }

    pub fn execute(&mut self, inst: u8) -> u32 {
        match inst {
            0x76 => todo!("HLT"),
            0x40..=0x7F => todo!("MOVs"),
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E => todo!("MVIs"),
            0x01 | 0x11 | 0x21 => todo!("LXIs"),
            0x02 | 0x12 => todo!("STAXs"),
            0x0a | 0x1a => todo!("LDAXs"),
            0x32 => todo!("STA"),
            0x3A => todo!("LDA"),
            0x22 => todo!("SHLD"),
            0x2A => todo!("LHLD"),
            0xEB => todo!("XCHG"),
            0xC5 | 0xD5 | 0xE5 | 0xF5 => todo!("PUSH Pairs"),
            0xC1 | 0xD1 | 0xE1 | 0xF1 => todo!("POP Pairs"),
            0xE3 => todo!("XTHL"),
            0xF9 => todo!("SPHL"),
            0x33 => todo!("INX SP"),
            0x3B => todo!("DCX SP"),
            0xC3 | 0xDA | 0xD2 | 0xCA | 0xC2 | 0xF2 | 0xFA | 0xEA | 0xE2 => todo!("Jumps"),
            0xE9 => todo!("PCHL"),
            0xCD | 0xDC | 0xD4 | 0xCC | 0xC4 | 0xF4 | 0xFC | 0xEC | 0xE4 => todo!("Calls"),
            0xC9 | 0xD8 | 0xD0 | 0xC8 | 0xC0 | 0xF0 | 0xF8 | 0xE8 | 0xE0 => todo!("RETs"),
            0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xf7 | 0xFF => todo!("RSTs"),
            0xDB => todo!("IN"),
            0xD3 => todo!("OUT"),
            0x04 | 0x0C | 0x14 | 0x1C | 0x24 | 0x2C | 0x34 | 0x3C => todo!("Increments"),
            0x05 | 0x0D | 0x15 | 0x1D | 0x25 | 0x2D | 0x35 | 0x3D => todo!("Decrements"),
            0x03 | 0x13 | 0x23 => todo!("Increment pairs"),
            0x0B | 0x1B | 0x2B => todo!("Increment pairs"),
            0x80..=0x8F => todo!("ADDs"), // Conferir datasheet com prof -> ADD M
            0xC6 => todo!("ADI sem carry"),
            0xCE => todo!("ACI (ADI com carry)"),
            0x09 | 0x19 | 0x29 | 0x39 => todo!("DADs"),
            0x90..=0x9F => todo!("SUBs"),
            0xD6 => todo!("SUI"),
            0xDE => todo!("SBI"),
            0xA0..=0xA7 => todo!("ANDs"),
            0xA8..=0xAF => todo!("XORs"),
            0xB0..=0xB7 => todo!("ORs"),
            0xB8..=0xBF => todo!("CMPs"),
            0xE6 => todo!("ANI"),
            0xEE => todo!("XRI"),
            0xF6 => todo!("ORI"),
            0xFE => todo!("CPI"),
            0x07 | 0x0F | 0x17 | 0x1F => todo!("Rotates"),
            0x2F => todo!("CMA"),
            0x37 => todo!("STC"),
            0x3F => todo!("CMC"),
            0x27 => todo!("DAA"),
            0xFB => todo!("EI"),
            0xF3 => todo!("DI"),
            0x00 => todo!("NOP"),
            0x20 => todo!("RIM"),
            0x30 => todo!("SIM"),
            _ => todo!("Instrução não identificada :c"),
        }

    }

    pub fn mov(&mut self, inst: u8) {
        let source = inst & 0x07;
        let dest = (inst >> 3) & 0x07;
        self.set_reg(
            CPU::reg_from_bits(dest),
            self.get_reg( CPU::reg_from_bits(source) )
        );
    }

}
