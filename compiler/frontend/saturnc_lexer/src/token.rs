use crate::pos::Pos;
use crate::token_tag::TokenTag;

#[derive(Clone, Copy, Debug)]
pub struct Token {
    pub tag: TokenTag,
    pub pos: Pos
}

