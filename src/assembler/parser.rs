use super::token::*;
use crate::assembler::AssemblerError;
use core::slice::Iter;
use std::collections::{HashMap, VecDeque};
use std::iter::Peekable;

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
    Append(u8),
}

struct Parser<'a> {
    iterator: Peekable<Iter<'a, Token>>,
    last_token: Option<&'a Token>,
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

impl<'a> Parser<'a> {
    fn new(tokens: &'a Vec<Token>) -> Self {
        Parser {
            iterator: tokens.iter().peekable(),
            last_token: None,
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
        while let Some(token) = self.iterator.next() {
            if let Some(state) = self.state_queue.pop_front() {
                self.process_token(token, state)?;
                self.last_token = Some(token);
            } else if !matches!(token.token_type(), TokenType::NewLine) {
                return Err(AssemblerError::SemanticError(
                    format!("expected new line, found \"{}\"", Token::type_of(&token)),
                    Some(token.line()),
                    Some(token.column()),
                ));
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
                return Err(AssemblerError::SemanticError(
                    format!("unknown label \"{}\"", label.to_string()),
                    None,
                    None,
                ));
            }
        }
        Ok(())
    }

    /// Main state machine logic for processing a single token.
    fn process_token(&mut self, token: &Token, state: State) -> Result<(), AssemblerError> {
        match state {
            State::Search => self.handle_search(token)?,
            State::Comma => {
                if !matches!(token.token_type(), TokenType::Comma) {
                    return Err(AssemblerError::SyntaxError(
                        format!("expected \",\", found \"{}\"", token.lexeme()),
                        Some(token.line()),
                        Some(token.column()),
                    ));
                }
                return Ok(());
            }
            State::Colon => {
                if !matches!(token.token_type(), TokenType::Colon) {
                    return Err(AssemblerError::SyntaxError(
                        format!(
                            "expected \":\" after label name, found \"{}\"",
                            token.lexeme()
                        ),
                        Some(token.line()),
                        Some(token.column()),
                    ));
                }
                self.state_queue.push_back(State::Search);
                return Ok(());
            }
            State::SrcReg => self.handle_register_arg(token, 0, &encode_arg3)?,
            State::DestReg => self.handle_register_arg(token, 3, &encode_arg3)?,
            State::RegPair => self.handle_register_arg(token, 4, &encode_arg2)?,
            State::Imm8 => self.handle_immediate(token, State::Imm8)?,
            State::Imm16 => self.handle_immediate(token, State::Imm16)?,
            State::RstImm => self.handle_register_arg(token, 3, &parse_arg)?,
            State::Append(bytes) => self.handle_append(token, bytes)?,
        }
        Ok(())
    }

    fn handle_search(&mut self, token: &Token) -> Result<(), AssemblerError> {
        match token.token_type() {
            TokenType::Name => {
                if let Some((op, states)) = encode_inst(token.lexeme()) {
                    if let Some(next_tok) = self.iterator.peek() {
                        if matches!(next_tok.token_type(), TokenType::Colon) {
                            return Err(AssemblerError::SemanticError(
                                format!(
                                    "label name \"{}\" is a reserved mnemonic, nice try nerd",
                                    token.lexeme(),
                                ),
                                Some(token.line()),
                                Some(token.column()),
                            ));
                        }
                    }
                    self.state_queue.extend(states);
                    self.next_bytes = op as u32;
                } else {
                    self.labels.insert(token.lexeme().to_string(), self.address);
                    self.state_queue.push_back(State::Colon);
                }
            }
            TokenType::HexLiteral => {
                if let Some(next_tok) = self.iterator.peek() {
                    if matches!(next_tok.token_type(), TokenType::Colon) {
                        return Err(AssemblerError::SyntaxError(
                            format!(
                                "label name \"{}\" fits as a valid hex literal, choose better names",
                                token.lexeme()
                            ),
                            Some(token.line()),
                            Some(token.column()),
                        ));
                    }
                }
            }
            TokenType::NewLine => self.state_queue.push_back(State::Search),
            _ => {
                return Err(AssemblerError::SemanticError(
                    format!(
                        "expected an instruction or label name, found {} \"{}\"",
                        Token::type_of(token),
                        token.lexeme()
                    ),
                    Some(token.line()),
                    Some(token.column()),
                ));
            }
        }
        Ok(())
    }

    fn handle_register_arg(
        &mut self,
        token: &Token,
        shift: u32,
        encoder: &dyn Fn(&Token) -> Result<u8, AssemblerError>,
    ) -> Result<(), AssemblerError> {
        if matches!(token.token_type(), TokenType::Name) {
            let register = encoder(token)?;
            self.next_bytes |= (register as u32) << shift;
            return Ok(());
        } else {
            return Err(AssemblerError::SyntaxError(
                format!(
                    "expected a register, found {} \"{}\"",
                    Token::type_of(&token),
                    token.lexeme()
                ),
                Some(token.line()),
                Some(token.column()),
            ));
        }
    }

    fn handle_immediate(&mut self, token: &Token, state: State) -> Result<(), AssemblerError> {
        match token.token_type() {
            TokenType::Name => {
                if matches!(state, State::Imm16) {
                    self.unresolved_labels
                        .push((self.buffer.len() + 1, token.lexeme().to_string()));
                    self.alloc_lable = true;
                    self.next_bytes |= 0;
                } else {
                    return Err(AssemblerError::SemanticError(
                        format!(
                            "expected a hex value, found {} \"{}\"",
                            Token::type_of(token),
                            token.lexeme()
                        ),
                        Some(token.line()),
                        Some(token.column()),
                    ));
                }
            }
            TokenType::HexLiteral => {
                let max_val: u16 = if matches!(state, State::Imm8) {
                    0xFF
                } else {
                    0xFFFF
                };
                let treated = token
                    .lexeme()
                    .strip_prefix("0x")
                    .or_else(|| token.lexeme().strip_suffix('H'))
                    .or_else(|| token.lexeme().strip_suffix('h'))
                    .unwrap();
                let val = match u16::from_str_radix(treated, 16) {
                    Ok(v) => v,
                    Err(_) => {
                        return Err(AssemblerError::SyntaxError(
                            format!("invalid hex literal \"{}\"", token.lexeme()),
                            Some(token.line()),
                            Some(token.column()),
                        ));
                    }
                };
                if val > max_val {
                    return Err(AssemblerError::SemanticError(
                        format!(
                            "value {} should be at most {}",
                            token.lexeme(),
                            format!("0x{:x}", max_val)
                        ),
                        Some(token.line()),
                        Some(token.column()),
                    ));
                }
                self.next_bytes |= (val as u32) << 8;
            }
            _ => {
                return Err(AssemblerError::SemanticError(
                    format!(
                        "expected hex value or label name, found \"{}\"",
                        token.lexeme()
                    ),
                    Some(token.line()),
                    Some(token.column()),
                ));
            }
        }
        Ok(())
    }

    fn handle_append(&mut self, token: &Token, bytes: u8) -> Result<(), AssemblerError> {
        if !matches!(token.token_type(), TokenType::NewLine) {
            return Err(AssemblerError::SyntaxError(
                format!("expected new line after instruction"),
                self.last_token.map(|t| t.line()),
                self.last_token.map(|t| t.column()),
            ));
        }

        let old_len = self.buffer.len();

        self.buffer.push(self.next_bytes as u8);
        self.next_bytes >>= 8;
        if self.alloc_lable {
            self.buffer.push(0);
            self.buffer.push(0);
        } else {
            for _ in 0..(bytes - 1) {
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

pub fn parse(tokens: &Vec<Token>) -> Result<Vec<u8>, AssemblerError> {
    Parser::new(tokens).parse()
}

fn parse_arg(token: &Token) -> Result<u8, AssemblerError> {
    match u8::from_str_radix(token.lexeme(), 10) {
        Ok(arg) => {
            if arg > 0b111 {
                return Err(AssemblerError::SemanticError(
                    format!("unknown RST argument \"{}\"", arg),
                    Some(token.line()),
                    Some(token.column()),
                ));
            }
            return Ok(arg);
        }
        Err(_e) => Err(AssemblerError::SemanticError(
            format!("unknown RST argument \"{}\"", token.lexeme()),
            Some(token.line()),
            Some(token.column()),
        )),
    }
}

fn encode_arg3(token: &Token) -> Result<u8, AssemblerError> {
    match token.lexeme().to_lowercase().as_str() {
        "b" => Ok(0),
        "c" => Ok(1),
        "d" => Ok(2),
        "e" => Ok(3),
        "h" => Ok(4),
        "l" => Ok(5),
        "m" => Ok(6),
        "a" => Ok(7),
        _ => Err(AssemblerError::SemanticError(
            format!("unknown register \"{}\"", token.lexeme()),
            Some(token.line()),
            Some(token.column()),
        )),
    }
}

fn encode_arg2(token: &Token) -> Result<u8, AssemblerError> {
    match token.lexeme().to_lowercase().as_str() {
        "b" => Ok(0),
        "d" => Ok(1),
        "h" => Ok(2),
        "sp" => Ok(3),
        _ => Err(AssemblerError::SemanticError(
            format!("unknown register \"{}\"", token.lexeme()),
            Some(token.line()),
            Some(token.column()),
        )),
    }
}

fn encode_inst(inst: &str) -> Option<(u8, Vec<State>)> {
    use State::{Append, Comma, DestReg, Imm8, Imm16, RegPair, RstImm, SrcReg};
    match inst.to_lowercase().as_str() {
        "mov" => Some((0x40, vec![DestReg, Comma, SrcReg, Append(1)])),
        "mvi" => Some((0x06, vec![DestReg, Comma, Imm8, Append(2)])),
        "lxi" => Some((0x01, vec![RegPair, Comma, Imm16, Append(3)])),
        "stax" => Some((0x02, vec![RegPair, Append(1)])),
        "ldax" => Some((0x0A, vec![RegPair, Append(1)])),
        "sta" => Some((0x32, vec![Imm16, Append(3)])),
        "lda" => Some((0x3A, vec![Imm16, Append(3)])),
        "shld" => Some((0x22, vec![Imm16, Append(3)])),
        "lhld" => Some((0x2A, vec![Imm16, Append(3)])),
        "xchg" => Some((0xEB, vec![Append(1)])),
        "push" => Some((0xC5, vec![RegPair, Append(1)])),
        "pop" => Some((0xC1, vec![RegPair, Append(1)])),
        "xthl" => Some((0xE3, vec![Append(1)])),
        "sphl" => Some((0xF9, vec![Append(1)])),
        "inx" => Some((0x03, vec![RegPair, Append(1)])),
        "dcx" => Some((0x0B, vec![RegPair, Append(1)])),
        "jmp" => Some((0xC3, vec![Imm16, Append(3)])),
        "jc" => Some((0xDA, vec![Imm16, Append(3)])),
        "jnc" => Some((0xD2, vec![Imm16, Append(3)])),
        "jz" => Some((0xCA, vec![Imm16, Append(3)])),
        "jnz" => Some((0xC2, vec![Imm16, Append(3)])),
        "jp" => Some((0xF2, vec![Imm16, Append(3)])),
        "jm" => Some((0xFA, vec![Imm16, Append(3)])),
        "jpe" => Some((0xEA, vec![Imm16, Append(3)])),
        "jpo" => Some((0xE2, vec![Imm16, Append(3)])),
        "pchl" => Some((0xE9, vec![Append(1)])),
        "call" => Some((0xCD, vec![Imm16, Append(3)])),
        "cc" => Some((0xDC, vec![Imm16, Append(3)])),
        "cnc" => Some((0xD4, vec![Imm16, Append(3)])),
        "cz" => Some((0xCC, vec![Imm16, Append(3)])),
        "cnz" => Some((0xC4, vec![Imm16, Append(3)])),
        "cp" => Some((0xF4, vec![Imm16, Append(3)])),
        "cm" => Some((0xFC, vec![Imm16, Append(3)])),
        "cpe" => Some((0xEC, vec![Imm16, Append(3)])),
        "cpo" => Some((0xE4, vec![Imm16, Append(3)])),
        "ret" => Some((0xC9, vec![Append(1)])),
        "rc" => Some((0xD8, vec![Append(1)])),
        "rnc" => Some((0xD0, vec![Append(1)])),
        "rz" => Some((0xC8, vec![Append(1)])),
        "rnz" => Some((0xC0, vec![Append(1)])),
        "rp" => Some((0xF0, vec![Append(1)])),
        "rm" => Some((0xF8, vec![Append(1)])),
        "rpe" => Some((0xE8, vec![Append(1)])),
        "rpo" => Some((0xE0, vec![Append(1)])),
        "rst" => Some((0xC7, vec![RstImm, Append(1)])),
        "in" => Some((0xDB, vec![Imm8, Append(2)])),
        "out" => Some((0xD3, vec![Imm8, Append(2)])),
        "inr" => Some((0x04, vec![DestReg, Append(1)])),
        "dcr" => Some((0x05, vec![DestReg, Append(1)])),
        "add" => Some((0x80, vec![SrcReg, Append(1)])),
        "adc" => Some((0x88, vec![SrcReg, Append(1)])),
        "adi" => Some((0xC6, vec![Imm8, Append(2)])),
        "aci" => Some((0xCE, vec![Imm8, Append(2)])),
        "dad" => Some((0x09, vec![RegPair, Append(1)])),
        "sub" => Some((0x90, vec![SrcReg, Append(1)])),
        "sbb" => Some((0x98, vec![SrcReg, Append(1)])),
        "sui" => Some((0xD6, vec![Imm8, Append(2)])),
        "sbi" => Some((0xDE, vec![Imm8, Append(2)])),
        "ana" => Some((0xA0, vec![SrcReg, Append(1)])),
        "xra" => Some((0xA8, vec![SrcReg, Append(1)])),
        "ora" => Some((0xB0, vec![SrcReg, Append(1)])),
        "cmp" => Some((0xB8, vec![SrcReg, Append(1)])),
        "ani" => Some((0xE6, vec![Imm8, Append(2)])),
        "xri" => Some((0xEE, vec![Imm8, Append(2)])),
        "ori" => Some((0xF6, vec![Imm8, Append(2)])),
        "cpi" => Some((0xFE, vec![Imm8, Append(2)])),
        "rlc" => Some((0x07, vec![Append(1)])),
        "rrc" => Some((0x0F, vec![Append(1)])),
        "ral" => Some((0x17, vec![Append(1)])),
        "rar" => Some((0x1F, vec![Append(1)])),
        "cma" => Some((0x2F, vec![Append(1)])),
        "stc" => Some((0x37, vec![Append(1)])),
        "cmc" => Some((0x3F, vec![Append(1)])),
        "daa" => Some((0x27, vec![Append(1)])),
        "ei" => Some((0xFB, vec![Append(1)])),
        "di" => Some((0xF3, vec![Append(1)])),
        "nop" => Some((0x00, vec![Append(1)])),
        "hlt" => Some((0x76, vec![Append(1)])),
        "rim" => Some((0x20, vec![Append(1)])),
        "sim" => Some((0x30, vec![Append(1)])),
        _ => None,
    }
}
