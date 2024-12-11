use crate::{
    lexer::{Kind, TokenId},
    parser::{ExpectedItem, Item, ItemKind, ItemSequence, Parse, Parser},
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
        parser.stream.maybe(Kind![::]);
        let token = parser.stream.first();
        let start = token.id;
        if !matches!(token.kind, Kind::Ident) {
            parser.push_error(ExpectedItem::here(
                ItemSequence::Single(ItemKind::Ident),
                Item::from_token(token),
            ));
            return Err(());
        }
        // skip ident
        parser.stream.skip();

        let mut len = 0;
        loop {
            if parser.stream.expect(Kind![::]).is_err() {
                break;
            }

            if let Err(_) = parser.stream.expect(Kind::Ident) {
                parser.push_error(ExpectedItem::here(
                    ItemSequence::Single(ItemKind::Ident),
                    Item::from_token(token),
                ));
                return Err(());
            }

            len += 1;
        }

        Ok(TypePathExpr::new(start, len))
    }
}
