use super::token::*;

pub(super) fn parse(tokens: Vec<Box<Token>>) {
    for token in tokens {
        println!("{:?}", (*token));
    }
}
