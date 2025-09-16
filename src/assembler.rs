pub mod lexer;
pub mod parser;
pub mod token;

use lexer::tokenize;
use parser::parse;
use std::fs::File;
use std::io::prelude::*;

#[allow(dead_code, unused_variables)]
pub fn assemble(input_path: &str, output_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut input = File::open(input_path)?;
    let mut contents = String::new();
    input.read_to_string(&mut contents)?;
    let tokens = tokenize(&contents);
    println!("{:?}", tokens);
    let machine_code = parse(tokens)?;
    let mut output = File::create(format!("bin/{}.o", output_name))?;
    output.write_all(machine_code.as_slice())?;
    Ok(())
}
