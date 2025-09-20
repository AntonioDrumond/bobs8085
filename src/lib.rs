pub mod assembler;
pub mod bus;
pub mod changes;
pub mod cpu;
pub mod utils;


use crate::{
    assembler::assemble_program,
    bus::Bus,
    cpu::CPU,
};

pub struct Simulator {
    pub cpu: CPU,
    pub bus: Bus,
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
    
}


pub fn assemble(input_path: &str, output_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    assemble_program(input_path, output_name)
}

