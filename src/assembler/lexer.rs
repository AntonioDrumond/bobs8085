use crate::assembler::AssemblerError;
use super::token::Token;

use AssemblerError::SyntaxError;

fn str_to_tok(str: &str) -> Result<Token, AssemblerError> {
    if let Some(hex) = str.to_lowercase().strip_prefix("0x") {
        if hex.is_empty() {
            return Err(SyntaxError(String::from("empty hex literal")))
        }
        match u16::from_str_radix(hex, 16)
        {
            Ok(value) => return Ok(Token::HexLiteral(value)),
            Err(_e) => return Err(SyntaxError(format!("invalid hex literal {hex}")))
        }
    } else {
        return Ok(Token::Name(str.to_string()))
    }
}

fn flush_buffer(buf: &mut String, tokens: &mut Vec<Token>) -> Result<(), AssemblerError> {
    if !buf.is_empty() {
        tokens.push(str_to_tok(buf)?);
        buf.clear();
    }
    Ok(())
}

pub fn tokenize(buffer: &str) -> Result<Vec<Token>, AssemblerError> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut buf = String::new();
    for line in buffer.lines() {
        let mut chars = line.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                ';' => break,
                '/' => { 
                    if chars.peek() == Some(&'/') {
                        break;
                    } else {
                        return Err(SyntaxError(format!("invalid character: {c}")))
                    }
                }
                ',' => {
                    flush_buffer(&mut buf, &mut tokens)?;
                    tokens.push(Token::Comma);
                }
                ':' => {
                    flush_buffer(&mut buf, &mut tokens)?;
                    tokens.push(Token::Colon);
                }
                c if c.is_whitespace() => flush_buffer(&mut buf, &mut tokens)?,
                c if !c.is_alphabetic() && !c.is_ascii_digit() => return Err(SyntaxError(format!("invalid character: {c}"))),
                _ => buf.push(c),
            }
        }
        flush_buffer(&mut buf, &mut tokens)?;
        tokens.push(Token::NewLine);
    }
    Ok(tokens)
}
