use crate::{
    lexer::{AssignKind, Kind, Operator},
    parser::{ExprKind, ExpressionExt, Parse, ParseError, Parser, StatementExt},
};

use super::StmtKind;

#[derive(Debug, Clone)]
pub struct AssignStmt {
    pub call: ExprKind,
    pub op: AssignKind,
    pub stmt: Box<StmtKind>,
}

impl Parse for AssignStmt {
    type Parsed = Self;
    type Error = ();

    fn parse(parser: &mut Parser<'_>) -> Result<Self::Parsed, ()> {
        let call = parser.parse_expression()?;
        let token = parser.peek()?;
        let op = match token.kind {
            Kind::Operator(Operator::Assign(kind)) => kind,
            _ => {
                parser.errors.push(ParseError::Unexpected(token));
                return Err(());
            }
        };
        let token = parser.stream.next();
        let Ok(stmt) = parser.parse_statement() else {
            parser.errors.push(ParseError::ExpectedAfter {
                expected: Kind::Dollar,
                before: token,
                found: parser.stream.second(),
            });
            return Err(());
        };
        Ok(Self {
            call,
            op,
            stmt: Box::new(stmt),
        })
    }
}
