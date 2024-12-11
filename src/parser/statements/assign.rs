use crate::{
    lexer::{AssignKind, Kind},
    parser::{
        ExpectedItem, ExprKind, ExpressionExt, Item, ItemSequence, Parse, Parser, StatementExt,
        Token,
    },
};

use super::StmtKind;

#[derive(Debug, Clone)]
pub struct AssignStmt {
    pub call: Box<ExprKind>,
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
            Kind::Assign(kind) => kind,
            _ => {
                parser.push_error(ExpectedItem::here(
                    ItemSequence::Assign, //
                    Item::from_token(token),
                ));
                return Err(());
            }
        };
        let assign = parser.stream.next();
        let Ok(stmt) = parser.parse_statement() else {
            parser.push_error(stmt_after_assign_expected(assign));
            return Err(());
        };
        
        Ok(Self {
            call: Box::new(call),
            op,
            stmt: Box::new(stmt),
        })
    }
}

fn stmt_after_assign_expected(assign: Token) -> ExpectedItem {
    ExpectedItem::after(ItemSequence::StmtWithReturnValue, Item::from_token(assign))
}
