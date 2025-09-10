use super::tokens::*;

/**
 * Basic lexing algorithm
 * No test with characters outside of ASCII table were made
 * No test with control characters were intensively made
 */

fn str_to_tok(str: &str) -> Option<Box<dyn Token>> {
    let mut chars = str.chars();
    if str.len() == 1 {
        if let Some(char) = chars.nth(0) {
            if char == ',' {
                return Some(Box::new(Comma));
            } else if char.is_alphabetic() {
                return Some(Box::new(Register {
                    content: char.to_string(),
                }));
            } else if char.is_numeric() {
                return Some(Box::new(Address {
                    content: char.to_string(),
                }));
            }
            return None;
        } else {
            unreachable!();
        }
    }
    for char in chars {
        if char.is_alphabetic() {
            return Some(Box::new(Instruction {
                content: String::from(str),
            }));
        }
    }
    Some(Box::new(Address {
        content: String::from(str),
    }))
}

pub(super) fn tokenize(buff: &str) -> Vec<Box<dyn Token>> {
    let mut tokens: Vec<Box<dyn Token>> = Vec::new();
    let mut str = String::new();
    for line in buff.lines() {
        for c in line.chars() {
            if c.is_whitespace() || c.is_control() {
                if !str.is_empty() {
                    if let Some(tok) = str_to_tok(&str) {
                        tokens.push(tok);
                    }
                    str.clear();
                }
            } else if c == ',' {
                if !str.is_empty() {
                    if let Some(tok) = str_to_tok(&str) {
                        tokens.push(tok);
                    }
                    str.clear();
                }
                tokens.push(Box::new(Comma));
            } else if c == ';' {
                break;
            } else {
                str.push(c);
            }
        }
        if !str.is_empty() {
            if let Some(tok) = str_to_tok(&str) {
                tokens.push(tok);
            }
        }
        tokens.push(Box::new(NewLine));
    }
    tokens
}
