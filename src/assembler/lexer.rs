use super::token::Token;
use crate::assembler::AssemblerError;

use AssemblerError::SyntaxError;

fn str_to_tok(str: &str, line: usize, column: usize) -> Result<Token, AssemblerError> {
    let treated = str
        .strip_prefix("0x")
        .or_else(|| str.strip_suffix('H'))
        .or_else(|| str.strip_suffix('h'))
        .unwrap_or("");

    if treated.len() <= 4 && u16::from_str_radix(treated, 16).is_ok() {
        return Ok(Token::new_hex_literal(
            str.to_string(),
            line + 1,
            column + 1,
        ));
    } else {
        return Ok(Token::new_name(str.to_string(), line + 1, column + 1));
    }
}

fn flush_buffer(
    buf: &mut String,
    tokens: &mut Vec<Token>,
    line: usize,
    column: usize,
) -> Result<(), AssemblerError> {
    if !buf.trim().is_empty() {
        tokens.push(str_to_tok(buf, line, column)?);
        buf.clear();
    }
    Ok(())
}

pub fn tokenize(buffer: &str) -> Result<Vec<Token>, AssemblerError> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut buf = String::new();
    let mut lines = buffer.lines().enumerate().peekable();
    while let Some((i, l)) = lines.next() {
        let mut chars = l.chars().enumerate().peekable();
        while let Some((j, c)) = chars.next() {
            match c {
                ';' => break,
                '/' => {
                    if matches!(chars.peek(), Some((_, next)) if *next == '/') {
                        break;
                    } else {
                        return Err(SyntaxError(
                            format!("invalid character \"{c}\""),
                            Some(i + 1),
                            Some(j + 1),
                        ));
                    }
                }
                ',' => {
                    flush_buffer(&mut buf, &mut tokens, i, j)?;
                    tokens.push(Token::new_comma(i + 1, j + 1));
                }
                ':' => {
                    flush_buffer(&mut buf, &mut tokens, i, j)?;
                    tokens.push(Token::new_colon(i + 1, j + 1));
                }
                c if c.is_whitespace() => flush_buffer(&mut buf, &mut tokens, i, j)?,
                c if !c.is_alphabetic() && !c.is_ascii_digit() => {
                    return Err(SyntaxError(
                        format!("invalid character \"{c}\""),
                        Some(i),
                        Some(j),
                    ));
                }
                _ => buf.push(c),
            }
        }
        flush_buffer(&mut buf, &mut tokens, i, l.len())?;
        tokens.push(Token::new_new_line(i + 1, l.len() + 1));
        //flush_buffer(&mut buf, &mut tokens, i, l.len())?;
    }
    Ok(tokens)
}
