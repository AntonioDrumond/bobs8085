use super::CPU;
use crate::bus::Bus;

#[allow(dead_code, unused_variables)]
impl CPU {
    pub(super) fn mov(&mut self, bus: &mut Bus, inst: u8) {
        let s = inst & 0x07;
        let d = (inst >> 3) & 0x07;
        let value = self.get_reg(bus, s);
        self.set_reg(bus, d, value);
    }

    pub(super) fn mvi(&mut self, bus: &mut Bus, inst: u8) {
        let d = (inst >> 3) & 0x07;
        let value = self.fetch8(bus);
        self.set_reg(bus, d, value);
    }

    pub(super) fn lxi(&mut self, bus: &mut Bus, inst: u8) {
        let d = (inst >> 4) & 0x03;
        let value = self.fetch16(bus);
        self.set_reg_pair(d, value);
    }

    pub(super) fn stax(&mut self, bus: &mut Bus, inst: u8) {
        let s = (inst >> 4) & 1;
        let addr: u16 = if s == 0 {
            (self.b as u16) << 8 | self.c as u16
        } else {
            (self.d as u16) << 8 | self.c as u16
        };
        bus.mem_set8(addr, self.a);
    }

    pub(super) fn ldax(&mut self, bus: &mut Bus, inst: u8) {
        let s = (inst >> 4) & 1;
        let addr: u16 = if s == 0 {
            (self.b as u16) << 8 | self.c as u16
        } else {
            (self.d as u16) << 8 | self.c as u16
        };
        let value = bus.mem_get8(addr);
        self.a = value;
    }

    pub(super) fn sta(&mut self, bus: &mut Bus) {
        let addr = self.fetch16(bus);
        bus.mem_set8(addr, self.a);
    }

    pub(super) fn lda(&mut self, bus: &mut Bus) {
        let addr = self.fetch16(bus);
        self.a = bus.mem_get8(addr);
    }

    pub(super) fn shld(&mut self, bus: &mut Bus) {
        let addr = self.fetch16(bus);
        let value = (self.h as u16) << 8 | self.l as u16;
        bus.mem_set16_reverse(addr, value);
    }

    pub(super) fn lhld(&mut self, bus: &mut Bus) {
        let addr = self.fetch16(bus);
        let value = bus.mem_get16_reverse(addr);
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }

    pub(super) fn xchg(&mut self) {
        let h = self.h;
        let l = self.l;
        self.h = self.d;
        self.l = self.d;
        self.d = h;
        self.e = l;
    }

    pub(super) fn inr(&mut self, bus: &mut Bus, inst: u8) {
        let d = (inst >> 3) & 0x07;
        let old = self.get_reg(bus, d);
        let new = old.wrapping_add(1);
        self.set_reg(bus, d, new);
        if d != 0x7 {
            self.update_p(new);
            self.update_s(new);
            self.update_z(new);
            if old == 0xFF {
                self.ac = true;
                self.cy = true;
            }
            else {
                self.ac = false;
                self.cy = false;
            }
        }
    }

    pub(super) fn dcr(&mut self, bus: &mut Bus, inst: u8) {
        let d = (inst >> 3) & 0x07;
        let old = self.get_reg(bus, d);
        let new = old.wrapping_sub(1);
        self.set_reg(bus, d, new);
        if d != 0x7 {
            self.update_p(new);
            self.update_s(new);
            self.update_z(new);
            if old == 0x00 {
                self.ac = true;
                self.cy = true;
            }
            else {
                self.ac = false;
                self.cy = false;
            }
        }
    }

    pub(super) fn inx(&mut self, inst: u8) { // Does NOT alter flags
        let d = (inst >> 4) & 0x03;
        let value = self.get_reg_pair(d);
        self.set_reg_pair(d, value.wrapping_add(1));
    }

    pub(super) fn dcx(&mut self, inst: u8) { // Does NOT alter flags
        let d = (inst >> 4) & 0x03;
        let value = self.get_reg_pair(d);
        self.set_reg_pair(d, value.wrapping_sub(1));
    }

