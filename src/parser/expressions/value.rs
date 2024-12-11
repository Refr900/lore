use crate::{
    lexer::{Delimiter, Kind},
    parser::{
        BlockExt, ExpectedItem, ExpressionExt, IfExt, Item, ItemKind, ItemSequence, LitExt, Parse,
        Parser, StmtKind,
    },
};

use super::ExprKind;

#[derive(Debug, Clone, Copy)]
pub struct ValueExpr;

impl Parse for ValueExpr {
    type Parsed = ExprKind;
    type Error = ();

    fn parse(parser: &mut Parser<'_>) -> Result<ExprKind, ()> {
        let token = parser.stream.first();
        let value = match token.kind {
            Kind!['('] => {
                parser.stream.skip();
                let expr = parser.parse_expression()?;
                let token = parser.stream.first();
                match token.kind {
                    Kind![')'] => parser.stream.skip(),
                    _ => {
                        // TODO: create macro like in lexer and cursor modules
                        let kind = ItemKind::CloseDelim(Delimiter::Paren);
                        let expected = ItemSequence::Single(kind);
                        let found = Item::from_token(token);
                        parser.push_error(ExpectedItem::here(expected, found));
                    }
                }
                expr
            }
            Kind::Literal(_) => ExprKind::Lit(parser.parse_lit()?),
            _ => return Err(()),
        };
        Ok(value)
    }
}
