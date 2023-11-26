use saturnc_ast::{Declaration, Pos};

pub fn parse<'source>(
    _source: &'source str,
) -> Result<Vec<Declaration<'source>>, ParseError<'source>> {
    todo!()
}

pub struct ParseError<'source> {
    pub kind: ParseErrorKind<'source>,
    pub pos: Pos,
}

pub enum ParseErrorKind<'source> {
    TODO(&'source ()),
}