    pub(super) fn rotate(&mut self, inst: u8) {
        let which = inst >> 3;
        match which {
            0 => {
                // RLC
                let carry = self.a & 0x80 == 0x80;
                self.a <<= 1;
                if carry {
                    self.cy = true;
                    self.a |= 0x01;
                }
            }
            1 => {
                // RRC
                let carry = self.a & 0x01 == 0x01;
                self.a >>= 1;
                if carry {
                    self.cy = true;
                    self.a |= 0x80;
                }
            }
            2 => {
                // RAL
                let cyout = self.cy;
                let cyin = self.a & 0x80 == 0x80;
                self.a <<= 1;
                if cyout {
                    self.a |= 0x01;
                }
                self.cy = cyin;
            }
            3 => {
                // RAR
                let cyout = self.cy;
                let cyin = self.a & 0x01 == 0x01;
                self.a >>= 1;
                if cyout {
                    self.a |= 0x80;
                }
                self.cy = cyin;
            }
            _ => panic!("Instrução não encontrada: {inst:X} / {inst:b}"),
        }
    }

    pub(super) fn add(&mut self, bus: &mut Bus, inst: u8) {
        let s = inst & 0x07;
        let value = self.get_reg(bus, s);
        let prev_a = self.a;
        self.a = prev_a.wrapping_add(value);
        self.update_s(self.a);
        self.update_z(self.a);
        self.update_p(self.a);
        self.ac = (self.a & 0x0F) < (prev_a & 0x0F);
        self.cy = self.a < prev_a;
    }

    pub(super) fn adc(&mut self, bus: &mut Bus, inst: u8) {
        let s = inst & 0x07;
        let value = self.get_reg(bus, s);
        let prev_a = self.a;
        //self.a = prev_a + value + self.cy as u8;
        self.a = self.a.wrapping_add(value);
        self.a = self.a.wrapping_add(self.cy as u8);
        self.update_s(self.a);
        self.update_z(self.a);
        self.update_p(self.a);
        self.ac = (self.a & 0x0F) < (prev_a & 0x0F);
        self.cy = self.a < prev_a;
    }

    pub(super) fn adi(&mut self, bus: &mut Bus) {
        let value = self.fetch8(bus);
        let prev_a = self.a;
        self.a = prev_a.wrapping_add(value);
        self.update_s(self.a);
        self.update_z(self.a);
        self.update_p(self.a);
        self.ac = (self.a & 0x0F) < (prev_a & 0x0F);
        self.cy = self.a < prev_a;
    }

    pub(super) fn aci(&mut self, bus: &mut Bus) {
        let value = self.fetch8(bus);
        let prev_a = self.a;
        //self.a = prev_a + value + self.cy as u8;
        self.a = prev_a.wrapping_add(value);
        self.a = self.a.wrapping_add(self.cy as u8);
        self.update_s(self.a);
        self.update_z(self.a);
        self.update_p(self.a);
        self.ac = (self.a & 0x0F) < (prev_a & 0x0F);
        self.cy = self.a < prev_a;
    }

    pub(super) fn dad(&mut self, inst: u8) {
        let s = (inst >> 4) & 0x03;
        let value = self.get_reg_pair(s);
        let prev_hl = (self.h as u16) << 8 | self.l as u16;
        let cur_hl = prev_hl.wrapping_add(value);
        self.h = (value >> 8) as u8;
        self.l = value as u8;
        self.cy = cur_hl < prev_hl;
    }

    pub(super) fn sub(&mut self, bus: &mut Bus, inst: u8) {
        let s = inst & 0x03;
        let value = self.get_reg(bus, s);
        let prev_a = self.a;
        // self.a = prev_a - value;
        self.a = prev_a.wrapping_sub(value);
        self.update_s(self.a);
        self.update_z(self.a);
        self.update_p(self.a);
        self.ac = (value & 0x0F) > (prev_a & 0x0F);
        self.cy = value > prev_a;
    }

    pub(super) fn sbb(&mut self, bus: &mut Bus, inst: u8) {
        let s = inst & 0x03;
        // let value = self.get_reg(bus, s) + self.cy as u8;
        let value = self.get_reg(bus, s).wrapping_add(self.cy as u8);
        let prev_a = self.a;
        // self.a = prev_a - value;
        self.a = prev_a.wrapping_sub(value);
        self.update_s(self.a);
        self.update_z(self.a);
        self.update_p(self.a);
        self.ac = (value & 0x0F) > (prev_a & 0x0F);
        self.cy = value > prev_a;
    }

    pub(super) fn sui(&mut self, bus: &mut Bus) {
        let value = self.fetch8(bus);
        let prev_a = self.a;
        // self.a = prev_a - value;
        self.a = prev_a.wrapping_sub(value);
        self.update_s(self.a);
        self.update_z(self.a);
        self.update_p(self.a);
        self.ac = (value & 0x0F) > (prev_a & 0x0F);
        self.cy = value > prev_a;
    }

