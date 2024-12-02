use crate::lexer::{BinaryToken, TokenId};

use super::expressions::ExprKind;

#[derive(Clone)]
pub enum StmtKind {
    Expr(ExprKind),
    Variable(Variable),
    Assign(Assign),
    Print(ExprKind),
}

impl std::fmt::Debug for StmtKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Expr(arg0) => arg0.fmt(f),
            Self::Variable(arg0) => arg0.fmt(f),
            Self::Assign(arg0) => arg0.fmt(f),
            Self::Print(arg0) => arg0.fmt(f),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StatementId(pub usize);

#[derive(Debug, Clone)]
pub struct Variable {
    pub visibility: Visibility,
    pub mutability: Mutability,
    pub name: TokenId,
    pub expr: ExprKind,
}

#[derive(Debug, Clone)]
pub enum Visibility {
    Private,
    Public,
}

#[derive(Debug, Clone)]
pub enum Mutability {
    Const,
    Let,
    // let mut
    Mut,
}

#[derive(Debug, Clone)]
pub struct Assign {
    pub call: ExprKind,
    pub op: AssignOp,
    pub expr: ExprKind,
}

#[derive(Debug, Clone)]
pub struct AssignOp {
    pub kind: BinaryToken,
}

#[derive(Debug, Clone)]
pub struct Print {
    pub expr: ExprKind,
}
