pub mod lexer;
pub mod parser;
pub mod token;

use lexer::tokenize;
use parser::parse;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub enum AssemblerError {
    SyntaxError(String),
    SemanticError(String)
}

impl fmt::Display for AssemblerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::SyntaxError(content) => write!(f, "Syntax Error: {content}"),
            Self::SemanticError(content) => write!(f, "Semantic Error: {content}"),
        }
    }
}

impl Error for AssemblerError {}

#[allow(dead_code, unused_variables)]
pub fn assemble(input_path: &str, output_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut input = File::open(input_path)?;
    let mut contents = String::new();
    input.read_to_string(&mut contents)?;
    let tokens = tokenize(&contents)?;
    println!("{tokens:?}");
    let machine_code = parse(tokens)?;
    let mut output = File::create(format!("bin/{output_name}.bin"))?;
    output.write_all(machine_code.as_slice())?;
    Ok(())
}