    pub(super) fn sbi(&mut self, bus: &mut Bus) {
        let value = self.fetch8(bus) + self.cy as u8;
        let prev_a = self.a;
        // self.a = prev_a - value;
        self.a = prev_a.wrapping_sub(value);
        self.update_s(self.a);
        self.update_z(self.a);
        self.update_p(self.a);
        self.ac = (value & 0x0F) > (prev_a & 0x0F);
        self.cy = value > prev_a;
    }

    pub(super) fn daa(&mut self) {

        let prev_a = self.a;
        let lo = prev_a & 0xF;

        if lo > 0x9 || self.ac {
            self.a = self.a.wrapping_add(0x6);
        }

        self.ac = (self.a & 0x0F) < (prev_a & 0x0F);
        self.cy = self.cy || (self.a < prev_a);

        let hi = (self.a >> 4) & 0xF;

        if hi > 0x9 || self.cy {
            self.a = self.a.wrapping_add(0x60);
            self.cy = true;
        }

        self.update_s(self.a);
        self.update_z(self.a);
        self.update_p(self.a);
    }

    pub(super) fn push(&mut self, inst: u8, bus: &mut Bus) {
        if self.sp <= 0xC000 {
            self.sp = 0xD000;
        }
        let which = (inst >> 4) & 0x03;
        match which {
            0 => {
                // BC
                self.sp -= 1;
                bus.mem_set8(self.sp, self.b);
                self.sp -= 1;
                bus.mem_set8(self.sp, self.c);
            }
            1 => {
                // DE
                self.sp -= 1;
                bus.mem_set8(self.sp, self.d);
                self.sp -= 1;
                bus.mem_set8(self.sp, self.e);
            }
            2 => {
                // HL
                self.sp -= 1;
                bus.mem_set8(self.sp, self.h);
                self.sp -= 1;
                bus.mem_set8(self.sp, self.l);
            }
            3 => {
                // PSW - AF
                self.sp -= 1;
                bus.mem_set8(self.sp, self.a);
                self.sp -= 1;
                let mut flags: u8 = 0;
                if self.cy {
                    flags += 1;
                }
                if self.p {
                    flags += 4;
                }
                if self.ac {
                    flags += 16;
                }
                if self.z {
                    flags += 64;
                }
                if self.s {
                    flags += 128;
                }
                bus.mem_set8(self.sp, flags);
            }
            _ => panic!("Instrução não encontrada: {inst:X} / {inst:b}"),
        }
    }

    pub(super) fn pop(&mut self, inst: u8, bus: &mut Bus) {
        if self.sp == 0xCFFF {
            self.sp = 0x0000;
        }
        let which = (inst >> 4) & 0x03;
        match which {
            0 => {
                // BC
                self.c = bus.mem_get8(self.sp);
                self.sp += 1;
                self.b = bus.mem_get8(self.sp);
                self.sp += 1;
            }
            1 => {
                // DE
                self.e = bus.mem_get8(self.sp);
                self.sp += 1;
                self.d = bus.mem_get8(self.sp);
                self.sp += 1;
            }
            2 => {
                // HL
                self.l = bus.mem_get8(self.sp);
                self.sp += 1;
                self.h = bus.mem_get8(self.sp);
                self.sp += 1;
            }
            3 => {
                // PSW - AF
                let flags = bus.mem_get8(self.sp);
                self.s = (flags & 0x80) == 0x80;
                self.z = (flags & 0x40) == 0x40;
                self.ac = (flags & 0x10) == 0x10;
                self.p = (flags & 0x04) == 0x04;
                self.cy = (flags & 0x01) == 0x01;
                self.sp += 1;
                self.a = bus.mem_get8(self.sp);
                self.sp += 1;
            }
            _ => panic!("ERRO: Instrução não encontrada: {inst:X} / {inst:b}"),
        }
        if self.sp >= 0xCFFF {
            self.sp = 0xC000;
        }
    }

    pub(super) fn sphl(&mut self) {
        self.sp = self.get_reg_pair(2);
    }

    pub(super) fn xthl(&mut self, bus: &mut Bus) {
        let tmp_l: u8 = bus.mem_get8(self.sp);
        let tmp_h: u8 = bus.mem_get8(self.sp + 1);
        bus.mem_set8(self.sp, self.l);
        bus.mem_set8(self.sp + 1, self.h);
        self.l = tmp_l;
        self.h = tmp_h;
    }

