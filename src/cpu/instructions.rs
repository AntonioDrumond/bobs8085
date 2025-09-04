use super::CPU;
use crate::bus::Bus;

impl CPU {
    pub(super) fn mov(&mut self, inst: u8) {
        let s = inst & 0x07;
        let d = (inst >> 3) & 0x07;
        let value = self.get_reg(s);
        self.set_reg(d, value);
    }

    pub(super) fn inr(&mut self, inst: u8) {
        let d = (inst >> 3) & 0x07;
        let value = self.get_reg(d);
        self.set_reg(d, value + 1);
    }

    pub(super) fn dcr(&mut self, inst: u8) {
        let d = (inst >> 3) & 0x07;
        let value = self.get_reg(d);
        self.set_reg(d, value - 1);
    }

    pub(super) fn inx(&mut self, inst: u8) {
        let d = (inst >> 4) & 0x03;
        let value = self.get_reg_pair(d);
        self.set_reg_pair(d, value + 1);
    }

    pub(super) fn dcx(&mut self, inst: u8) {
        let d = (inst >> 4) & 0x03;
        let value = self.get_reg_pair(d);
        self.set_reg_pair(d, value - 1);
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

    pub(super) fn add(&mut self, inst: u8) {
        let s = inst & 0x07;
        let value = self.get_reg(s);
        let prev_a = self.a;
        self.a = prev_a + value;
        self.update_s(self.a);
        self.update_z(self.a);
        self.update_p(self.a);
        self.cy = self.a < prev_a;
    }

    pub(super) fn adc(&mut self, inst: u8) {
        let s = inst & 0x07;
        let value = self.get_reg(s);
        let prev_a = self.a;
        self.a = prev_a + value + self.cy as u8;
        self.update_s(self.a);
        self.update_z(self.a);
        self.update_p(self.a);
        self.cy = self.a < prev_a;
    }

    pub(super) fn adi(&mut self, bus: &mut Bus, inst: u8) {
        let value = bus.mem_get8(self.pc + 1);
        let prev_a = self.a;
        self.a = prev_a + value;
        self.update_s(self.a);
        self.update_z(self.a);
        self.update_p(self.a);
        self.cy = self.a < prev_a;
    }

    pub(super) fn aci(&mut self, bus: &mut Bus, inst: u8) {
        let value = bus.mem_get8(self.pc + 1);
        let prev_a = self.a;
        self.a = prev_a + value + self.cy as u8;
        self.update_s(self.a);
        self.update_z(self.a);
        self.update_p(self.a);
        self.cy = self.a < prev_a;
    }

    pub(super) fn dad(&mut self, inst: u8) {
        let s = (inst >> 4) & 0x03;
        let value = self.get_reg_pair(s);
        let prev_hl = self.get_reg_pair(2);
        self.set_reg_pair(2, prev_hl + value);
        let cur_hl = self.get_reg_pair(2);
        self.cy = cur_hl < prev_hl;
    }
}
