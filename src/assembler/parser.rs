use std::collections::VecDeque;

use super::token::*;

pub(super) enum ParserError {
    UnknownName,
    UnknownInstruction,
    InvalidInstructionArgument,
    InvalidValueFormat,
    UnknownRegister,
}

enum State {
    Search,
    Arg30,
    Arg33,
    Arg24,
    Immediate,
}

pub(super) fn parse(tokens: Vec<Box<Token>>) -> Result<(), ParserError> {
    let mut state_queue: VecDeque<State> = VecDeque::from([State::Search]);
    let mut buffer: Vec<u8> = Vec::default();
    let mut next_mc: u16 = 0;
    for token in tokens {
        if let Some(state) = state_queue.pop_front() {
            match state {
                State::Search => match token.get_token_type() {
                    TokenType::Instruction => {
                        if let (op, Some(states)) = encode_inst(token.get_content())? {
                            next_mc = op as u16;
                            state_queue.extend(states);
                        }
                    }
                    TokenType::Label => todo!("labels are yet to be supported"),
                    _ => return Err(ParserError::UnknownName),
                },
                State::Arg30 => {
                    match token.get_token_type() {
                        TokenType::Register =>  {
                            let value = encode_arg3(token.get_content())?;
                            next_mc |= value as u16;
                        }
                        _ => return Err(ParserError::InvalidInstructionArgument)
                    }
                }
                State::Arg33 => {
                    match token.get_token_type() {
                        TokenType::Register => { 
                            let value = encode_arg3(token.get_content())?; 
                            next_mc |= (value << 3) as u16; 
                        }
                        _ => return Err(ParserError::InvalidInstructionArgument)
                    }
                }
                State::Arg24 => {
                    match token.get_token_type() {
                        TokenType::Register => { 
                            let value = encode_arg2(token.get_content())?; 
                            next_mc |= (value << 4) as u16; 
                        }
                        _ => return Err(ParserError::InvalidInstructionArgument)
                    }
                }
                State::Immediate => {
                    let value = token.get_content();
                    if(value.starts_with("0x") && value.ends_with('h')) {
                        return Err(ParserError::InvalidValueFormat)
                    }
                    let trimmed = value.trim_start_matches("0x").trim_start_matches('0').trim_end_matches('h');
                    if value.len() > 4 || value.len() == 0 {
                        return Err(ParserError::InvalidValueFormat)
                    }
                    next_mc = u16::from_str_radix(trimmed, 16).map_err(|_| ParserError::InvalidValueFormat)?;
                }
            }

            if state_queue.is_empty() {
                if next_mc < 0x0100 {
                    buffer.push(next_mc as u8);
                } else {
                    buffer.push(next_mc as u8);
                    buffer.push((next_mc >> 8) as u8);
                }
                state_queue.push_back(State::Search);
            }
        } 
    }
    Ok(())
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
        _ => Err(ParserError::InvalidInstructionArgument),
    }
}

fn encode_arg2(arg: &str) -> Result<u8, ParserError> {
    match arg {
        "b" => Ok(0),
        "d" => Ok(1),
        "h" => Ok(2),
        "sp" => Ok(3),
        _ => Err(ParserError::InvalidInstructionArgument),
    }
}

fn encode_inst(inst: &str) -> Result<(u8, Option<Vec<State>>), ParserError> {
    match inst {
        "mov" => Ok((0x40, Some(vec![State::Arg33, State::Arg30]))),
        "mvi" => Ok((0x06, Some(vec![State::Arg33, State::Immediate]))),
        "lxi" => Ok((0x01, Some(vec![State::Arg24]))),
        "stax" => Ok((0x02, Some(vec![State::Arg24]))),
        "ldax" => Ok((0x0A, Some(vec![State::Arg24]))),
        "sta" => Ok((0x32, Some(vec![State::Immediate]))),
        "lda" => Ok((0x3A, Some(vec![State::Immediate]))),
        "shld" => Ok((0x22, Some(vec![State::Immediate]))),
        "lhld" => Ok((0x2A, Some(vec![State::Immediate]))),
        "xchg" => Ok((0xEB, None)),
        "push" => Ok((0xC5, Some(vec![State::Arg24]))),
        "pop" => Ok((0xC1, Some(vec![State::Arg24]))),
        "xthl" => Ok((0xE3, None)),
        "sphl" => Ok((0xF9, None)),
        "inx" => Ok((0x03, Some(vec![State::Arg24]))),
        "dcx" => Ok((0x0B, Some(vec![State::Arg24]))),
        "jmp" => Ok((0xC3, Some(vec![State::Immediate]))),
        "jc" => Ok((0xDA, Some(vec![State::Immediate]))),
        "jnc" => Ok((0xD2, Some(vec![State::Immediate]))),
        "jz" => Ok((0xCA, Some(vec![State::Immediate]))),
        "jnz" => Ok((0xC2, Some(vec![State::Immediate]))),
        "jp" => Ok((0xF2, Some(vec![State::Immediate]))),
        "jm" => Ok((0xFA, Some(vec![State::Immediate]))),
        "jpe" => Ok((0xEA, Some(vec![State::Immediate]))),
        "jpo" => Ok((0xE2, Some(vec![State::Immediate]))),
        "pchl" => Ok((0xE9, None)),
        "call" => Ok((0xCD, Some(vec![State::Immediate]))),
        "cc" => Ok((0xDC, Some(vec![State::Immediate]))),
        "cnc" => Ok((0xD4, Some(vec![State::Immediate]))),
        "cz" => Ok((0xCC, Some(vec![State::Immediate]))),
        "cnz" => Ok((0xC4, Some(vec![State::Immediate]))),
        "cp" => Ok((0xF4, Some(vec![State::Immediate]))),
        "cm" => Ok((0xFC, Some(vec![State::Immediate]))),
        "cpe" => Ok((0xEC, Some(vec![State::Immediate]))),
        "cpo" => Ok((0xE4, Some(vec![State::Immediate]))),
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
        "in" => Ok((0xDB, Some(vec![State::Immediate]))),
        "out" => Ok((0xD3, Some(vec![State::Immediate]))),
        "inr" => Ok((0x04, Some(vec![State::Arg33]))),
        "dcr" => Ok((0x05, Some(vec![State::Arg33]))),
        "add" => Ok((0x80, Some(vec![State::Arg30]))),
        "adc" => Ok((0x88, Some(vec![State::Arg30]))),
        "adi" => Ok((0xC6, Some(vec![State::Immediate]))),
        "aci" => Ok((0xCE, Some(vec![State::Immediate]))),
        "dad" => Ok((0x09, Some(vec![State::Arg24]))),
        "sub" => Ok((0x90, Some(vec![State::Arg30]))),
        "sbb" => Ok((0x98, Some(vec![State::Arg30]))),
        "sui" => Ok((0xD6, Some(vec![State::Immediate]))),
        "sbi" => Ok((0xDE, Some(vec![State::Immediate]))),
        "ana" => Ok((0xA0, Some(vec![State::Arg30]))),
        "xra" => Ok((0xA8, Some(vec![State::Arg30]))),
        "ora" => Ok((0xB0, Some(vec![State::Arg30]))),
        "cmp" => Ok((0xB8, Some(vec![State::Arg30]))),
        "ani" => Ok((0xE6, Some(vec![State::Immediate]))),
        "xri" => Ok((0xEE, Some(vec![State::Immediate]))),
        "ori" => Ok((0xF6, Some(vec![State::Immediate]))),
        "cpi" => Ok((0xFE, Some(vec![State::Immediate]))),
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
        _ => Err(ParserError::UnknownInstruction),
    }
}