    pub(super) fn pchl(&mut self) {
        self.pc = self.get_reg_pair(2);
    }

    pub(super) fn jump(&mut self, inst: u8, bus: &Bus) {
        match inst {
            0xC3 => {
                // jmp
                self.pc = self.fetch16(bus);
            }
            0xDA => {
                // jc
                if self.cy {
                    self.pc = self.fetch16(bus);
                }
            }
            0xD2 => {
                // jnc
                if !self.cy {
                    self.pc = self.fetch16(bus);
                }
            }
            0xCA => {
                // jz
                if self.z {
                    self.pc = self.fetch16(bus);
                }
            }
            0xC2 => {
                // jnz
                if !self.z {
                    self.pc = self.fetch16(bus);
                }
            }
            0xF2 => {
                // jp
                if !self.s {
                    self.pc = self.fetch16(bus);
                }
            }
            0xFA => {
                // jn
                if self.s {
                    self.pc = self.fetch16(bus);
                }
            }
            0xEA => {
                // jpe
                if self.p {
                    self.pc = self.fetch16(bus);
                }
            }
            0xE2 => {
                // jpo
                if !self.p {
                    self.pc = self.fetch16(bus);
                }
            }
            _ => panic!("Instrução não encontrada: {inst:X} / {inst:b}"),
        }
    }

    pub(super) fn call(&mut self, inst: u8, bus: &mut Bus) {
        if self.sp <= 0xC000 {
            self.sp = 0xD000;
        }
        match inst {
            0xCD => {
                // call
                self.sp -= 2;
                bus.mem_set16_reverse(self.sp, self.pc + 2);
                self.pc = self.fetch16(bus);
            }
            0xDC => {
                // cc
                if self.cy {
                    self.sp -= 2;
                    bus.mem_set16_reverse(self.sp, self.pc + 2);
                    self.pc = self.fetch16(bus);
                }
            }
            0xD4 => {
                // cnc
                if !self.cy {
                    self.pc = self.fetch16(bus);
                    self.sp -= 2;
                    bus.mem_set16_reverse(self.sp, self.pc + 2);
                }
            }
            0xCC => {
                // cz
                if self.z {
                    self.sp -= 2;
                    bus.mem_set16_reverse(self.sp, self.pc + 2);
                    self.pc = self.fetch16(bus);
                }
            }
            0xC4 => {
                // cnz
                if !self.z {
                    self.sp -= 2;
                    bus.mem_set16_reverse(self.sp, self.pc + 2);
                    self.pc = self.fetch16(bus);
                }
            }
            0xF4 => {
                // cp
                if !self.s {
                    self.pc = self.fetch16(bus);
                    self.sp -= 2;
                    bus.mem_set16_reverse(self.sp, self.pc + 2);
                }
            }
            0xFC => {
                // cn
                if self.s {
                    self.sp -= 2;
                    bus.mem_set16_reverse(self.sp, self.pc + 2);
                    self.pc = self.fetch16(bus);
                }
            }
            0xEC => {
                // cpe
                if self.p {
                    self.sp -= 2;
                    bus.mem_set16_reverse(self.sp, self.pc + 2);
                    self.pc = self.fetch16(bus);
                }
            }
            0xE4 => {
                // cpo
                if !self.p {
                    self.sp -= 2;
                    bus.mem_set16_reverse(self.sp, self.pc + 2);
                    self.pc = self.fetch16(bus);
                }
            }
            _ => panic!("ERR: Instrução não encontrada: {inst:X} / {inst:b}"),
        }
    }

