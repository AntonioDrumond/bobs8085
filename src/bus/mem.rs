use std::fs::File;
use std::io::prelude::*;

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

    pub fn clone(&self) -> Memory {
        Memory { arr: self.arr.clone() }
    }
    
    pub fn diff(&self, other:Memory) -> Vec<(u16, u8)> {
        
        let mut changes: Vec<(u16, u8)> = vec![];
        let mut j : u16 = 0;

        for i in &self.arr {
            if *i != other.arr[j as usize]
            {
                changes.push((j, other.arr[j as usize]));
            }
            j = j.wrapping_add(1);
        }
        changes
    }

    pub fn print(&self) {
        let mut i = 0;
        while i < self.arr.len()-1 {
            println!("{:04X} => {:02X?}", i, &self.arr[i..i+16]);
            i += 16;
        }
    }

    pub fn print_range(&self, lower:u16, upper:u16) {
        let mut i = lower as usize;
        while i < upper.into() {
            println!("{:04X} => {:02X?}", i, &self.arr[i..i+16]);
            i += 16;
        }
    }

    pub fn print_program(&self) {
        self.print_range(0xC000, 0xCFFF);
    }

    pub fn write_file(&self, filename:&str) -> std::io::Result<()> {
        let mut file = File::create(filename)?;
        let mut i = 0;
        let mut str = String::default();

        while i < self.arr.len()-1 {
            let slice = &self.arr[i..i+16.min(self.arr.len()-i)];
            let line = format!("{i:04X} => {slice:02X?}\n");
            str.push_str(&line);
            i += 16;
        }
        file.write_all(str.as_bytes())?;
        Ok(())
    }

    pub fn dump(&self, filename:&str) -> std::io::Result<()> {
        let mut file = File::create(filename)?;
        let mut i = 0;
        while i < self.arr.len()-1 {
            file.write_all(&self.arr[i..i+16])?;
            i += 16;
        }
        Ok(())
    }
    
    pub fn read_dump(&mut self, filename:&str) -> std::io::Result<()> {
        let mut file = File::open(filename)?;
        let mut i = 0;
        while i < self.arr.len()-1 {
            file.read_exact(&mut self.arr[i..i+16])?;
            i+=16;
        }
        Ok(())
    }

    pub fn get8(&self, pos:u16) -> u8 {
        self.arr[pos as usize]
    }

    pub fn get16(&self, pos:u16) -> u16 {
        (self.arr[pos as usize] as u16) << 8 | self.arr[pos as usize + 1] as u16
    }

    pub fn get16_reverse(&self, pos:u16) -> u16 {
        (self.arr[pos as usize + 1] as u16) << 8 | self.arr[pos as usize] as u16
    }

    pub fn set8(&mut self, pos:u16, value:u8) {
        self.arr[pos as usize] = value;
    }

    pub fn set16(&mut self, pos:u16, value:u16) {
        let hi:u8 = (value & 0x00FF) as u8;
        let lo:u8 = (value >> 8) as u8;
        self.arr[pos as usize] = lo;
        self.arr[pos as usize + 1] = hi;
        self.arr[0xFFFF+1] = 0;
    }

    pub fn set16_reverse(&mut self, pos:u16, value:u16) {
        let lo:u8 = (value & 0x00FF) as u8;
        let hi:u8 = (value >> 8) as u8;
        self.arr[pos as usize] = lo;
        self.arr[pos as usize + 1] = hi;
        self.arr[0xFFFF+1] = 0;
    }


}

