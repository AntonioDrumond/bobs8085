use std::fmt;
use std::fmt::Debug;

#[derive(Clone, Copy)]
pub enum TokenType {
    Name,
    HexLiteral,
    Comma,
    Colon,
    NewLine,
}

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
    column: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.lexeme)
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.token_type {
            TokenType::Name => write!(f, "NAME(lexeme: {}, ", self.lexeme)?,
            TokenType::HexLiteral => write!(f, "HEX(lexeme: {}, ", self.lexeme)?,
            TokenType::Comma => write!(f, "COMMA(")?,
            TokenType::Colon => write!(f, "COLON(")?,
            TokenType::NewLine => write!(f, "NEW_LINE(")?,
        }

        write!(f, "line: {}, column: {})", self.line, self.column)
    }
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, line: usize, column: usize) -> Self {
        Token {
            token_type,
            lexeme,
            line,
            column,
        }
    }

    pub fn new_name(lexeme: String, line: usize, column: usize) -> Self {
        Self::new(TokenType::Name, lexeme, line, column)
    }

    pub fn new_hex_literal(lexeme: String, line: usize, column: usize) -> Self {
        Self::new(TokenType::HexLiteral, lexeme, line, column)
    }

    pub fn new_comma(line: usize, column: usize) -> Self {
        Self::new(TokenType::Comma, String::from(","), line, column)
    }

    pub fn new_colon(line: usize, column: usize) -> Self {
        Self::new(TokenType::Colon, String::from(":"), line, column)
    }

    pub fn new_new_line(line: usize, column: usize) -> Self {
        Self::new(TokenType::NewLine, String::from("\n"), line, column)
    }

    pub fn type_of(token: &Token) -> &str {
        match token.token_type {
            TokenType::Name => "name",
            TokenType::HexLiteral => "hex literal",
            TokenType::Comma => "comma",
            TokenType::Colon => "colon",
            TokenType::NewLine => "new line",
        }
    }

    pub fn token_type(&self) -> TokenType {
        self.token_type
    }

    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }
}
