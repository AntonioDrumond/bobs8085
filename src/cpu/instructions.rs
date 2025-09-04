use super::CPU;

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
}
