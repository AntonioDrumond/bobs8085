use crate::assembler::AssemblerError;
use super::token::Token;

use AssemblerError::SyntaxError;

fn str_to_tok(str: &str) -> Result<Token, AssemblerError> {
    if let Some(hex) = str.to_lowercase().strip_prefix("0x") {
        if hex.is_empty() {
            return Err(SyntaxError("empty hex literal"))
        }
        return u16::from_str_radix(hex, 16)
            .map(Token::Value)
            .map_err(|_| Err(SyntaxError("invalid hex literal")))
    } else {
        return Token::Name(str)
    }
}

fn flush_buffer(buf: &mut String, tokens: &mut Vec<Token>) -> Result<(), AssemblerError> {
    if !buf.is_empty() {
        tokens.push(str_to_tok(buf)?);
        buf.clear();
    }
}

pub fn tokenize(buffer: &str) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
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
                        Err(SyntaxError(format!("invalid character: {c}")))
                    }
                }
                ',' => {
                    flush_buffer(&mut buf, &mut tokens);
                    tokens.push(Token::Comma);
                }
                ':' => {
                    flush_buffer(&mut buf, &mut tokens);
                    tokens.push(Token::Colon);
                }
                c if c.is_whitespace() => flush_buffer(&mut buf, &mut tokens),
                c if !c.is_alphabetic() && !c.is_ascii_digit() => Err(SyntaxError(format!("invalid character: {c}"))),
                _ => buf.push(c),
            }
        }
        flush_buffer(&mut buf, &mut tokens);
        tokens.push(Token::NewLine);
    }
    tokens
}
