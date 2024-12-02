use super::statements::StmtKind;

#[derive(Debug, Clone)]
pub enum DeclKind {
    Statement(StmtKind),
}
