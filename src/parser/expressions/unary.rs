use crate::{
    lexer::{BinaryKind, Kind, Operator},
    parser::{CallExt, Parse, ParseError, Parser},
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
        let Parser { stream, errors } = parser;
        let mut operators = Vec::new();
        loop {
            let token = match stream.peek() {
                Ok(token) => token,
                Err(err) => {
                    errors.push(err);
                    break;
                }
            };
            let op = match token.kind {
                Kind![!] => UnaryKind::Not,
                Kind![-] => UnaryKind::Minus,
                _ => match token.kind {
                    Kind::Ident | Kind::Literal(_) | Kind!['('] | Kind!['{'] | Kind!['}'] => break,
                    _ => {
                        errors.push(ParseError::expected_any(&[Kind![!], Kind![-]], token));
                        stream.skip();
                        continue;
                    }
                },
            };
            operators.push(op);
            stream.skip();
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
