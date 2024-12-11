use crate::lexer::{self, AssignKind, BinaryKind, Delimiter, Keyword};

use super::{Span, Token};

// region: ----- ParseError -----

#[derive(Debug, Clone)]
pub enum ParseError {
    Syntax(SyntaxError),
    Expected(ExpectedItem),
    Unexpected(Item),
}

impl ParseError {
    pub fn span(&self) -> Span {
        match self {
            Self::Syntax(kind) => match kind {
                SyntaxError::UnvalidAssignment { span } => *span,
            },
            Self::Expected(item) => item.span,
            Self::Unexpected(item) => item.span,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum ItemSequence {
    Single(ItemKind),
    Unary,
    Assign,
    StmtWithReturnValue,
}

impl ItemSequence {
    pub const fn items(&self) -> &[ItemKind] {
        match self {
            Self::Single(kind) => core::array::from_ref(kind),
            Self::Unary => &[ItemKind::Not, ItemKind::Binary(Some(BinaryKind::Minus))],
            Self::Assign => &[ItemKind::Assign(None)],
            Self::StmtWithReturnValue => &[ItemKind::Expr, ItemKind::BlockStmt, ItemKind::IfStmt],
        }
    }
}

#[derive(Debug, Clone)]
pub enum Place {
    Before,
    Here,
    After,
}

#[derive(Debug, Clone)]
pub struct ExpectedItem {
    pub place: Place,
    pub expected: ItemSequence,
    pub found: ItemKind,
    pub span: Span,
}

impl ExpectedItem {
    pub fn new(place: Place, expected: ItemSequence, found: Item) -> Self {
        Self {
            place,
            expected,
            found: found.kind,
            span: found.span,
        }
    }

    #[inline]
    pub fn here(expected: ItemSequence, found: Item) -> Self {
        Self::new(Place::Here, expected, found)
    }

    #[inline]
    pub fn after(expected: ItemSequence, found: Item) -> Self {
        Self::new(Place::After, expected, found)
    }
}

impl From<ExpectedItem> for ParseError {
    fn from(items: ExpectedItem) -> Self {
        Self::Expected(items)
    }
}

impl core::error::Error for ParseError {}

impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Syntax(kind) => write!(f, "{kind}"),
            Self::Expected(ExpectedItem {
                place: _,
                expected,
                found,
                span: _,
            }) => {
                if matches!(found, ItemKind::Eof) {
                    write!(f, "Expected ")?;
                } else {
                    write!(f, "Found {found}, expected ")?;
                }

                let items = expected.items();
                if items.len() > 1 {
                    let penult = items.len() - 2;
                    for item in &items[..penult] {
                        write!(f, "{item}, ")?;
                    }
                    write!(f, "{} or {}", items[penult], items[penult + 1])
                } else {
                    write!(f, "{}", items[0])
                }
            }
            Self::Unexpected(item) => write!(f, "Unexpected {}", item.kind),
        }
    }
}

// endregion: ----- ParseError -----

#[derive(Debug, Clone)]
pub enum SyntaxError {
    UnvalidAssignment { span: Span },
}

impl core::error::Error for SyntaxError {}

