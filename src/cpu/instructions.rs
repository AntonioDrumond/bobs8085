use super::CPU;

impl CPU {
    pub(super) fn mov(&mut self, inst: u8) {
        let source = self.get_byte(inst & 0x07);
        self.set_byte((inst >> 3) & 0x07, source);
    }
}
