use crate::lexer::{self, TokenId};

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub start: TokenId,
    pub end: TokenId,
}

impl Span {
    pub fn new(start: TokenId, end: TokenId) -> Self {
        debug_assert!(start.0 <= end.0);
        Self { start, end }
    }

    pub fn to_lexer_span(&self, spans: &[lexer::Span]) -> lexer::Span {
        let start = spans[self.start.0 as usize].start;
        let end = spans[self.end.0 as usize].start;
        lexer::Span::new(start, end)
    }
}
