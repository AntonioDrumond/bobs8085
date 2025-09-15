use super::token::Token;

/**
 * Basic lexing algorithm
 * No test with characters outside of ASCII table were made
 * No test with control characters were intensively made
 */

fn str_to_tok(str: &str, found_inst: bool) -> (bool, Option<Token>) {
    if str.chars().next().unwrap() == '/' {
        return (false, None);
    }

    let mut is_inst = false;
    let tok = if str.len() == 1 {
        Token::register(str)
    } else if str.ends_with(':') {
        Token::label_declr(str)
    } else if str.chars().any(|c| c.is_numeric()) {
        Token::value(str)
    } else if found_inst {
        Token::label_value(str)
    } else {
        is_inst = true;
        Token::instruction(str)
    };
    
    (is_inst, Some(tok))
}

fn try_add_token(buf: &mut String, tokens: &mut Vec<Token>, found_inst: bool) -> bool {
    if !buf.is_empty() {
        let (is_inst, token_opt) = str_to_tok(&buf, found_inst);
        if let Some(tok) = token_opt {
            tokens.push(tok);
        }
        buf.clear();
        return is_inst;
    }
    false
}

pub fn tokenize(buffer: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut buf = String::new();
    let mut found_inst = false;
    for line in buffer.lines() {
        for c in line.chars() {
            match c {
                ';' => break,
                '/' => {
                    if buf.starts_with('/') {
                        break;
                    } else {
                        found_inst = try_add_token(&mut buf, &mut tokens, found_inst);
                        buf.push(c);
                    }
                }
                ',' => {
                    found_inst = try_add_token(&mut buf, &mut tokens, found_inst);
                    tokens.push(Token::comma());
                }
                c if c.is_whitespace() => found_inst = try_add_token(&mut buf, &mut tokens, found_inst),
                _ => buf.push(c),
            }
        }
        try_add_token(&mut buf, &mut tokens, found_inst);
        tokens.push(Token::new_line());
    }
    tokens
}
