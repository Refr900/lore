use crate::{
    lexer::Kind,
    parser::{Item, ItemKind, Parse, ParseError, Parser, StatementExt, Token},
};

use super::{ParseStmtError, StmtKind};

impl Parse for Vec<StmtKind> {
    type Parsed = Self;
    type Error = ();

    fn parse(parser: &mut Parser<'_>) -> Result<Self::Parsed, ()> {
        let mut stmts = Vec::new();
        loop {
            match parser.parse_statement() {
                Ok(stmt) => {
                    match stmt {
                        StmtKind::Assign(_) | StmtKind::Var(_) => {
                            let _ = parser.expect(Kind![;]);
                            let token = parser.stream.first();
                            if let Kind!['}'] = token.kind {
                                stmts.push(stmt);
                                break;
                            }
                        }
                        StmtKind::Expr(_) => {
                            let token = parser.stream.first();
                            if !matches!(token.kind, Kind!['}']) {
                                let _ = parser.expect(Kind![;]);
                            }
                        }
                        StmtKind::Block(_) | StmtKind::If(_) => (),
                    }
                    stmts.push(stmt);
                }
                Err(err) => break,
            }
        }
        Ok(stmts)
    }
}
