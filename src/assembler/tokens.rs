pub(super) trait Token {
    fn get_content(&self) -> &str;
}

pub(super) struct Instruction {
    pub content: String,
}

impl Token for Instruction {
    fn get_content(&self) -> &str {
        &self.content
    }
}

pub(super) struct Register {
    pub content: String,
}

impl Token for Register {
    fn get_content(&self) -> &str {
        &self.content
    }
}

pub(super) struct Address {
    pub content: String,
}

impl Token for Address {
    fn get_content(&self) -> &str {
        &self.content
    }
}

pub(super) struct Comma;

impl Token for Comma {
    fn get_content(&self) -> &str {
        ","
    }
}

pub(super) struct NewLine;

impl Token for NewLine {
    fn get_content(&self) -> &str {
        "\n"
    }
}
