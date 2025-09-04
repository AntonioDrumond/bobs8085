use super::CPU;

const B   : u8 = 0;
const C   : u8 = 1;
const D   : u8 = 2;
const E   : u8 = 3;
const H   : u8 = 4;
const L   : u8 = 5;
const MEM : u8 = 6;
const A   : u8 = 7;

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

    pub(super) fn add(&mut self, inst: u8) {
        let s = inst & 0x07;
        let value = self.get_reg(s);
        let a_value = self.get_reg(A);
        self.set_reg(A, a_value + value);
    }

    pub(super) fn adc(&mut self, inst: u8) {
    }

    pub(super) fn adi(&mut self, inst: u8) {
    }

    pub(super) fn aci(&mut self, inst: u8) {
    }

    pub(super) fn dad(&mut self, inst: u8) {
    }
}
