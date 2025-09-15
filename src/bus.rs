mod mem;
mod io;
use crate::bus::io::Io;
use crate::bus::mem::Memory;

#[allow(dead_code, unused_variables)]
#[derive(Debug)]
pub struct Bus {
    mem: Memory,
    io: Io,
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code, unused_variables)]
impl Bus {
    
    pub fn from_file(filename: &str) -> Bus {
        todo!("Create bus from memory file");
    }

    pub fn new() -> Bus {
        Bus { mem: Memory::default(), io: Io::default() }
    }

    pub fn mem_get8(&self, pos:u16) -> u8 {
        self.mem.get8(pos)
    }

    pub fn mem_get16(&self, pos:u16) -> u16 {
        self.mem.get16(pos)
    }

    pub fn mem_get16_reverse(&self, pos:u16) -> u16 {
        self.mem.get16_reverse(pos)
    }

    pub fn mem_set8(&mut self, pos:u16, value:u8) {
        self.mem.set8(pos, value);
    }

    pub fn mem_set16(&mut self, pos:u16, value:u16) {
        self.mem.set16(pos, value);
    }

    pub fn mem_set16_reverse(&mut self, pos:u16, value:u16) {
        self.mem.set16_reverse(pos, value);
    }

    pub fn mem_dump(&self, filename:&str) -> std::io::Result<()> {
        self.mem.dump(filename)?;
        Ok(())
    }

    pub fn mem_read_dump(&mut self, filename:&str) -> std::io::Result<()> {
        self.mem.read_dump(filename)?;
        Ok(())
    }

    pub fn mem_write_file(&self, filename:&str) -> std::io::Result<()> {
        self.mem.write_file(filename)?;
        Ok(())
    }

    pub fn mem_print(&self) {
        self.mem.print();
    }

    pub fn mem_print_program(&self) {
        self.mem.print_program();
    }

    pub fn io_print(&self) {
        self.io.print();
    }

    pub fn io_get8(&self, pos:u8) -> u8 {
        self.io.get8(pos)
    }

    pub fn io_get16(&self, pos:u8) -> u16 {
        self.io.get16(pos)
    }

    pub fn io_get16_reverse(&self, pos:u8) -> u16 {
        self.io.get16_reverse(pos)
    }

    pub fn io_set8(&mut self, pos:u8, value:u8) {
        self.io.set8(pos, value);
    }

    pub fn io_set16(&mut self, pos:u8, value:u16) {
        self.io.set16(pos, value);
    }

    pub fn io_set16_reverse(&mut self, pos:u8, value:u16) {
        self.io.set16_reverse(pos, value);
    }

    pub fn io_write_file(&self, filename:&str) -> std::io::Result<()> {
        self.io.write_file(filename)?;
        Ok(())
    }

}
