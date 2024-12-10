use crate::{
    lexer::Kind,
    parser::{Parse, ParseError, Parser, StatementExt, Token},
};

use super::StmtKind;

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
                Err(_) => break,
            }
        }

        if let [.., last] = parser.errors.as_slice() {
            if matches!(
                last,
                ParseError::Unexpected(Token {
                    id: _,
                    kind: Kind::Eof
                })
            ) {
                parser.errors.pop();
            }
        }

        Ok(stmts)
    }
}