    pub(super) fn ret(&mut self, inst: u8, bus: &Bus) {
        if self.sp == 0xCFFF {
            self.sp = 0x0000;
        }
        match inst {
            0xC9 => {
                // ret
                self.pc = bus.mem_get16_reverse(self.sp);
                self.sp += 2;
            }
            0xD8 => {
                // rc
                if self.cy {
                    self.pc = bus.mem_get16_reverse(self.sp);
                    self.sp += 2;
                }
            }
            0xD0 => {
                // rnc
                if !self.cy {
                    self.pc = bus.mem_get16_reverse(self.sp);
                    self.sp += 2;
                }
            }
            0xC8 => {
                // rz
                if self.z {
                    self.pc = bus.mem_get16_reverse(self.sp);
                    self.sp += 2;
                }
            }
            0xC0 => {
                // rnz
                if !self.z {
                    self.pc = bus.mem_get16_reverse(self.sp);
                    self.sp += 2;
                }
            }
            0xF0 => {
                //rp
                if !self.s {
                    self.pc = bus.mem_get16_reverse(self.sp);
                    self.sp += 2;
                }
            }
            0xF8 => {
                // rm
                if self.s {
                    self.pc = bus.mem_get16_reverse(self.sp);
                    self.sp += 2;
                }
            }
            0xE8 => {
                // rpe
                if self.p {
                    self.pc = bus.mem_get16_reverse(self.sp);
                    self.sp += 2;
                }
            }
            0xE0 => {
                // rpo
                if !self.p {
                    self.pc = bus.mem_get16_reverse(self.sp);
                    self.sp += 2;
                }
            }
            _ => panic!("ERRO: Instrução não encontrada: {inst:X} / {inst:b}"),
        }
        if self.sp >= 0xCFFF {
            self.sp = 0xC000;
        }
    }

    pub(super) fn rst(&mut self, inst: u8, bus: &mut Bus) {
        if self.sp <= 0xC000 {
            self.sp = 0xD000;
        }

        let value = (inst >> 3) & 0x7;

        self.sp -= 2;
        bus.mem_set16_reverse(self.sp, self.pc + 2);

        self.pc =(value as u16) << 3;
        println!("pc = {}", self.pc);
    }

    pub(super) fn ana(&mut self, bus: &mut Bus, inst: u8) {
        let which = inst & 0x07;
        self.a &= self.get_reg(bus, which);
        self.update_p(self.a);
        self.update_s(self.a);
        self.update_z(self.a);
    }

    pub(super) fn ora(&mut self, bus: &Bus, inst: u8) {
        let which = inst & 0x07;
        self.a |= self.get_reg(bus, which);
        self.update_p(self.a);
        self.update_s(self.a);
        self.update_z(self.a);
    }

    pub(super) fn xra(&mut self, bus: &Bus, inst: u8) {
        let which = inst & 0x07;
        self.a ^= self.get_reg(bus, which);
        self.update_p(self.a);
        self.update_s(self.a);
        self.update_z(self.a);
    }

    pub(super) fn cmp(&mut self, bus: &Bus, inst: u8) {
        let which = inst & 0x07;
        if self.a < self.get_reg(bus, which) {
            self.cy = true;
            self.z = false;
        }
        if self.a == self.get_reg(bus, which) {
            self.cy = false;
            self.z = true;
        } else {
            self.cy = false;
            self.z = false;
        }
    }

    pub(super) fn ani(&mut self, bus: &Bus, inst: u8) {
        let immediate = self.fetch8(bus);
        self.a &= immediate;
        self.update_p(self.a);
        self.update_s(self.a);
        self.update_z(self.a);
    }

    pub(super) fn ori(&mut self, bus: &Bus, inst: u8) {
        let immediate = self.fetch8(bus);
        self.a |= immediate;
        self.update_p(self.a);
        self.update_s(self.a);
        self.update_z(self.a);
    }

    pub(super) fn xri(&mut self, bus: &Bus, inst: u8) {
        let immediate = self.fetch8(bus);
        self.a ^= immediate;
        self.update_p(self.a);
        self.update_s(self.a);
        self.update_z(self.a);
    }

    pub(super) fn cpi(&mut self, bus: &Bus, inst: u8) {
        let immediate = self.fetch8(bus);
        if self.a < immediate {
            self.cy = true;
            self.z = false;
        }
        if self.a == immediate {
            self.cy = false;
            self.z = true;
        } else {
            self.cy = false;
            self.z = false;
        }
    }

    pub(super) fn cma(&mut self) {
        self.a = !self.a;
    }

    pub(super) fn stc(&mut self) {
        self.cy = true;
    }

    pub(super) fn cmc(&mut self) {
        self.cy = !self.cy;
    }

    pub(super) fn nop(&self) {}

    pub(super) fn io_in(&mut self, bus: &Bus) {
        let addr = self.fetch8(bus);
        self.a = bus.io_get8(addr);
    }

    pub(super) fn io_out(&mut self, bus: &mut Bus) {
        let addr = self.fetch8(bus);
        bus.io_set8(addr, self.a);
    }

}
