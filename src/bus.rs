mod mem;
use crate::bus::mem::Memory;

#[allow(dead_code, unused_variables)]
#[derive(Debug)]
pub struct Bus {
    mem: Memory,
    // io: IO,
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}

impl Bus {
    pub fn new() -> Bus {
        Bus { mem: Memory::default() }
    }

    pub fn mem_get8(&self, pos:u16) -> u8 {
        self.mem.get8(pos)
    }

    pub fn mem_get16(&self, pos:u16) -> u16 {
        self.mem.get16(pos)
    }

    pub fn mem_set8(&mut self, pos:u16, value:u8) {
        self.mem.set8(pos, value);
    }

    pub fn mem_set16(&mut self, pos:u16, value:u8) {
        self.mem.set8(pos, value);
    }
}
