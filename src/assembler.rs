pub mod lexer;
pub mod parser;
pub mod token;

use lexer::tokenize;
use parser::parse;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::{fmt, fs};

#[derive(Debug)]
pub enum AssemblerError {
    SyntaxError(String, Option<usize>, Option<usize>),
    SemanticError(String, Option<usize>, Option<usize>),
}

impl fmt::Display for AssemblerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (kind, content, line, column) = match self {
            Self::SyntaxError(content, line, column) => ("Syntax Error", content, line, column),
            Self::SemanticError(content, line, column) => ("Semantic Error", content, line, column),
        };

        write!(f, "{}: {}", kind, content)?;

        if line.is_some() || column.is_some() {
            write!(f, " (")?;
            if let Some(l) = line {
                write!(f, "line {l}")?;
            }
            if let Some(c) = column {
                if line.is_some() {
                    write!(f, ", ")?;
                }
                write!(f, "column {c}")?;
            }
            write!(f, ")")?;
        }

        Ok(())
    }
}

impl Error for AssemblerError {}

#[allow(dead_code, unused_variables)]
pub fn assemble(input_path: &str, output_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut input = File::open(input_path)?;
    let mut contents = String::new();
    input.read_to_string(&mut contents)?;
    let tokens = tokenize(&contents)?;
    println!("{tokens:#?}");
    let machine_code = parse(&tokens)?;
    fs::create_dir_all("bin/")?;
    let mut output = File::create(format!("bin/{output_name}.bin"))?;
    output.write_all(machine_code.as_slice())?;
    Ok(())
}

#[allow(dead_code)]
pub fn print_warning(warning: String, line: Option<usize>, column: Option<usize>) {
    print!("Warning: {}", warning);
    if let Some(l) = line {
        print!(" (line {}", l);
        if let Some(c) = column {
            print!(", column {}", c);
        }
        print!(")");
    }
    println!()
}
