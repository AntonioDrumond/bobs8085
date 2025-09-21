pub mod assembler;
pub mod bus;
pub mod changes;
pub mod cpu;

use crate::{
    assembler::assemble_program,
    bus::{
        Bus,
        mem::Memory,
        io::Io,
    },
    cpu::CPU,
    changes::Changes,
};

#[derive(Debug)]
pub struct Simulator {
    cpu: CPU,
    bus: Bus,
}

impl Default for Simulator {
    fn default() -> Self {
        Self::new()
    }
}

impl Simulator {
    pub fn new() -> Simulator {
        Simulator { cpu: CPU::default(), bus: Bus::default(), }
    }

    pub fn cpu_print_state(&self) {
        self.cpu.print_state();
    }

    pub fn bus_from_file(filename: &str) -> Simulator {
        Simulator { cpu: CPU::default(), bus: Bus::from_file(filename), }
    }

    pub fn execute(&mut self) -> bool {
        self.cpu.execute(&mut self.bus)
    }

    pub fn print_state(&self) {
        self.cpu.print_state();
        println!("\n");
        match self.bus.mem_write_file("./memory.txt") {
            Ok(()) => println!("Memory saved to \"./memory.txt\""),
            Err(e) => eprintln!("Error printing memory: {e}"),
        }
        match self.bus.io_write_file("./io.txt") {
            Ok(()) => println!("IO ports saved to \"./io.txt\""),
            Err(e) => eprintln!("Error printing IO: {e}"),
        }
    }
    
    pub fn get_pc(&mut self) -> u16 {
        self.cpu.get_pc()
    }

    pub fn set_pc(&mut self, val: u16) {
        self.cpu.set_pc(val);
    }

    pub fn cpu_get_reg(&self, target: u8) -> u8 {
        self.cpu.get_reg(&self.bus, target)
    }

    pub fn cpu_get_reg_pair(&self, target: u8) -> u16 {
        self.cpu.get_reg_pair(target)
    }

    pub fn clone_cpu_bus(&self) -> (CPU, Memory) {
        (self.cpu.clone(), self.bus.mem_clone())
    }

    pub fn get_changes(&self, cpu_old: CPU, mem_old: Memory, io_old: Io) -> Changes {
        Changes { 
                cpu: self.cpu.diff(cpu_old), 
                memory: self.bus.mem_diff(mem_old),
                io: self.bus.io_diff(io_old),
        }
    }

    pub fn mem_get8(&self, pos: u16) -> u8 {
        self.bus.mem_get8(pos)
    }

    pub fn restore(&mut self, changes: &Changes) {
        self.cpu.restore(&mut self.bus, changes);
    }

    pub fn print_program(&self) {
        self.bus.mem_print_program();
    }

    pub fn print_mem_range(&self, lower: u16, upper: u16) {
        self.bus.mem_print_range(lower, upper);
    }
}


pub fn assemble(input_path: &str, output_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    assemble_program(input_path, output_name)
}

