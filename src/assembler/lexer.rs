use super::token::Token;

/**
 * Basic lexing algorithm
 * No test with characters outside of ASCII table were made
 * No test with control characters were intensively made
 */

fn str_to_tok(str: &str) -> Option<Box<Token>> {
    let first = str.chars().next()?;
    let tok = match first {
        ',' => Token::comma(),
        '.' => Token::label(str),
        c if c.is_alphabetic() && str.len() == 1 => Token::register(str),
        _ => {
            if str.chars().any(|c| c.is_numeric()) {
                Token::value(str)
            } else {
                Token::instruction(str)
            }
        }
    };
    Some(Box::new(tok))
}

fn try_add_token(buf: &mut String, tokens: &mut Vec<Box<Token>>) {
    if !buf.is_empty() {
        if let Some(tok) = str_to_tok(&buf) {
            tokens.push(tok);
        }
        buf.clear();
    }
}

pub(super) fn tokenize(buffer: &str) -> Vec<Box<Token>> {
    let mut tokens: Vec<Box<Token>> = Vec::new();
    let mut buf = String::new();
    for line in buffer.lines() {
        for c in line.chars() {
            match c {
                ';' | '/' => break,
                ',' => {
                    try_add_token(&mut buf, &mut tokens);
                    tokens.push(Box::new(Token::comma()));
                }
                c if c.is_whitespace() => try_add_token(&mut buf, &mut tokens),
                _ => buf.push(c),
            }
        }
        try_add_token(&mut buf, &mut tokens);
        tokens.push(Box::new(Token::new_line()));
    }
    tokens
}
