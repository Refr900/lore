use crate::{
    lexer::Kind,
    parser::{Parse, Parser, StatementsExt},
};

use super::StmtKind;

#[derive(Debug, Clone)]
pub struct BlockStmt(pub Vec<StmtKind>);

impl Parse for BlockStmt {
    type Parsed = BlockStmt;
    type Error = ();

    fn parse(parser: &mut Parser<'_>) -> Result<Self::Parsed, ()> {
        parser.expect(Kind!['{'])?;
        let stmts = parser.parse_statements()?;
        parser.expect(Kind!['}'])?;
        Ok(Self(stmts))
    }
}
