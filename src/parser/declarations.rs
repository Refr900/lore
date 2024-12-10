use crate::lexer::TokenId;

use super::{statements::StmtKind, Vis};

#[derive(Debug, Clone)]
pub enum DeclKind {
    Statement(StmtKind),
    Fn(FnDecl),
}

#[derive(Debug, Clone)]
pub struct FnDecl {
    pub vis: Vis,
    pub name: TokenId,
    pub inner: Vec<StmtKind>,
}
