#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TokenTag {
    // Keywords
    Fn,
    Const,
    Struct,
    If,
    Else,
    While,
    Let,
    Mut,
    Return,
    Break,
    Continue,
    As,

    // Two-char
    // ||
    Or,
    // &&
    AndAnd,
    // !=
    BangEqual,
    // ==
    EqualEqual,
    // >=
    GreaterEqual,
    // <=
    LessEqual,

    // One-char
    // (
    LParen,
    // )
    RParen,
    // {
    LCurly,
    // }
    RCurly,
    // :
    Colon,
    // <
    Less,
    // >
    Greater,
    // +
    Plus,
    // *
    Asterisk,
    // /
    Slash,
    // !
    Bang,
    // -
    Minus,
    // &
    And,
}

