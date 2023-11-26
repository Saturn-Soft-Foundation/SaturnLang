#[derive(Clone, Copy, Debug)]
pub struct Token {
    pub tag: TokenTag,
    // TODO: Implement spans for better diagnostics
    // pub span: Span
}

