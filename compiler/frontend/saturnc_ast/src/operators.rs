#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,

    Or,
    And,

    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum UnOp {
    Deref,
    Ref,
    Neg,
    Not,
}
