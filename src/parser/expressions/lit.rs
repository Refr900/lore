use crate::{
    lexer::{Kind, TokenId},
    parser::{Item, Parse, ParseError, Parser},
};

#[derive(Debug, Clone, Copy)]
pub struct LitExpr(pub TokenId);

impl Parse for LitExpr {
    type Parsed = Self;
    type Error = ();

    fn parse(parser: &mut Parser<'_>) -> Result<Self, ()> {
        let token = match parser.stream.peek() {
            Ok(token) => token,
            Err(_) => return Err(()),
        };

        if let Kind::Literal(_) = token.kind {
            Ok(Self(parser.stream.next_id()))
        } else {
            parser.push_error(ParseError::Unexpected(Item::from_token(token)));
            Err(())
        }
    }
}
