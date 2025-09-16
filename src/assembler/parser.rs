use super::token::*;
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub enum ParserError {
    UnknownName(String),
    UnknownInstruction(String),
    InvalidInstructionArgument(String),
    InvalidInstructionFormat,
    InvalidValueFormat(String),
    UnknownRegister(String),
    LabelNotDefined(String),
    MissingNewLine,
    UnexpectedToken,
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownName(fragment) => write!(f, "unknown name \"{}\"", fragment),
            Self::UnknownInstruction(fragment) => {
                write!(f, "unknown instruction \"{}\"", fragment)
            }
            Self::InvalidInstructionArgument(fragment) => {
                write!(f, "invalid instruction argument \"{}\"", fragment)
            }
            Self::InvalidInstructionFormat => {
                write!(f, "invalid instruction format, expected comma")
            }
            Self::InvalidValueFormat(fragment) => {
                write!(f, "invalid value format \"{}\"", fragment)
            }
            Self::UnknownRegister(fragment) => {
                write!(f, "unknown register \"{}\"", fragment)
            }
            Self::LabelNotDefined(fragment) => {
                write!(f, "label not defined \"{}\"", fragment)
            }
            Self::MissingNewLine => write!(f, "missing new line after instruction"),
            Self::UnexpectedToken => write!(f, "unexpected token at end of input"),
        }
    }
}

impl std::error::Error for ParserError {}

/// Represents the parser's current expectation for the next token
#[derive(Debug)]
enum State {
    /// Expecting an instruction, label or new line
    Search,
    /// Expecting a comma
    Comma,
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
        }
    }

    fn parse(mut self) -> Result<Vec<u8>, ParserError> {
        self.first_pass()?;
        self.second_pass()?;
        Ok(self.buffer)
    }

    fn first_pass(&mut self) -> Result<(), ParserError> {
        let tokens = self.tokens.clone();
        for token in tokens {
            if let Some(state) = self.state_queue.pop_front() {
                self.process_token(token, state)?;
            } else if !matches!(token, Token::NewLine) {
                return Err(ParserError::UnexpectedToken);
            }
        }
        Ok(())
    }

    fn second_pass(&mut self) -> Result<(), ParserError> {
        for (pos, label) in &self.unresolved_labels {
            if let Some(&label_address) = self.labels.get(label) {
                self.buffer[*pos] = label_address as u8;
                self.buffer[*pos + 1] = (label_address >> 8) as u8;
            } else {
                return Err(ParserError::LabelNotDefined(label.to_string()));
            }
        }
        Ok(())
    }

    /// Main state machine logic for processing a single token.
    fn process_token(&mut self, token: Token, state: State) -> Result<(), ParserError> {
        match state {
            State::Search => self.handle_search(token),
            State::Comma => {
                if !matches!(token, Token::Comma) {
                    return Err(ParserError::InvalidInstructionFormat);
                }
                Ok(())
            }
            State::SrcReg => self.handle_register_arg(token, 0, &encode_arg3),
            State::DestReg => self.handle_register_arg(token, 3, &encode_arg3),
            State::RegPair => self.handle_register_arg(token, 4, &encode_arg2),
            State::Imm8 => self.handle_immediate(token, State::Imm8),
            State::Imm16 => self.handle_immediate(token, State::Imm16),
            State::Append => self.handle_append(token),
        }
    }

    fn handle_search(&mut self, token: Token) -> Result<(), ParserError> {
        match token {
            Token::Instruction(content) => {
                let (op, states_opt) = encode_inst(&content)?;
                if let Some(states) = states_opt {
                    self.state_queue.extend(states);
                }
                self.next_bytes = op as u32;
                self.state_queue.push_back(State::Append);
            }
            Token::LabelDeclaration(content) => {
                self.labels.insert(content, self.address);
                self.state_queue.push_back(State::Search);
            }
            Token::NewLine => self.state_queue.push_back(State::Search),
            _ => return Err(ParserError::UnknownName(token.to_string())),
        }
        Ok(())
    }

    fn handle_register_arg(
        &mut self,
        token: Token,
        shift: u32,
        encoder: &dyn Fn(&str) -> Result<u8, ParserError>,
    ) -> Result<(), ParserError> {
        if let Token::Register(content) = token {
            let value = encoder(&content)?;
            self.next_bytes |= (value as u32) << shift;
            return Ok(());
        } else {
            return Err(ParserError::InvalidInstructionArgument(token.to_string()));
        }
    }

    fn handle_immediate(&mut self, token: Token, state: State) -> Result<(), ParserError> {
        match token {
            Token::LabelValue(content) => {
                if matches!(state, State::Imm16) {
                    self.unresolved_labels
                        .push((self.buffer.len() + 1, content));
                    self.next_bytes |= 0;
                } else {
                    return Err(ParserError::InvalidInstructionArgument(content));
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
                    return Err(ParserError::InvalidValueFormat(content));
                }
                self.next_bytes |= (value as u32) << 8;
            }
            _ => {
                return Err(ParserError::InvalidInstructionArgument(token.to_string()));
            }
        }
        Ok(())
    }

    fn handle_append(&mut self, token: Token) -> Result<(), ParserError> {
        if !matches!(token, Token::NewLine) {
            return Err(ParserError::MissingNewLine);
        }

        let old_len = self.buffer.len();

        self.buffer.push(self.next_bytes as u8);
        self.next_bytes >>= 8;
        while self.next_bytes > 0 {
            self.buffer.push(self.next_bytes as u8);
            self.next_bytes >>= 8;
        }

        self.address += (self.buffer.len() - old_len) as u32;
        self.state_queue.push_back(State::Search);
        Ok(())
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<u8>, ParserError> {
    Parser::new(tokens).parse()
}

fn parse_immediate(value: &str) -> Result<u16, ParserError> {
    u16::from_str_radix(value, 16).map_err(|_| ParserError::InvalidValueFormat(value.to_string()))
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
        _ => Err(ParserError::UnknownRegister(arg.to_string())),
    }
}

