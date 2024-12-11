use crate::{
    lexer::{Kind, TokenId},
    parser::{ExpectedItem, Item, ItemKind, ItemSequence, Parse, Parser, TypePathExt},
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
        let mut len = 0;
        loop {
            if parser.stream.expect(Kind![.]).is_err() {
                break;
            }

            let token = parser.stream.first();
            if !matches!(token.kind, Kind::Ident) {
                parser.push_error(ExpectedItem::here(
                    ItemSequence::Single(ItemKind::Ident),
                    Item::from_token(token),
                ));
                return Err(());
            }
            // skip ident
            parser.stream.skip();

            len += 1;
        }

        Ok(PathExpr::new(type_path.start, type_path.len, len))
    }
}
