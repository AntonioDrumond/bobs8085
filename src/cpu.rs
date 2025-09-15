mod instructions;
mod changes;

use crate::bus::Bus;
use crate::changes::*;

#[derive(Default, Debug, Clone)]
#[allow(dead_code, unused_variables, clippy::upper_case_acronyms)]
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
    #[rustfmt::skip]
    pub fn print_state(&self) {
        println!("📥A  => {:02X} - {:08b}    |    🚩S  => {}", self.a, self.a, self.s);
        println!("📥B  => {:02X} - {:08b}    |    🚩Z  => {}", self.b, self.b, self.z);
        println!("📥C  => {:02X} - {:08b}    |    🚩AC => {}", self.c, self.c, self.ac);
        println!("📥D  => {:02X} - {:08b}    |    🚩P  => {}", self.d, self.d, self.p);
        println!("📥E  => {:02X} - {:08b}    |    🚩CY => {}", self.e, self.e, self.cy);
        println!("📥H  => {:02X} - {:08b}", self.h, self.h);
        println!("📥L  => {:02X} - {:08b}", self.l, self.l);
        println!("📥SP => {:04X} - {:08b}", self.sp, self.sp);
        println!("📥PC => {:04X} - {:08b}", self.pc, self.pc);
    }

    pub fn set_pc(&mut self, val: u16) {
        self.pc = val;
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    fn fetch8(&mut self, bus: &Bus) -> u8 {
        self.pc += 1;
        bus.mem_get8(self.pc - 1)
    }

    fn fetch16(&mut self, bus: &Bus) -> u16 {
        self.pc += 2;
        bus.mem_get16_reverse(self.pc - 2)
    }

    fn get_reg(&self, bus: &Bus, target: u8) -> u8 {
        match target {
            0 => self.b,
            1 => self.c,
            2 => self.d,
            3 => self.e,
            4 => self.h,
            5 => self.l,
            6 => bus.mem_get8(self.get_reg_pair(2)),
            7 => self.a,
            _ => panic!("Unknown target"),
        }
    }

    pub fn set_reg(&mut self, bus: &mut Bus, target: u8, value: u8) {
        match target {
            0 => self.b = value,
            1 => self.c = value,
            2 => self.d = value,
            3 => self.e = value,
            4 => self.h = value,
            5 => self.l = value,
            6 => bus.mem_set8(self.get_reg_pair(2), value),
            7 => self.a = value,
            _ => panic!("Unknown target"),
        }
    }

    fn set_reg_pair(&mut self, target: u8, value: u16) {
        let l = (value >> 8) as u8;
        let r = value as u8;
        match target {
            0 => {
                self.b = l;
                self.c = r;
            }
            1 => {
                self.d = l;
                self.e = r;
            }
            2 => {
                self.h = l;
                self.l = r;
            }
            3 => self.sp = value,
            _ => panic!("Unknown target"),
        }
    }

    fn get_reg_pair(&self, target: u8) -> u16 {
        let mut value: u16;
        match target {
            0 => {
                value = (self.b as u16) << 8;
                value |= self.c as u16;
            }
            1 => {
                value = (self.d as u16) << 8;
                value |= self.e as u16;
            }
            2 => {
                value = (self.h as u16) << 8;
                value |= self.l as u16;
            }
            3 => {
                value = self.sp;
            }
            _ => panic!("Unknown target"),
        }
        value
    }

    fn update_s(&mut self, value: u8) {
        self.s = value > 0x7F;
    }

    fn update_z(&mut self, value: u8) {
        self.z = value == 0;
    }

    fn update_p(&mut self, value: u8) {
        self.p = true;
        let mut i: u16 = 1;
        while i < 0x100 {
            if value & i as u8 > 0 {
                self.p = !self.p;
            }
            i <<= 1;
        }
    }

    fn diff (&self, other:CPU) -> vec<CPU_E> {
        if self.a != other.a {
            CPU_E.registers.push(A, self.a);
        }
        if self.b != other.b {
            CPU_E.registers.push(B, self.b);
        }
        if self.c != other.c {
            CPU_E.registers.push(C, self.c);
        }
        if self.d != other.d {
            CPU_E.registers.push(D, self.d);
        }
        if self.e != other.e {
            CPU_E.registers.push(E, self.e);
        }
        if self.h != other.h {
            CPU_E.registers.push(H, self.h);
        }
        if self.l != other.l {
            CPU_E.registers.push(L, self.l);
        }

        if self.z != other.z {
            CPU_E.
        }
    }

    pub fn execute(&mut self, bus: &mut Bus) -> bool {
        if self.pc >= 0xD000 { return false; }
        let inst = bus.mem_get8(self.pc);
        self.pc += 1;
        match inst {
            0x76 => return false,
            0x40..=0x7F => self.mov(bus, inst),
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E => self.mvi(bus, inst),
            0x01 | 0x11 | 0x21 => self.lxi(bus, inst),
            0x02 | 0x12 => self.stax(bus, inst),
            0x0a | 0x1a => self.ldax(bus, inst),
            0x32 => self.sta(bus),
            0x3A => self.lda(bus),
            0x22 => self.shld(bus),
            0x2A => self.lhld(bus),
            0xEB => self.xchg(),
            0xC5 | 0xD5 | 0xE5 | 0xF5 => self.push(inst, bus),
            0xC1 | 0xD1 | 0xE1 | 0xF1 => self.pop(inst, bus),
            0xE3 => self.xthl(bus),
            0xF9 => self.sphl(),
            0x33 => self.inx(inst),
            0x3B => self.dcx(inst),
            0xC3 | 0xDA | 0xD2 | 0xCA | 0xC2 | 0xF2 | 0xFA | 0xEA | 0xE2 => self.jump(inst, bus),
            0xE9 => self.pchl(),
            0xCD | 0xDC | 0xD4 | 0xCC | 0xC4 | 0xF4 | 0xFC | 0xEC | 0xE4 => self.call(inst, bus),
            0xC9 | 0xD8 | 0xD0 | 0xC8 | 0xC0 | 0xF0 | 0xF8 | 0xE8 | 0xE0 => self.ret(inst, bus),
            0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xf7 | 0xFF => self.rst(inst, bus),
            0xDB => self.io_in(bus),
            0xD3 => self.io_out(bus),
            0x04 | 0x0C | 0x14 | 0x1C | 0x24 | 0x2C | 0x34 | 0x3C => self.inr(bus, inst),
            0x05 | 0x0D | 0x15 | 0x1D | 0x25 | 0x2D | 0x35 | 0x3D => self.dcr(bus, inst),
            0x03 | 0x13 | 0x23 => self.inx(inst),
            0x0B | 0x1B | 0x2B => self.dcx(inst),
            0x80..=0x87 => self.add(bus, inst),
            0x88..=0x8F => self.adc(bus, inst),
            0xC6 => self.adi(bus),
            0xCE => self.aci(bus),
            0x09 | 0x19 | 0x29 | 0x39 => self.dad(inst),
            0x90..=0x9F => self.sub(bus, inst),
            0xD6 => self.sui(bus),
            0xDE => self.sbi(bus),
            0xA0..=0xA7 => self.ana(bus, inst),
            0xA8..=0xAF => self.xra(bus, inst),
            0xB0..=0xB7 => self.ora(bus, inst),
            0xB8..=0xBF => self.cmp(bus, inst),
            0xE6 => self.cpi(bus, inst),
            0xEE => self.xri(bus, inst),
            0xF6 => self.ori(bus, inst),
            0xFE => self.cpi(bus, inst),
            0x07 | 0x0F | 0x17 | 0x1F => self.rotate(inst),
            0x2F => self.cma(),
            0x37 => self.stc(),
            0x3F => self.cmc(),
            0x27 => self.daa(),
            0xFB => todo!("EI"),
            0xF3 => todo!("DI"),
            0x00 => self.nop(),
            0x20 => todo!("RIM"),
            0x30 => todo!("SIM"),
            _ => panic!("Instrução não identificada: {inst:02X}"),
        };
        true
    }
}
