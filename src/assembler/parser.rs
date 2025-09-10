use super::tokens::*;

pub(super) fn parse(tokens: Vec<Box<dyn Token>>) {
    for token in tokens {
        println!("{}", (*token).get_content());
    }
}
