use crate::lexer::BinaryToken;

use super::ExprKind;

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub op: BinaryOp,
    pub rhs: Box<ExprKind>,
    pub lhs: Box<ExprKind>,
}

impl BinaryExpr {
    pub const fn new(op: BinaryOp, rhs: Box<ExprKind>, lhs: Box<ExprKind>) -> Self {
        Self { op, rhs, lhs }
    }
}

#[derive(Debug, Clone)]
pub struct BinaryOp {
    pub kind: BinaryKind,
    pub eq: bool,
}

impl BinaryOp {
    pub const fn new(kind: BinaryKind) -> Self {
        Self { kind, eq: false }
    }
}

#[derive(Debug, Clone)]
pub enum BinaryKind {
    /// `+`
    Plus,
    /// `-`
    Minus,
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

impl BinaryKind {
    pub fn from_token(token: BinaryToken) -> Self {
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
