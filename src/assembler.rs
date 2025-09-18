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
    UnknownName(String),
    UnknownInstruction(String),
    InvalidInstructionArgument(String),
    InvalidInstructionFormat,
    InvalidValueFormat(String),
    UnknownRegister(String),
    LabelNotDefined(String),
    MissingNewLine,
    UnexpectedToken,
    InvalidCharacter(char),
    SyntaxError(String)
}

impl fmt::Dislplay for AssemblerError {
    fn fmt(&self, f: &mut fmt::Fomatter) -> fmt::Result {
        match self {
            Self::UnknownName(content) => write!(f, ""),
            Self::UnknownInstruction(content) => write!(f, ""),
            Self::InvalidInstructionArgument(content) => write!(f, ""),
            Self::InvalidInstructionFormat => write!(f, ""),
            Self::InvalidValueFormat(content) => write!(f, ""),
            Self::UnknownRegister(content) => write!(f, ""),
            Self::LabelNotDefined(content) => write!(f, ""),
            Self::MissingNewLine => write!(f, ""),
            Self::UnexpectedToken => write!(f, ""),
            Self::InvalidCharacter(content) => write!(f, ""),
            Self::SyntaxError(content) => write!(f, "")
        }
    }
}

impl Error for AssemblerError {}

#[allow(dead_code, unused_variables)]
pub fn assemble(input_path: &str, output_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut input = File::open(input_path)?;
    let mut contents = String::new();
    input.read_to_string(&mut contents)?;
    let tokens = tokenize(&contents);
    println!("{tokens:?}");
    let machine_code = parse(tokens)?;
    let mut output = File::create(format!("bin/{output_name}.bin"))?;
    output.write_all(machine_code.as_slice())?;
    Ok(())
}
