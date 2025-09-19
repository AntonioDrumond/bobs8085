pub mod assembler;
pub mod bus;
pub mod changes;
pub mod cpu;

use std::{
    io,
    io::Write,
};

use crate::{
    assembler::assemble_program,
    bus::Bus,
    changes::Changes,
    cpu::CPU,
};

pub struct Simulator {
    pub cpu: CPU,
    pub bus: Bus,
}

impl Simulator {
    
}


pub fn assemble(input_path: &str, output_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    assemble_program(input_path, output_name)
}