impl core::fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnvalidAssignment { .. } => write!(f, "unvalid assignment"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Item {
    pub(crate) kind: ItemKind,
    pub(crate) span: Span,
}

impl Item {
    pub fn new(kind: ItemKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn from_token(token: Token) -> Self {
        Self {
            kind: ItemKind::from_lexer_kind(token.kind),
            span: Span::dot(token.id),
        }
    }

    pub fn from_lexer_kind(kind: lexer::Kind, span: Span) -> Self {
        Self {
            kind: ItemKind::from_lexer_kind(kind),
            span,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ItemKind {
    // region: ----- Lexer -----
    /// `/// docs`
    Docs,
    /// `!`
    Not,
    /// `~`
    Tilde,
    /// `@`
    At,
    /// `.`
    Dot,
    /// `..`
    DotDot,
    /// `...`
    DotDotDot,
    /// `..=`
    DotDotEq,
    /// `,`
    Comma,
    /// `;`
    Semi,
    /// `:`
    Colon,
    /// `::`
    PathSep,
    /// `->`
    RArrow,
    /// `=>`
    FatArrow,
    /// `#`
    Pound,
    /// `$`
    Dollar,
    /// `?`
    Question,
    Ident,
    Keyword(Keyword),
    Literal,
    Binary(Option<BinaryKind>),
    Assign(Option<AssignKind>),
    OpenDelim(Delimiter),
    CloseDelim(Delimiter),
    Unknown,
    Eof,
    // endregion: ----- Lexer -----

    // region: ----- Parser -----
    Expr,
    Stmt,
    VarStmt,
    AssignStmt,
    BlockStmt,
    IfStmt,
    Fn,
    // endregion: ----- Parser -----
}

impl ItemKind {
    pub const fn from_lexer_kind(kind: lexer::Kind) -> Self {
        match kind {
            lexer::Kind::Docs => Self::Docs,
            lexer::Kind::Not => Self::Not,
            lexer::Kind::Tilde => Self::Tilde,
            lexer::Kind::At => Self::At,
            lexer::Kind::Dot => Self::Dot,
            lexer::Kind::DotDot => Self::DotDot,
            lexer::Kind::DotDotDot => Self::DotDotDot,
            lexer::Kind::DotDotEq => Self::DotDotEq,
            lexer::Kind::Comma => Self::Comma,
            lexer::Kind::Semi => Self::Semi,
            lexer::Kind::Colon => Self::Colon,
            lexer::Kind::PathSep => Self::PathSep,
            lexer::Kind::RArrow => Self::RArrow,
            lexer::Kind::FatArrow => Self::FatArrow,
            lexer::Kind::Pound => Self::Pound,
            lexer::Kind::Dollar => Self::Dollar,
            lexer::Kind::Question => Self::Question,
            lexer::Kind::Ident => Self::Ident,
            lexer::Kind::Keyword(keyword) => Self::Keyword(keyword),
            lexer::Kind::Literal(_) => Self::Literal,
            lexer::Kind::Binary(kind) => Self::Binary(Some(kind)),
            lexer::Kind::Assign(kind) => Self::Assign(Some(kind)),
            lexer::Kind::OpenDelim(delimiter) => Self::OpenDelim(delimiter),
            lexer::Kind::CloseDelim(delimiter) => Self::CloseDelim(delimiter),
            lexer::Kind::Unknown => Self::Unknown,
            lexer::Kind::Eof => Self::Eof,
        }
    }
}

impl ItemKind {
    pub fn is_lexer_kind(&self) -> bool {
        match self {
            Self::Docs
            | Self::Ident
            | Self::Literal
            | Self::Binary(None)
            | Self::Assign(None)
            | Self::Unknown
            | Self::Eof => false,
            _ => true,
        }
    }
}

impl core::fmt::Display for ItemKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let c = if self.is_lexer_kind() { "`" } else { "" };
        write!(f, "{c}");
        match self {
            Self::Docs => write!(f, "docs"),
            Self::Not => write!(f, "!"),
            Self::Tilde => write!(f, "~"),
            Self::At => write!(f, "@"),
            Self::Dot => write!(f, "."),
            Self::DotDot => write!(f, ".."),
            Self::DotDotDot => write!(f, "..."),
            Self::DotDotEq => write!(f, "..="),
            Self::Comma => write!(f, ","),
            Self::Semi => write!(f, ";"),
            Self::Colon => write!(f, ":"),
            Self::PathSep => write!(f, "::"),
            Self::RArrow => write!(f, "->"),
            Self::FatArrow => write!(f, "=>"),
            Self::Pound => write!(f, "#"),
            Self::Dollar => write!(f, "$"),
            Self::Question => write!(f, "?"),
            Self::Ident => write!(f, "ident"),
            Self::Keyword(keyword) => write!(f, "{keyword}"),
            Self::Literal => write!(f, "literal"),
            Self::Binary(kind) => match kind {
                Some(kind) => write!(f, "{kind}"),
                None => write!(f, "binary operator"),
            },
            Self::Assign(kind) => match kind {
                Some(kind) => write!(f, "{kind}"),
                None => write!(f, "assign operator"),
            },
            Self::OpenDelim(delim) => {
                let c = match delim {
                    Delimiter::Paren => '(',
                    Delimiter::Brace => '{',
                    Delimiter::Bracket => '[',
                };
                write!(f, "{c}")
            }
            Self::CloseDelim(delim) => {
                let c = match delim {
                    Delimiter::Paren => ')',
                    Delimiter::Brace => '}',
                    Delimiter::Bracket => ']',
                };
                write!(f, "{c}")
            }
            Self::Unknown => write!(f, "unknown"),
            Self::Eof => write!(f, "end of file"),
            Self::Expr => write!(f, "expression"),
            Self::Stmt => write!(f, "statement"),
            Self::VarStmt => write!(f, "variable"),
            Self::AssignStmt => write!(f, "assign"),
            Self::BlockStmt => write!(f, "block"),
            Self::IfStmt => write!(f, "if statement"),
            Self::Fn => write!(f, "function"),
        }?;
        write!(f, "{c}")
    }
}

impl From<lexer::Kind> for ItemKind {
    fn from(kind: lexer::Kind) -> Self {
        Self::from_lexer_kind(kind)
    }
}
