use crate::{Block, Expr, Ident, Type};

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration<'source> {
    Function(FunctionDeclaration<'source>),
    Constant(ConstantDeclaration<'source>),
    Struct(StructDeclaration<'source>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration<'source> {
    pub id: Ident<'source>,
    pub args: Vec<FunctionArg<'source>>,
    pub return_ty: Type<'source>,
    pub body: Block<'source>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionArg<'source> {
    pub id: Ident<'source>,
    pub ty: Type<'source>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstantDeclaration<'source> {
    pub id: Ident<'source>,
    pub ty: Type<'source>,
    pub expr: Expr<'source>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructDeclaration<'source> {
    pub id: Ident<'source>,
    pub fields: StructField<'source>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructField<'source> {
    pub id: Ident<'source>,
    pub ty: Type<'source>,
}
