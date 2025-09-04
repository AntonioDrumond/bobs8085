#[allow(dead_code, unused_variables)]
#[derive(Debug)]
pub struct Memory { 
    arr: Vec<u8>,
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code, unused_variables)]
impl Memory {
    pub fn new() -> Memory {
        Memory { arr: vec![0; 0xFFFF+2] }
    }

    pub fn print(&self) {
        let mut i = 0;
        while i < self.arr.len()-1 {
            println!("{:04X} => {:X?}", i, &self.arr[i..i+16]);
            i += 16;
        }
    }

    pub fn dump(){
        todo!();
    }

    pub fn get8(&self, pos:u16) -> u8 {
        self.arr[pos as usize]
    }

    pub fn get16(&self, pos:u16) -> u16 {
        (self.arr[pos as usize] as u16) << 4 | self.arr[pos as usize + 1] as u16
    }

    pub fn set8(&mut self, pos:u16, value:u8) {
        self.arr[pos as usize] = value;
    }

    pub fn set16(&mut self, pos:u16, value:u16) {
        let hi:u8 = (value & 0xF) as u8;
        let lo:u8 = (value >> 4) as u8;
        self.arr[pos as usize] = lo;
        self.arr[pos as usize + 1] = hi;
        self.arr[0xFFFF+1] = 0;
    }

}

