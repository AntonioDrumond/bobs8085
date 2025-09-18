use crate::assembler::AssemblerError;
use super::token::*;
use std::collections::{HashMap, VecDeque};

/// Represents the parser's current expectation for the next token
#[derive(Debug)]
enum State {
    /// Expecting an instruction, label or new line
    Search,
    /// Expecting a comma
    Comma,
    /// Expecting a colon
    Colon,
    /// Expecting a source register (encoded in bits 0-2)
    SrcReg,
    /// Expecting a destination register (encoded in bits 3-5)
    DestReg,
    /// Expecting a register pair (encoded in bits 2-4)
    RegPair,
    /// Expecting an 8-bit immediate value
    Imm8,
    /// Expecting an 16-bit immediate value
    Imm16,
    /// Special RST immediate
    RstImm,
    /// Ready to append the assembled instruction to the buffer
    Append,
}

struct Parser {
    tokens: Vec<Token>,
    state_queue: VecDeque<State>,
    buffer: Vec<u8>,
    /// The paritally assembled bytes for the current instruction
    next_bytes: u32,
    /// The current memory address being written to
    address: u32,
    /// A map of label names to their memory addresses
    labels: HashMap<String, u32>,
    /// A list of locations in the buffer that need to be patched with label addresses
    unresolved_labels: Vec<(usize, String)>,
    /// Flag to indicate the need of allocating bytes for a label in the buffer
    alloc_lable: bool,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens,
            state_queue: VecDeque::from([State::Search]),
            next_bytes: 0,
            buffer: Vec::new(),
            address: 0xC000,
            labels: HashMap::new(),
            unresolved_labels: Vec::new(),
            alloc_lable: false,
        }
    }

    fn parse(mut self) -> Result<Vec<u8>, AssemblerError> {
        self.first_pass()?;
        self.second_pass()?;
        Ok(self.buffer)
    }

    fn first_pass(&mut self) -> Result<(), AssemblerError> {
        let tokens = self.tokens.clone();
        for token in tokens {
            if let Some(state) = self.state_queue.pop_front() {
                self.process_token(token, state)?;
            } else if !matches!(token, Token::NewLine) {
                return Err(AssemblerError::UnexpectedToken);
            }
        }
        Ok(())
    }

    fn second_pass(&mut self) -> Result<(), AssemblerError> {
        for (pos, label) in &self.unresolved_labels {
            if let Some(&label_address) = self.labels.get(label) {
                self.buffer[*pos] = label_address as u8;
                self.buffer[*pos + 1] = (label_address >> 8) as u8;
            } else {
                return Err(AssemblerError::LabelNotDefined(label.to_string()));
            }
        }
        Ok(())
    }

    /// Main state machine logic for processing a single token.
    fn process_token(&mut self, token: Token, state: State) -> Result<(), AssemblerError> {
        match state {
            State::Search => self.handle_search(token),
            State::Comma => {
                if !matches!(token, Token::Comma) {
                    return Err(AssemblerError::InvalidInstructionFormat);
                }
                Ok(())
            }
            State::Colon => {
                if !matches!(token, Token::Colon) {
                    return Err(AssemblerError::SyntaxError(format!("unknown name: {token}")))
                }
                Ok(())
            }
            State::SrcReg => self.handle_register_arg(token, 0, &encode_arg3),
            State::DestReg => self.handle_register_arg(token, 3, &encode_arg3),
            State::RegPair => self.handle_register_arg(token, 4, &encode_arg2),
            State::Imm8 => self.handle_immediate(token, State::Imm8),
            State::Imm16 => self.handle_immediate(token, State::Imm16),
            State::RstImm => self.handle_register_arg(token, 3, &parse_arg),
            State::Append => self.handle_append(token),
        }
    }

    fn handle_search(&mut self, token: Token) -> Result<(), AssemblerError> {
        match token {
            Token::Name(content) => {
                if let Some((op, states_opt)) = encode_inst(&content) {
                    if let Some(states) = states_opt {
                        self.state_queue.extend(states);
                    }
                    self.next_bytes = op as u32;
                    self.state_queue.push_back(State::Append);
                } else {
                    self.labels.insert(content, self.address);
                    self.state_queue.push_back(State::Colon);
                }
            }
            Token::NewLine => self.state_queue.push_back(State::Search),
            _ => return Err(AssemblerError::UnknownName(token.to_string())),
        }
        Ok(())
    }

    fn handle_register_arg(
        &mut self,
        token: Token,
        shift: u32,
        encoder: &dyn Fn(&str) -> Result<u8, AssemblerError>,
    ) -> Result<(), AssemblerError> {
        if let Token::Register(content) = token {
            let value = encoder(&content)?;
            self.next_bytes |= (value as u32) << shift;
            return Ok(());
        } else {
            return Err(AssemblerError::InvalidInstructionArgument(token.to_string()));
        }
    }

    fn handle_immediate(&mut self, token: Token, state: State) -> Result<(), AssemblerError> {
        match token {
            Token::LabelValue(content) => {
                if matches!(state, State::Imm16) {
                    self.unresolved_labels
                        .push((self.buffer.len() + 1, content));
                    self.alloc_lable = true;
                    self.next_bytes |= 0;
                } else {
                    return Err(AssemblerError::InvalidInstructionArgument(content));
                }
            }
            Token::Value(content) => {
                let value = parse_immediate(&content)?;
                let max_val = if matches!(state, State::Imm8) {
                    0xFF
                } else {
                    0xFFFF
                };
                if value > max_val {
                    return Err(AssemblerError::InvalidValueFormat(content));
                }
                self.next_bytes |= (value as u32) << 8;
            }
            _ => {
                return Err(AssemblerError::InvalidInstructionArgument(token.to_string()));
            }
        }
        Ok(())
    }

    fn handle_append(&mut self, token: Token) -> Result<(), AssemblerError> {
        if !matches!(token, Token::NewLine) {
            return Err(AssemblerError::MissingNewLine);
        }

        let old_len = self.buffer.len();

        self.buffer.push(self.next_bytes as u8);
        self.next_bytes >>= 8;
        if self.alloc_lable {
            self.buffer.push(0);
            self.buffer.push(0);
        } else {
            while self.next_bytes > 0 {
                self.buffer.push(self.next_bytes as u8);
                self.next_bytes >>= 8;
            }
        }

        self.alloc_lable = false;
        self.address += (self.buffer.len() - old_len) as u32;
        self.state_queue.push_back(State::Search);
        Ok(())
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<u8>, AssemblerError> {
    Parser::new(tokens).parse()
}

fn parse_immediate(value: &str) -> Result<u16, AssemblerError> {
    u16::from_str_radix(value, 16).map_err(|_| AssemblerError::InvalidValueFormat(value.to_string()))
}

fn parse_arg(value: &str) -> Result<u8, AssemblerError> {
    u8::from_str_radix(value, 10).map_err(|_| AssemblerError::InvalidValueFormat(value.to_string()))
}

fn encode_arg3(arg: &str) -> Result<u8, AssemblerError> {
    match arg {
        "b" => Ok(0),
        "c" => Ok(1),
        "d" => Ok(2),
        "e" => Ok(3),
        "h" => Ok(4),
        "l" => Ok(5),
        "m" => Ok(6),
        "a" => Ok(7),
        _ => Err(AssemblerError::UnknownRegister(arg.to_string())),
    }
}

fn encode_arg2(arg: &str) -> Result<u8, AssemblerError> {
    match arg {
        "b" => Ok(0),
        "d" => Ok(1),
        "h" => Ok(2),
        "sp" => Ok(3),
        _ => Err(AssemblerError::UnknownRegister(arg.to_string())),
    }
}

fn encode_inst(inst: &str) -> Option<(u8, Option<Vec<State>>)> {
    use State::{Comma, DestReg, Imm8, Imm16, RegPair, RstImm, SrcReg};
    match inst {
        "mov" => Some((0x40, Some(vec![DestReg, Comma, SrcReg]))),
        "mvi" => Some((0x06, Some(vec![DestReg, Comma, Imm8]))),
        "lxi" => Some((0x01, Some(vec![RegPair, Comma, Imm16]))),
        "stax" => Some((0x02, Some(vec![RegPair]))),
        "ldax" => Some((0x0A, Some(vec![RegPair]))),
        "sta" => Some((0x32, Some(vec![Imm16]))),
        "lda" => Some((0x3A, Some(vec![Imm16]))),
        "shld" => Some((0x22, Some(vec![Imm16]))),
        "lhld" => Some((0x2A, Some(vec![Imm16]))),
        "xchg" => Some((0xEB, None)),
        "push" => Some((0xC5, Some(vec![RegPair]))),
        "pop" => Some((0xC1, Some(vec![RegPair]))),
        "xthl" => Some((0xE3, None)),
        "sphl" => Some((0xF9, None)),
        "inx" => Some((0x03, Some(vec![RegPair]))),
        "dcx" => Some((0x0B, Some(vec![RegPair]))),
        "jmp" => Some((0xC3, Some(vec![Imm16]))),
        "jc" => Some((0xDA, Some(vec![Imm16]))),
        "jnc" => Some((0xD2, Some(vec![Imm16]))),
        "jz" => Some((0xCA, Some(vec![Imm16]))),
        "jnz" => Some((0xC2, Some(vec![Imm16]))),
        "jp" => Some((0xF2, Some(vec![Imm16]))),
        "jm" => Some((0xFA, Some(vec![Imm16]))),
        "jpe" => Some((0xEA, Some(vec![Imm16]))),
        "jpo" => Some((0xE2, Some(vec![Imm16]))),
        "pchl" => Some((0xE9, None)),
        "call" => Some((0xCD, Some(vec![Imm16]))),
        "cc" => Some((0xDC, Some(vec![Imm16]))),
        "cnc" => Some((0xD4, Some(vec![Imm16]))),
        "cz" => Some((0xCC, Some(vec![Imm16]))),
        "cnz" => Some((0xC4, Some(vec![Imm16]))),
        "cp" => Some((0xF4, Some(vec![Imm16]))),
        "cm" => Some((0xFC, Some(vec![Imm16]))),
        "cpe" => Some((0xEC, Some(vec![Imm16]))),
        "cpo" => Some((0xE4, Some(vec![Imm16]))),
        "ret" => Some((0xC9, None)),
        "rc" => Some((0xD8, None)),
        "rnc" => Some((0xD0, None)),
        "rz" => Some((0xC8, None)),
        "rnz" => Some((0xC0, None)),
        "rp" => Some((0xF0, None)),
        "rm" => Some((0xF8, None)),
        "rpe" => Some((0xE8, None)),
        "rpo" => Some((0xE0, None)),
        "rst" => Some((0xC7, Some(vec![RstImm]))),
        "in" => Some((0xDB, Some(vec![Imm8]))),
        "out" => Some((0xD3, Some(vec![Imm8]))),
        "inr" => Some((0x04, Some(vec![DestReg]))),
        "dcr" => Some((0x05, Some(vec![DestReg]))),
        "add" => Some((0x80, Some(vec![SrcReg]))),
        "adc" => Some((0x88, Some(vec![SrcReg]))),
        "adi" => Some((0xC6, Some(vec![Imm8]))),
        "aci" => Some((0xCE, Some(vec![Imm8]))),
        "dad" => Some((0x09, Some(vec![RegPair]))),
        "sub" => Some((0x90, Some(vec![SrcReg]))),
        "sbb" => Some((0x98, Some(vec![SrcReg]))),
        "sui" => Some((0xD6, Some(vec![Imm8]))),
        "sbi" => Some((0xDE, Some(vec![Imm8]))),
        "ana" => Some((0xA0, Some(vec![SrcReg]))),
        "xra" => Some((0xA8, Some(vec![SrcReg]))),
        "ora" => Some((0xB0, Some(vec![SrcReg]))),
        "cmp" => Some((0xB8, Some(vec![SrcReg]))),
        "ani" => Some((0xE6, Some(vec![Imm8]))),
        "xri" => Some((0xEE, Some(vec![Imm8]))),
        "ori" => Some((0xF6, Some(vec![Imm8]))),
        "cpi" => Some((0xFE, Some(vec![Imm8]))),
        "rlc" => Some((0x07, None)),
        "rrc" => Some((0x0F, None)),
        "ral" => Some((0x17, None)),
        "rar" => Some((0x1F, None)),
        "cma" => Some((0x2F, None)),
        "stc" => Some((0x37, None)),
        "cmc" => Some((0x3F, None)),
        "daa" => Some((0x27, None)),
        "ei" => Some((0xFB, None)),
        "di" => Some((0xF3, None)),
        "nop" => Some((0x00, None)),
        "hlt" => Some((0x76, None)),
        "rim" => Some((0x20, None)),
        "sim" => Some((0x30, None)),
        _ => None,
    }
}
