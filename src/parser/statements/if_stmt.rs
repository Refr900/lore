use crate::{
    lexer::Kind,
    parser::{BlockExt, Parse, ParseError, Parser, StatementExt},
};

use super::{BlockStmt, StmtKind};

#[derive(Debug, Clone)]
pub struct IfStmt {
    pub condition: Box<StmtKind>,
    pub block: BlockStmt,
    pub else_stmt: Option<Box<StmtKind>>,
}

impl Parse for IfStmt {
    type Parsed = IfStmt;
    type Error = ();

    fn parse(parser: &mut Parser<'_>) -> Result<Self::Parsed, ()> {
        parser.expect(Kind![if])?;
        let Ok(condition) = parser.parse_statement() else {
            return Err(());
        };
        let block = parser.parse_block()?;
        let else_stmt = match parser.stream.maybe(Kind![else]) {
            Some(_) => match parser.parse_statement() {
                Ok(stmt) => Some(Box::new(stmt)),
                Err(_) => return Err(()),
            },
            None => None,
        };

        Ok(Self {
            condition: Box::new(condition),
            block,
            else_stmt,
        })
    }
}
