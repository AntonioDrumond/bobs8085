use std::collections::{ HashMap, VecDeque };
use std::error::Error;
use std::fmt::Display;
use super::token::*;

#[derive(Debug)]
pub enum ParserError {
    UnknownName(String),
    UnknownInstruction(String),
    InvalidInstructionArgument(String),
    InvalidInstructionFormat,
    InvalidValueFormat(String),
    UnknownRegister(String),
    MissingNewLine,
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::UnknownName(fragment) =>
                write!(f, "unknown name \"{}\"", fragment),
            ParserError::UnknownInstruction(fragment) =>
                write!(f, "unknown instruction \"{}\"", fragment),
            ParserError::InvalidInstructionArgument(fragment) =>
                write!(f, "invalid instruction argument \"{}\"", fragment),
            ParserError::InvalidInstructionFormat =>
                write!(f, "invalid instruction format, expected comma"),
            ParserError::InvalidValueFormat(fragment) =>
                write!(f, "invalid value format \"{}\"", fragment),
            ParserError::UnknownRegister(fragment) =>
                write!(f, "unknown register \"{}\"", fragment),
            ParserError::MissingNewLine => 
                write!(f, "missing new line after instruction")
        }
    }
}

impl std::error::Error for ParserError {}

#[derive(Debug)]
enum State {
    Search,
    Comma,
    Arg30,
    Arg33,
    Arg24,
    Immediate8,
    Immediate16,
    Append,
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<u8>, ParserError> {
    let mut state_queue: VecDeque<State> = VecDeque::from([State::Search]);
    let mut buffer: Vec<u8> = Vec::new();
    let mut next_bytes: u32 = 0;
    let mut expect_label = false;
    let mut address: u32 = 0xC000;
    let mut expecting_label: Vec<u32> = Vec::new();
    let mut labels: HashMap<String, u32> = HashMap::new();
    for token in tokens {
        if let Some(state) = state_queue.pop_front() {
            println!("{:?}, {:?}", state, token);
            match state {
                State::Search => match token.get_token_type() {
                    TokenType::Instruction => {
                        let (op, states_opt) = encode_inst(token.get_content())?;
                        if let Some(states) = states_opt {
                            state_queue.extend(states);
                        }
                        next_bytes = op as u32;
                        state_queue.push_back(State::Append);
                    }
                    TokenType::LabelDeclaration => labels.insert(token.get_content.to_string(), address),
                    TokenType::NewLine => state_queue.push_back(State::Search),
                    _ => return Err(ParserError::UnknownName(token.get_content().to_string())),
                },
                State::Comma => {
                    if !matches!(token.get_token_type(), TokenType::Comma) {
                        return Err(ParserError::InvalidInstructionFormat)
                    }
                }
                State::Arg30 => {
                    match token.get_token_type() {
                        TokenType::Register =>  {
                            let value = encode_arg3(token.get_content())?;
                            next_bytes |= value as u32;
                        }
                        _ => return Err(ParserError::InvalidInstructionArgument(token.get_content().to_string()))
                    }
                }
                State::Arg33 => {
                    match token.get_token_type() {
                        TokenType::Register => { 
                            let value = encode_arg3(token.get_content())?; 
                            next_bytes |= (value << 3) as u32; 
                        }
                        _ => return Err(ParserError::InvalidInstructionArgument(token.get_content().to_string()))
                    }
                }
                State::Arg24 => {
                    match token.get_token_type() {
                        TokenType::Register => { 
                            let value = encode_arg2(token.get_content())?; 
                            next_bytes |= (value << 4) as u32; 
                        }
                        _ => return Err(ParserError::InvalidInstructionArgument(token.get_content().to_string()))
                    }
                }
                State::Immediate8 => next_bytes |= (parse_immediate(&token, &state) as u32) << 8,
                State::Immediate16 => {
                    if matches!(token.get_token_type(), TokenType::LabelValue) {
                        expect_label = true;
                    } else {
                        next_bytes |= (parse_immediate(&token, &state) as u32) << 8
                    }
                }
                State::Append => {
                    if !matches!(token.get_token_type(), TokenType::NewLine) {
                        return Err(ParserError::MissingNewLine)
                    }

                    buffer.push(next_bytes as u8);
                    next_bytes >>= 8;
                    if expect_label {
                        buffer.push(0);
                        buffer.push(0);
                    } else {
                        while next_bytes > 0 {
                            buffer.push(next_bytes as u8);
                            next_bytes >>= 8;
                        }
                    }
                    state_queue.push_back(State::Search);

                    expect_label = false;
                }
            }

        } 
    }
    println!();
    Ok(buffer)
}

fn parse_immediate(token: &Token, state: &State) -> Result<u16, ParseError> {
    let value = token.get_content();
    if value.starts_with("0x") && value.ends_with('h') {
        return Err(ParserError::InvalidValueFormat(token.get_content().to_string()))
    }
    let trimmed = value.trim_start_matches("0x").trim_start_matches('0').trim_end_matches('h');
    if (matches!(state, State::Immediate8) && value.len() > 2) ||
        (matches!(state, State::Immediate16) && value.len() > 4) || value.len() == 0 {
        return Err(ParserError::InvalidValueFormat(token.get_content().to_string()))
    }
    parsed = u16::from_str_radix(trimmed, 16)
        .map_err(|_| ParserError::InvalidValueFormat(token.get_content().to_string()))?
}

fn encode_arg3(arg: &str) -> Result<u8, ParserError> {
    match arg {
        "b" => Ok(0),
        "c" => Ok(1),
        "d" => Ok(2),
        "e" => Ok(3),
        "h" => Ok(4),
        "l" => Ok(5),
        "m" => Ok(6),
        "a" => Ok(7),
        _ => Err(ParserError::InvalidInstructionArgument(arg.to_string())),
    }
}

fn encode_arg2(arg: &str) -> Result<u8, ParserError> {
    match arg {
        "b" => Ok(0),
        "d" => Ok(1),
        "h" => Ok(2),
        "sp" => Ok(3),
        _ => Err(ParserError::InvalidInstructionArgument(arg.to_string())),
    }
}

fn encode_inst(inst: &str) -> Result<(u8, Option<Vec<State>>), ParserError> {
    match inst {
        "mov" => Ok((0x40, Some(vec![State::Arg33, State::Comma, State::Arg30]))),
        "mvi" => Ok((0x06, Some(vec![State::Arg33, State::Comma, State::Immediate8]))),
        "lxi" => Ok((0x01, Some(vec![State::Arg24, State::Comma, State::Immediate16]))),
        "stax" => Ok((0x02, Some(vec![State::Arg24]))),
        "ldax" => Ok((0x0A, Some(vec![State::Arg24]))),
        "sta" => Ok((0x32, Some(vec![State::Immediate16]))),
        "lda" => Ok((0x3A, Some(vec![State::Immediate16]))),
        "shld" => Ok((0x22, Some(vec![State::Immediate16]))),
        "lhld" => Ok((0x2A, Some(vec![State::Immediate16]))),
        "xchg" => Ok((0xEB, None)),
        "push" => Ok((0xC5, Some(vec![State::Arg24]))),
        "pop" => Ok((0xC1, Some(vec![State::Arg24]))),
        "xthl" => Ok((0xE3, None)),
        "sphl" => Ok((0xF9, None)),
        "inx" => Ok((0x03, Some(vec![State::Arg24]))),
        "dcx" => Ok((0x0B, Some(vec![State::Arg24]))),
        "jmp" => Ok((0xC3, Some(vec![State::Immediate16]))),
        "jc" => Ok((0xDA, Some(vec![State::Immediate16]))),
        "jnc" => Ok((0xD2, Some(vec![State::Immediate16]))),
        "jz" => Ok((0xCA, Some(vec![State::Immediate16]))),
        "jnz" => Ok((0xC2, Some(vec![State::Immediate16]))),
        "jp" => Ok((0xF2, Some(vec![State::Immediate16]))),
        "jm" => Ok((0xFA, Some(vec![State::Immediate16]))),
        "jpe" => Ok((0xEA, Some(vec![State::Immediate16]))),
        "jpo" => Ok((0xE2, Some(vec![State::Immediate16]))),
        "pchl" => Ok((0xE9, None)),
        "call" => Ok((0xCD, Some(vec![State::Immediate16]))),
        "cc" => Ok((0xDC, Some(vec![State::Immediate16]))),
        "cnc" => Ok((0xD4, Some(vec![State::Immediate16]))),
        "cz" => Ok((0xCC, Some(vec![State::Immediate16]))),
        "cnz" => Ok((0xC4, Some(vec![State::Immediate16]))),
        "cp" => Ok((0xF4, Some(vec![State::Immediate16]))),
        "cm" => Ok((0xFC, Some(vec![State::Immediate16]))),
        "cpe" => Ok((0xEC, Some(vec![State::Immediate16]))),
        "cpo" => Ok((0xE4, Some(vec![State::Immediate16]))),
        "ret" => Ok((0xC9, None)),
        "rc" => Ok((0xD8, None)),
        "rnc" => Ok((0xD0, None)),
        "rz" => Ok((0xC8, None)),
        "rnz" => Ok((0xC0, None)),
        "rp" => Ok((0xF0, None)),
        "rm" => Ok((0xF8, None)),
        "rpe" => Ok((0xE8, None)),
        "rpo" => Ok((0xE0, None)),
        "rst" => Ok((0xC7, Some(vec![State::Arg33]))),
        "in" => Ok((0xDB, Some(vec![State::Immediate8]))),
        "out" => Ok((0xD3, Some(vec![State::Immediate8]))),
        "inr" => Ok((0x04, Some(vec![State::Arg33]))),
        "dcr" => Ok((0x05, Some(vec![State::Arg33]))),
        "add" => Ok((0x80, Some(vec![State::Arg30]))),
        "adc" => Ok((0x88, Some(vec![State::Arg30]))),
        "adi" => Ok((0xC6, Some(vec![State::Immediate8]))),
        "aci" => Ok((0xCE, Some(vec![State::Immediate8]))),
        "dad" => Ok((0x09, Some(vec![State::Arg24]))),
        "sub" => Ok((0x90, Some(vec![State::Arg30]))),
        "sbb" => Ok((0x98, Some(vec![State::Arg30]))),
        "sui" => Ok((0xD6, Some(vec![State::Immediate8]))),
        "sbi" => Ok((0xDE, Some(vec![State::Immediate8]))),
        "ana" => Ok((0xA0, Some(vec![State::Arg30]))),
        "xra" => Ok((0xA8, Some(vec![State::Arg30]))),
        "ora" => Ok((0xB0, Some(vec![State::Arg30]))),
        "cmp" => Ok((0xB8, Some(vec![State::Arg30]))),
        "ani" => Ok((0xE6, Some(vec![State::Immediate8]))),
        "xri" => Ok((0xEE, Some(vec![State::Immediate8]))),
        "ori" => Ok((0xF6, Some(vec![State::Immediate8]))),
        "cpi" => Ok((0xFE, Some(vec![State::Immediate8]))),
        "rlc" => Ok((0x07, None)),
        "rrc" => Ok((0x0F, None)),
        "ral" => Ok((0x17, None)),
        "rar" => Ok((0x1F, None)),
        "cma" => Ok((0x2F, None)),
        "stc" => Ok((0x37, None)),
        "cmc" => Ok((0x3F, None)),
        "daa" => Ok((0x27, None)),
        "ei" => Ok((0xFB, None)),
        "di" => Ok((0xF3, None)),
        "nop" => Ok((0x00, None)),
        "hlt" => Ok((0x76, None)),
        "rim" => Ok((0x20, None)),
        "sim" => Ok((0x30, None)),
        _ => Err(ParserError::UnknownInstruction(inst.to_string())),
    }
}
