use std::fs::File;
use std::io::prelude::*;

#[allow(dead_code, unused_variables)]
#[derive(Debug)]
pub struct Io{ 
    arr: Vec<u8>,
}

impl Default for Io {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code, unused_variables)]
impl Io {
    pub fn new() -> Io {
        Io { arr: vec![0; 0xFF+2] }
    }

    pub fn print(&self) {
        let mut i = 0;
        while i < self.arr.len()-1 {
            println!("{:02X} => {:02X?}", i, &self.arr[i..i+16]);
            i += 16;
        }
    }

    pub fn clone(&self) -> Io {
        Io { arr: self.arr.clone() }
    }

    pub fn diff(&self, other:Io) -> Vec<(u8, u8)> {
        let mut changes: Vec<(u8, u8)> = vec![];
        let mut j : u8 = 0;

        for i in &self.arr {
            if *i != other.arr[j as usize] {
                changes.push((j, other.arr[j as usize]));
            }
            j = j.wrapping_add(1);
        }
        changes
    }

    pub fn get8(&self, pos:u8) -> u8 {
        self.arr[pos as usize]
    }

    pub fn get16(&self, pos:u8) -> u16 {
        (self.arr[pos as usize] as u16) << 8 | self.arr[pos as usize + 1] as u16
    }

    pub fn get16_reverse(&self, pos:u8) -> u16 {
        (self.arr[pos as usize + 1] as u16) << 8 | self.arr[pos as usize] as u16
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

    pub fn set8(&mut self, pos:u8, value:u8) {
        self.arr[pos as usize] = value;
    }

    pub fn set16(&mut self, pos:u8, value:u16) {
        let hi:u8 = (value & 0x00FF) as u8;
        let lo:u8 = (value >> 8) as u8;
        self.arr[pos as usize] = lo;
        self.arr[pos as usize + 1] = hi;
        self.arr[0xFF+1] = 0;
    }

    pub fn set16_reverse(&mut self, pos:u8, value:u16) {
        let lo:u8 = (value & 0x00FF) as u8;
        let hi:u8 = (value >> 8) as u8;
        self.arr[pos as usize] = lo;
        self.arr[pos as usize + 1] = hi;
        self.arr[0xFF+1] = 0;
    }

}
