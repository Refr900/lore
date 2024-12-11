use crate::{
    lexer::{Keyword, Kind},
    parser::{
        CallExt, ExpectedItem, Item, ItemKind, ItemSequence, Parse, ParseError, Parser, Span,
    },
};

use super::ExprKind;

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub operators: Vec<UnaryKind>,
    pub expr: Box<ExprKind>,
}

impl UnaryExpr {
    pub const fn new(operators: Vec<UnaryKind>, expr: Box<ExprKind>) -> Self {
        Self { operators, expr }
    }
}

impl Parse for UnaryExpr {
    type Parsed = ExprKind;
    type Error = ();

    fn parse(parser: &mut Parser<'_>) -> Result<ExprKind, ()> {
        let mut operators = Vec::new();
        loop {
            let token = parser.stream.first();
            if matches!(token.kind, Kind::Eof) {
                parser.push_error(ParseError::Unexpected(Item::from_token(token)));
                return Err(());
            }

            let op = match token.kind {
                Kind![!] => UnaryKind::Not,
                Kind![-] => UnaryKind::Minus,
                _ => match token.kind {
                    Kind!['('] | Kind!['{'] | Kind!['}'] | Kind::Ident | Kind::Literal(_) => break,
                    Kind::Keyword(keyword) => {
                        let kind = match keyword {
                            Keyword::Pub => ItemKind::Stmt,
                            Keyword::Let => ItemKind::VarStmt,
                            Keyword::Const => ItemKind::VarStmt,
                            // TODO: For now mutable references not supported
                            Keyword::Mut => ItemKind::VarStmt,
                            Keyword::Else => ItemKind::Stmt,
                            Keyword::In => ItemKind::Keyword(Keyword::In),
                            Keyword::Fn => ItemKind::Fn,
                            Keyword::If | Keyword::While | Keyword::For => break,
                        };
                        parser.push_error(ExpectedItem::here(
                            ItemSequence::Single(ItemKind::Expr),
                            Item::new(kind, Span::dot(token.id)),
                        ));
                        break;
                    }
                    _ => {
                        parser.push_error(ExpectedItem::here(
                            ItemSequence::Unary,
                            Item::from_token(token),
                        ));
                        parser.stream.skip();
                        continue;
                    }
                },
            };
            operators.push(op);
            parser.stream.skip();
        }

        let expr = parser.parse_call()?;
        Ok(if !operators.is_empty() {
            let unary = UnaryExpr::new(operators, Box::new(expr));
            ExprKind::Unary(unary)
        } else {
            expr
        })
    }
}

#[derive(Debug, Clone)]
pub enum UnaryKind {
    /// `!`
    Not,
    /// `-`
    Minus,
}
