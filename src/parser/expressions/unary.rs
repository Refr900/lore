use crate::lexer::BinaryToken;

use super::ExprKind;

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub operators: Vec<UnaryOp>,
    pub expr: Box<ExprKind>,
}

impl UnaryExpr {
    pub const fn new(operators: Vec<UnaryOp>, expr: Box<ExprKind>) -> Self {
        Self { operators, expr }
    }
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    // Valid
    /// `!`
    Not,
    /// `-`
    Minus,

    // Unvalid
    /// `+`
    Plus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `%`
    Percent,
    /// `^`
    Caret,
    /// `&`
    And,
    /// `|`
    Or,
    /// `<<`
    Shl,
    /// `>>`
    Shr,
}

impl UnaryOp {
    pub fn from_binary(token: BinaryToken) -> Self {
        match token {
            BinaryToken::Plus => Self::Plus,
            BinaryToken::Minus => Self::Minus,
            BinaryToken::Star => Self::Star,
            BinaryToken::Slash => Self::Slash,
            BinaryToken::Percent => Self::Percent,
            BinaryToken::Caret => Self::Caret,
            BinaryToken::And => Self::And,
            BinaryToken::Or => Self::Or,
            BinaryToken::Shl => Self::Shl,
            BinaryToken::Shr => Self::Shr,
        }
    }
}
