use crate::{
    lexer::{Kind, TokenId},
    parser::{Parse, ParseError, Parser},
};

#[derive(Debug, Clone, Copy)]
pub struct LitExpr(pub TokenId);

impl Parse for LitExpr {
    type Parsed = Self;
    type Error = ();

    fn parse(parser: &mut Parser<'_>) -> Result<Self, ()> {
        let Parser { stream, errors } = parser;
        let token = match stream.peek() {
            Ok(token) => token,
            Err(err) => {
                errors.push(err);
                return Err(());
            },
        };
        
        if let Kind::Literal(_) = token.kind {
            Ok(Self(stream.next_id()))
        } else {
            errors.push(ParseError::Unexpected(token));
            Err(())
        }
    }
}

