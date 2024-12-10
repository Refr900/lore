use crate::{
    lexer::{Kind, TokenId},
    parser::{Parse, Parser},
};

#[derive(Debug, Clone)]
pub struct TypePathExpr {
    // `path::path.path`
    //  ^^^^
    pub(crate) start: TokenId,
    // Like `path::path.path` = 1
    //           ^^
    pub(crate) len: u16,
}

impl TypePathExpr {
    pub const fn new(start: TokenId, len: u16) -> Self {
        Self { start, len }
    }
}

impl Parse for TypePathExpr {
    type Parsed = Self;
    type Error = ();

    fn parse(parser: &mut Parser<'_>) -> Result<Self, ()> {
        let Parser { stream, errors } = parser;
        stream.maybe(Kind![::]);
        let start = match stream.expect(Kind::Ident) {
            Ok(id) => id,
            Err(err) => {
                errors.push(err);
                return Err(());
            }
        };

        let mut len = 0;
        loop {
            if stream.expect(Kind![::]).is_err() {
                break;
            }

            if let Err(err) = stream.expect(Kind::Ident) {
                errors.push(err);
                return Err(());
            }

            len += 1;
        }

        Ok(TypePathExpr::new(start, len))
    }
}
