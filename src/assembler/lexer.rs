use super::token::Token;

/**
 * Basic lexing algorithm
 * No test with characters outside of ASCII table were made
 * No test with control characters were intensively made
 */

fn str_to_tok(str: &str, found_inst_on_line: bool) -> (bool, Token) {
    let mut is_inst = false;
    let lower = str.to_lowercase();
    let value_opt = lower
        .strip_suffix('h')
        .filter(|val| !val.is_empty())
        .map(|val| val.to_string());

    let token = if let Some(label) = str.strip_suffix(':') {
        Token::LabelDeclaration(label.to_string())
    } else if let Some(value) = value_opt {
        Token::Value(value)
    } else if found_inst_on_line {
        if matches!(
            lower.as_str(),
            "b" | "c" | "d" | "e" | "h" | "l" | "m" | "a" | "sp"
        ) {
            Token::Register(lower)
        } else {
            Token::LabelValue(str.to_string())
        }
    } else {
        is_inst = true;
        Token::Instruction(lower)
    };
    (is_inst, token)
}

fn flush_buffer(buf: &mut String, tokens: &mut Vec<Token>, found_inst_on_line: bool) -> bool {
    if buf.is_empty() {
        return false;
    }
    let (is_inst, token) = str_to_tok(buf, found_inst_on_line);
    tokens.push(token);
    buf.clear();
    is_inst
}

pub fn tokenize(buffer: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut buf = String::new();
    let mut found_inst_on_line = false;
    for line in buffer.lines() {
        let mut chars = line.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                ';' => break,
                '/' if chars.peek() == Some(&'/') => break,
                ',' => {
                    found_inst_on_line |= flush_buffer(&mut buf, &mut tokens, found_inst_on_line);
                    tokens.push(Token::Comma);
                }
                c if c.is_whitespace() => {
                    found_inst_on_line |= flush_buffer(&mut buf, &mut tokens, found_inst_on_line)
                }
                _ => buf.push(c),
            }
        }
        flush_buffer(&mut buf, &mut tokens, found_inst_on_line);
        tokens.push(Token::NewLine);
        found_inst_on_line = false;
    }
    tokens
}