fn encode_arg2(arg: &str) -> Result<u8, ParserError> {
    match arg {
        "b" => Ok(0),
        "d" => Ok(1),
        "h" => Ok(2),
        "sp" => Ok(3),
        _ => Err(ParserError::UnknownRegister(arg.to_string())),
    }
}

fn encode_inst(inst: &str) -> Result<(u8, Option<Vec<State>>), ParserError> {
    use State::{Comma, DestReg, Imm8, Imm16, RegPair, SrcReg};
    match inst {
        "mov" => Ok((0x40, Some(vec![DestReg, Comma, SrcReg]))),
        "mvi" => Ok((0x06, Some(vec![DestReg, Comma, Imm8]))),
        "lxi" => Ok((0x01, Some(vec![RegPair, Comma, Imm16]))),
        "stax" => Ok((0x02, Some(vec![RegPair]))),
        "ldax" => Ok((0x0A, Some(vec![RegPair]))),
        "sta" => Ok((0x32, Some(vec![Imm16]))),
        "lda" => Ok((0x3A, Some(vec![Imm16]))),
        "shld" => Ok((0x22, Some(vec![Imm16]))),
        "lhld" => Ok((0x2A, Some(vec![Imm16]))),
        "xchg" => Ok((0xEB, None)),
        "push" => Ok((0xC5, Some(vec![RegPair]))),
        "pop" => Ok((0xC1, Some(vec![RegPair]))),
        "xthl" => Ok((0xE3, None)),
        "sphl" => Ok((0xF9, None)),
        "inx" => Ok((0x03, Some(vec![RegPair]))),
        "dcx" => Ok((0x0B, Some(vec![RegPair]))),
        "jmp" => Ok((0xC3, Some(vec![Imm16]))),
        "jc" => Ok((0xDA, Some(vec![Imm16]))),
        "jnc" => Ok((0xD2, Some(vec![Imm16]))),
        "jz" => Ok((0xCA, Some(vec![Imm16]))),
        "jnz" => Ok((0xC2, Some(vec![Imm16]))),
        "jp" => Ok((0xF2, Some(vec![Imm16]))),
        "jm" => Ok((0xFA, Some(vec![Imm16]))),
        "jpe" => Ok((0xEA, Some(vec![Imm16]))),
        "jpo" => Ok((0xE2, Some(vec![Imm16]))),
        "pchl" => Ok((0xE9, None)),
        "call" => Ok((0xCD, Some(vec![Imm16]))),
        "cc" => Ok((0xDC, Some(vec![Imm16]))),
        "cnc" => Ok((0xD4, Some(vec![Imm16]))),
        "cz" => Ok((0xCC, Some(vec![Imm16]))),
        "cnz" => Ok((0xC4, Some(vec![Imm16]))),
        "cp" => Ok((0xF4, Some(vec![Imm16]))),
        "cm" => Ok((0xFC, Some(vec![Imm16]))),
        "cpe" => Ok((0xEC, Some(vec![Imm16]))),
        "cpo" => Ok((0xE4, Some(vec![Imm16]))),
        "ret" => Ok((0xC9, None)),
        "rc" => Ok((0xD8, None)),
        "rnc" => Ok((0xD0, None)),
        "rz" => Ok((0xC8, None)),
        "rnz" => Ok((0xC0, None)),
        "rp" => Ok((0xF0, None)),
        "rm" => Ok((0xF8, None)),
        "rpe" => Ok((0xE8, None)),
        "rpo" => Ok((0xE0, None)),
        "rst" => Ok((0xC7, Some(vec![DestReg]))),
        "in" => Ok((0xDB, Some(vec![Imm8]))),
        "out" => Ok((0xD3, Some(vec![Imm8]))),
        "inr" => Ok((0x04, Some(vec![DestReg]))),
        "dcr" => Ok((0x05, Some(vec![DestReg]))),
        "add" => Ok((0x80, Some(vec![SrcReg]))),
        "adc" => Ok((0x88, Some(vec![SrcReg]))),
        "adi" => Ok((0xC6, Some(vec![Imm8]))),
        "aci" => Ok((0xCE, Some(vec![Imm8]))),
        "dad" => Ok((0x09, Some(vec![RegPair]))),
        "sub" => Ok((0x90, Some(vec![SrcReg]))),
        "sbb" => Ok((0x98, Some(vec![SrcReg]))),
        "sui" => Ok((0xD6, Some(vec![Imm8]))),
        "sbi" => Ok((0xDE, Some(vec![Imm8]))),
        "ana" => Ok((0xA0, Some(vec![SrcReg]))),
        "xra" => Ok((0xA8, Some(vec![SrcReg]))),
        "ora" => Ok((0xB0, Some(vec![SrcReg]))),
        "cmp" => Ok((0xB8, Some(vec![SrcReg]))),
        "ani" => Ok((0xE6, Some(vec![Imm8]))),
        "xri" => Ok((0xEE, Some(vec![Imm8]))),
        "ori" => Ok((0xF6, Some(vec![Imm8]))),
        "cpi" => Ok((0xFE, Some(vec![Imm8]))),
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
