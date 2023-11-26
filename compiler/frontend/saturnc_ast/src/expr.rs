use crate::{BinOp, Pos, Type, UnOp};

#[derive(PartialEq, Debug, Clone)]
pub enum Expr<'source> {
    Binary {
        lhs: Box<Self>,
        op: BinOp,
        rhs: Box<Self>,
    },
    Unary {
        op: UnOp,
        rhs: Box<Self>,
    },
    Call {
        id: Ident<'source>,
        args: Vec<Self>,
    },
    Cast {
        lhs: Box<Self>,
        ty: Type<'source>,
    },
    Atom(Atom<'source>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Atom<'source> {
    Ident(Ident<'source>),
    Literal(Literal<'source>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Literal<'source> {
    String(&'source str),
    Bool(bool),
    Float(f64),
    Int(u64),
}

#[derive(Debug, Clone, Copy)]
pub struct Ident<'source> {
    pub value: &'source str,
    pub pos: Pos,
}

impl PartialEq for Ident<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
