use std::fmt;
use std::fmt::Debug;

#[derive(Clone)]
pub enum Token {
    Name(String),
    HexLiteral(String),
    Comma,
    Colon,
    NewLine,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Name(content) => write!(f, "{content}"),
            Self::Value(content) => write!(f, "{content}"),
            Self::Comma => write!(f, ","),
            Self::Colon => write! (f, ":"),
            Self::NewLine => write!(f, "\\n"),
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Instruction(content) => write!(f, "INSTRUCTION({})", content),
            Self::Register(content) => write!(f, "REGISTER({})", content),
            Self::Value(content) => write!(f, "VALUE({})", content),
            Self::LabelDeclaration(content) => write!(f, "LABEL_DECLARATION({})", content),
            Self::LabelValue(content) => write!(f, "LABEL_VALUE({})", content),
            Self::Comma => write!(f, "COMMA"),
            Self::Colon => write!(f, "COLON"),
            Self::NewLine => write!(f, "NEW_LINE"),
        }
    }
}
