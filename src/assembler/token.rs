use std::fmt;

pub(super) enum TokenType {
    Instruction,
    Register,
    Address,
    Comma,
    NewLine,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Instruction => write!(f, "INSTRUCTION"),
            TokenType::Register => write!(f, "REGISTER"),
            TokenType::Address => write!(f, "ADDRESS"),
            TokenType::Comma => write!(f, "COMMA"),
            TokenType::NewLine => write!(f, "NEW_LINE"),
        }
    }
}

pub(super) struct Token {
    token_type: TokenType,
    content: String,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token_type.to_string())?;
        if matches!(
            self.token_type,
            TokenType::Instruction | TokenType::Register | TokenType::Address
        ) {
            write!(f, "({})", self.content)?;
        }
        Ok(())
    }
}

#[allow(dead_code)]
impl Token {
    pub(super) fn instruction(content: &str) -> Self {
        Token {
            token_type: TokenType::Instruction,
            content: content.to_owned(),
        }
    }

    pub(super) fn register(content: &str) -> Self {
        Token {
            token_type: TokenType::Register,
            content: content.to_owned(),
        }
    }

    pub(super) fn address(content: &str) -> Self {
        Token {
            token_type: TokenType::Address,
            content: content.to_owned(),
        }
    }

    pub(super) fn comma() -> Self {
        Token {
            token_type: TokenType::Comma,
            content: String::from(","),
        }
    }

    pub(super) fn new_line() -> Self {
        Token {
            token_type: TokenType::NewLine,
            content: String::from("\n"),
        }
    }

    pub(super) fn get_token_type(&self) -> &TokenType {
        &self.token_type
    }

    pub(super) fn get_content(&self) -> &str {
        &self.content
    }
}
