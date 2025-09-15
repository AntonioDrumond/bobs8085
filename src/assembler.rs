mod token;
mod lexer;
mod parser;

use std::fs::File;
use std::io::prelude::*;
use lexer::tokenize;
use parser::parse;

#[allow(dead_code, unused_variables)]
pub fn assemble(input_path: &str, output_path: &str) -> std::io::Result<()> {
    let mut input = File::open(input_path)?;
    let mut contents = String::new();
    input.read_to_string(&mut contents)?;
    let tokens = tokenize(&contents);
    parse(tokens);
    Ok(())
}
