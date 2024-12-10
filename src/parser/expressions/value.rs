use crate::{
    lexer::Kind,
    parser::{ExpressionExt, LitExt, Parse, Parser},
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
                if let Err(err) = parser.stream.expect(Kind![')']) {
                    parser.errors.push(err);
                }
                expr
            }
            Kind::Literal(_) => ExprKind::Lit(parser.parse_lit()?),
            _ => return Err(()),
        };
        Ok(value)
    }
}
