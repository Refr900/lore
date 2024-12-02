use crate::lexer::TokenId;

#[derive(Debug, Clone)]
pub struct PathExpr {
    pub(crate) start: TokenId,
    // Like `path::path::path` = 2
    //           ^^    ^^
    pub(crate) len: u16,
}

impl PathExpr {
    pub const fn new(start: TokenId, len: u16) -> Self {
        Self { start, len }
    }
}
