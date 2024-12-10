use crate::lexer::{self, Kind, TokenId};
use crate::parser::Token;

use super::Span;

#[derive(Debug, Clone)]
pub struct SyntaxError {
    span: Span,
    kind: SyntaxErrorKind,
}

impl SyntaxError {
    pub fn unvalid_assignment(span: Span) -> Self {
        Self {
            span,
            kind: SyntaxErrorKind::UnvalidAssignment,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SyntaxErrorKind {
    UnvalidAssignment,
}

impl core::error::Error for SyntaxErrorKind {}

impl core::fmt::Display for SyntaxErrorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnvalidAssignment => write!(f, "unvalid assignment"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ParseError<'a> {
    Syntax(SyntaxError),
    Expected {
        expected: Kind,
        found: Token,
    },
    ExpectedAfter {
        expected: Kind,
        before: Token,
        found: Token,
    },
    ExpectedAny {
        expected: &'a [Kind],
        found: Token,
    },
    Unexpected(Token),
}

impl<'a> ParseError<'a> {
    pub fn expected(expected: Kind, found: Token) -> Self {
        Self::Expected { expected, found }
    }

    pub fn expected_any(expected: &'a [Kind], found: Token) -> Self {
        Self::ExpectedAny { expected, found }
    }
}

impl<'a> ParseError<'a> {
    pub fn span_in(&self, spans: &[lexer::Span]) -> lexer::Span {
        match self {
            Self::Syntax(err) => err.span.to_lexer_span(spans),
            _ => spans[self.token_id().0 as usize],
        }
    }

    pub fn token_id(&self) -> TokenId {
        match self {
            Self::Syntax(err) => err.span.start,
            Self::Expected { expected: _, found } => found.id,
            Self::ExpectedAfter { found, .. } => found.id,
            Self::ExpectedAny { expected: _, found } => found.id,
            Self::Unexpected(token) => token.id,
        }
    }
}

impl<'a> core::error::Error for ParseError<'a> {}

impl<'a> core::fmt::Display for ParseError<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Syntax(SyntaxError { span: _, kind }) => write!(f, "{kind}"),
            Self::Expected { expected, found } => {
                let c = if expected.is_simple() { "`" } else { "" };
                write!(f, "Expected {c}{}{c}", expected)?;
                if !matches!(found.kind, Kind::Eof) {
                    let c = if found.kind.is_simple() { "`" } else { "" };
                    write!(f, " but found {c}{}{c}", found.kind)?;
                }
                Ok(())
            }
            Self::ExpectedAfter {
                expected,
                before,
                found,
            } => {
                let c1 = if expected.is_simple() { "`" } else { "" };
                let c2 = if before.kind.is_simple() { "`" } else { "" };
                write!(
                    f,
                    "Expected {c1}{}{c1} after {c2}{}{c2}",
                    expected, before.kind
                )?;
                if !matches!(found.kind, Kind::Eof) {
                    let c = if found.kind.is_simple() { "`" } else { "" };
                    write!(f, " but found {c}{}{c}", found.kind)?;
                }
                Ok(())
            }
            Self::ExpectedAny { expected, found } => {
                let c = if found.kind.is_simple() { "`" } else { "" };
                write!(f, "Unexpected {c}{}{c}, expected", found.kind)?;
                let [init @ .., last] = expected else {
                    return Ok(());
                };

                for expected in init.iter() {
                    let c = if expected.is_simple() { "`" } else { "" };
                    write!(f, " {c}{}{c},", expected)?;
                }

                let c = if last.is_simple() { "`" } else { "" };
                write!(f, " {c}{}{c}", last)?;
                Ok(())
            }
            Self::Unexpected(kind) => {
                let c = if kind.kind.is_simple() { "`" } else { "" };
                write!(f, "Unexpected  {c}{}{c}", kind.kind,)
            }
        }
    }
}
