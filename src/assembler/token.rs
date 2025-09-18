use std::fmt;
use std::fmt::Debug;

#[derive(Clone)]
pub enum Token {
    Name(String),
    HexLiteral(u16),
    Comma,
    Colon,
    NewLine,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Name(content) => write!(f, "{content}"),
            Self::HexLiteral(content) => write!(f, "{content}"),
            Self::Comma => write!(f, ","),
            Self::Colon => write! (f, ":"),
            Self::NewLine => write!(f, "\\n"),
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Name(content) => write!(f, "Name({content})"),
            Self::HexLiteral(content) => write!(f, "VALUE({content})"),
            Self::Comma => write!(f, "COMMA"),
            Self::Colon => write!(f, "COLON"),
            Self::NewLine => write!(f, "NEW_LINE"),
        }
    }
}

impl Token {
    pub fn name_of(token: &Token) -> &str {
        match token {
            Self::Name(_content) => "name",
            Self::HexLiteral(_content) => "hex literal",
            Self::Comma => "comma",
            Self::Colon => "colon",
            Self::NewLine => "new line"
        }
    }
}
