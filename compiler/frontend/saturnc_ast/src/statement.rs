use smallvec::SmallVec;

use crate::{Block, Expr, Ident, Type};

#[derive(Debug, Clone, PartialEq)]
pub enum Statement<'source> {
    If(IfStatement<'source>),
    While(WhileStatement<'source>),
    Let(LetStatement<'source>),
    Expr(ExprStatement<'source>),
    Return(ReturnStatement<'source>),
    Break,
    Continue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement<'source> {
    pub branches: SmallVec<[IfBranch<'source>; 1]>,
    pub else_body: Option<Block<'source>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfBranch<'source> {
    pub cond: Expr<'source>,
    pub body: Block<'source>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileStatement<'source> {
    pub cond: Expr<'source>,
    pub body: Block<'source>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetStatement<'source> {
    pub id: Ident<'source>,
    pub is_mut: bool,
    pub ty: Option<Type<'source>>,
    pub expr: Option<Expr<'source>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprStatement<'source> {
    Expr(Expr<'source>),
    Assign {
        id: Ident<'source>,
        expr: Expr<'source>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement<'source> {
    pub expr: Option<Expr<'source>>,
}
