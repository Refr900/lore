use crate::{
    lexer::{Kind, TokenId},
    parser::{Parse, Parser, TypePathExt},
};

#[derive(Debug, Clone)]
pub struct PathExpr {
    // `path::path.path`
    //  ^^^^
    pub(crate) start: TokenId,
    // Like `path::path.path` = 1
    //           ^^
    pub(crate) mod_len: u16,
    // Like `path::path.path` = 1
    //                 ^
    pub(crate) var_len: u16,
}

impl PathExpr {
    pub const fn new(start: TokenId, mod_len: u16, var_len: u16) -> Self {
        Self {
            start,
            mod_len,
            var_len,
        }
    }
}

impl Parse for PathExpr {
    type Parsed = Self;
    type Error = ();

    fn parse(parser: &mut Parser<'_>) -> Result<Self, ()> {
        let type_path = parser.parse_type_path()?;
        let Parser { stream, errors } = parser;
        
        let mut len = 0;
        loop {
            if stream.expect(Kind![.]).is_err() {
                break;
            }

            if let Err(err) = stream.expect(Kind::Ident) {
                errors.push(err);
                return Err(());
            }

            len += 1;
        }

        Ok(PathExpr::new(type_path.start, type_path.len, len))
    }
}
