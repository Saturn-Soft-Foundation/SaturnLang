use pos::Pos;

mod declaration;
mod expr;
mod operators;
mod pos;
mod statement;

pub use declaration::*;
pub use expr::*;
pub use operators::*;
pub use statement::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Block<'source> {
    pub statements: Vec<Statement<'source>>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Type<'source> {
    Ident(Ident<'source>),
    Ptr(Box<Self>),
}
