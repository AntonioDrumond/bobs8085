use std::fmt;

#[derive(Debug)]
pub enum TokenType {
    Instruction,
    Register,
    Value,
    LabelDeclaration,
    LabelValue,
    Comma,
    NewLine,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Instruction => write!(f, "INSTRUCTION"),
            TokenType::Register => write!(f, "REGISTER"),
            TokenType::Value => write!(f, "VALUE"),
            TokenType::LabelDeclaration => write!(f, "LABEL_DECLARATION"),
            TokenType::LabelValue => write!(f, "LABEL_VALUE"),
            TokenType::Comma => write!(f, "COMMA"),
            TokenType::NewLine => write!(f, "NEW_LINE"),
        }
    }
}

pub struct Token {
    token_type: TokenType,
    content: String,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token_type.to_string())?;
        if matches!(
            self.token_type,
            TokenType::Instruction | TokenType::Register | TokenType::Value | TokenType::Label
        ) {
            write!(f, "({})", self.content)?;
        }
        Ok(())
    }
}

#[allow(dead_code)]
impl Token {
    pub fn instruction(content: &str) -> Self {
        Token {
            token_type: TokenType::Instruction,
    }

    pub fn register(content: &str) -> Self {
        Token {
            token_type: TokenType::Register,
            content: content.to_lowercase(),
        }
    }

    pub fn value(content: &str) -> Self {
        Token {
            token_type: TokenType::Value,
            content: content.to_lowercase(),
        }
    }

    pub fn label_declr(content: &str) -> Self {
        Token {
            token_type: TokenType::LabelDeclaration,
            content: content.to_lowercase(),
        }
    }

    pub fn label_value(content: &str) -> Self {
        Token {
            token_type: TokenType::LabelValue,
            content: content.to_lowercase(),
        }
    }

    pub fn comma() -> Self {
        Token {
            token_type: TokenType::Comma,
            content: String::from(","),
        }
    }

    pub fn new_line() -> Self {
        Token {
            token_type: TokenType::NewLine,
            content: String::from("\n"),
        }
    }

    pub fn get_token_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }
}
