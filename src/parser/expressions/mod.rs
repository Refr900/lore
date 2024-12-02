use crate::lexer::TokenId;

mod binary;
mod path;
mod unary;

pub use binary::*;
pub use path::*;
pub use unary::*;

#[derive(Debug, Clone)]
pub enum ExprKind {
    Literal(TokenId),
    Path(PathExpr),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
}

#[derive(Debug, Clone, Copy)]
pub struct ExprId(pub usize);