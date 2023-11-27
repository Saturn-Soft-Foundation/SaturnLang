use super::pos::Pos;
use super::token_tag::TokenTag;

#[derive(Clone, Copy, Debug)]
pub struct Token {
    pub tag: TokenTag,
    pub pos: Pos
}

