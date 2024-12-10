use crate::{lexer::TokenId, parser::LogicOrExt};

mod binary;
mod call;
mod lit;
mod path;
mod type_path;
mod unary;
mod value;

pub use binary::*;
pub use call::*;
pub use lit::*;
pub use path::*;
pub use type_path::*;
pub use unary::*;
pub use value::*;

use super::{BlockStmt, Parse, Parser};

#[derive(Debug, Clone)]
pub enum ExprKind {
    Lit(LitExpr),
    Path(PathExpr),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
}

impl Parse for ExprKind {
    type Parsed = Self;
    type Error = ();

    fn parse(parser: &mut Parser<'_>) -> Result<Self::Parsed, ()> {
        parser.parse_logic_or()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ExprId(pub usize);
