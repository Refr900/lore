use std::str::FromStr;

use crate::cursor::Base;

use super::Span;

// region: ----- Token -----

#[derive(Debug, Clone)]
pub struct Token {
    pub(crate) kind: Kind,
    pub(crate) span: Span,
}

impl Token {
    pub const fn new(kind: Kind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenId(pub u32);

impl TokenId {
    pub fn as_index(&self) -> usize {
        self.0 as usize
    }
}

// endregion: ----- Token -----

// region: ----- Kind -----

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// `/// Docs`
    Docs,
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
    Literal(Literal),
    Operator(Operator),
    OpenDelim(Delimiter),
    CloseDelim(Delimiter),
    Unknown,
    Eof,
}

impl core::fmt::Display for Kind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Docs => write!(f, "Docs"),
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
            Self::Ident => write!(f, "Ident"),
            Self::Keyword(keyword) => write!(f, "{keyword}"),
            Self::Literal(literal) => write!(f, "{literal}"),
            Self::Operator(operator) => write!(f, "{operator}"),
            Self::OpenDelim(delim) => {
                let c = match delim {
                    Delimiter::Paren => '(',
                    Delimiter::Brace => '{',
                    Delimiter::Bracket => '[',
                };
                write!(f, "{}", c)
            }
            Self::CloseDelim(delim) => {
                let c = match delim {
                    Delimiter::Paren => ')',
                    Delimiter::Brace => '}',
                    Delimiter::Bracket => ']',
                };
                write!(f, "{}", c)
            }
            Self::Unknown => write!(f, "Unknown"),
            Self::Eof => write!(f, "Eof"),
        }
    }
}

impl Kind {
    pub fn is_simple(&self) -> bool {
        !matches!(
            self,
            Self::Docs | Self::Ident | Self::Literal { .. } | Self::Unknown | Self::Eof
        )
    }
}

// endregion: ----- Kind -----

// region: ----- Keyword -----

macro_rules! keywords {
    (
        $($str:literal => $keyword:ident),+ $(,)?
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Keyword {
            $($keyword),+
        }

        impl core::fmt::Display for Keyword {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                match self {
                    $(Self::$keyword => write!(f, $str)),+
                }
            }
        }

        impl FromStr for Keyword {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(match s {
                    $($str => Self::$keyword),+,
                    _ => return Err(()),
                })
            }
        }

    };
}

keywords! {
    "pub"   => Pub,
    "let"   => Let,
    "const" => Const,
    "mut"   => Mut,
    "if"    => If,
    "else"  => Else,
    "while" => While,
    "for"   => For,
    "in"    => In,
    "fn"    => Fn,
}

// endregion: ----- Keyword -----

// region: ----- Literal -----

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Literal {
    pub kind: LiteralKind,
    pub suffix_len: u16,
}

impl core::fmt::Display for Literal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl Literal {
    pub const fn new(kind: LiteralKind, suffix_len: u16) -> Self {
        Self { kind, suffix_len }
    }

    pub const fn bool() -> Self {
        Self::new(LiteralKind::Bool, 0)
    }
}

// region: ----- LiteralKind -----

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiteralKind {
    Int { base: Base },
    Float { base: Base },
    Bool,
    Char,
    Str,
}

impl core::fmt::Display for LiteralKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Int { .. } => write!(f, "integer"),
            Self::Float { .. } => write!(f, "float"),
            Self::Bool => write!(f, "bool"),
            Self::Char => write!(f, "char"),
            Self::Str => write!(f, "string"),
        }
    }
}

// endregion: ----- LiteralKind -----

// endregion: ----- Literal -----

// region: ----- Operator -----

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    /// `!`
    Not,
    // `-`, `/`
    Binary(BinaryKind),
    /// `-=`, `/=`
    Assign(AssignKind),
}

impl core::fmt::Display for Operator {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Not => write!(f, "!"),
            Self::Binary(kind) => write!(f, "{kind}"),
            Self::Assign(kind) => write!(f, "{kind}"),
        }
    }
}

// region: ----- BinaryKind -----

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryKind {
    /// '+'
    Plus,
    /// '-'
    Minus,
    /// '*'
    Star,
    /// '/'
    Slash,
    /// '%'
    Percent,
    /// '^'
    Caret,
    /// `|`
    BinOr,
    /// `&`
    BinAnd,
    /// '<<'
    Shl,
    /// '>>'
    Shr,
    /// `==`
    Eq,
    /// `!=`
    Ne,
    /// `<`
    Lt,
    /// `>`
    Gt,
    /// `<=`
    Le,
    /// `>=`
    Ge,
    /// `||`
    Or,
    /// `&&`
    And,
}

impl core::fmt::Display for BinaryKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Star => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::Percent => write!(f, "%"),
            Self::Caret => write!(f, "^"),
            Self::BinOr => write!(f, "|"),
            Self::BinAnd => write!(f, "&"),
            Self::Shl => write!(f, "<<"),
            Self::Shr => write!(f, ">>"),
            Self::Eq => write!(f, "=="),
            Self::Ne => write!(f, "!="),
            Self::Lt => write!(f, "<"),
            Self::Gt => write!(f, ">"),
            Self::Le => write!(f, "<="),
            Self::Ge => write!(f, ">="),
            Self::Or => write!(f, "||"),
            Self::And => write!(f, "&&"),
        }
    }
}

// endregion: ----- BinaryKind -----

// region: ----- AssignKind -----

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignKind {
    /// `=`
    Eq,
    /// '+'
    Plus,
    /// '-'
    Minus,
    /// '*'
    Star,
    /// '/'
    Slash,
    /// '%'
    Percent,
    /// '^'
    Caret,
    /// `|`
    Or,
    /// `&`
    And,
    /// '<<'
    Shl,
    /// '>>'
    Shr,
}

impl core::fmt::Display for AssignKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Eq => write!(f, "="),
            Self::Plus => write!(f, "+="),
            Self::Minus => write!(f, "-="),
            Self::Star => write!(f, "*="),
            Self::Slash => write!(f, "/="),
            Self::Percent => write!(f, "%="),
            Self::Caret => write!(f, "^="),
            Self::Or => write!(f, "|="),
            Self::And => write!(f, "&="),
            Self::Shl => write!(f, "<<="),
            Self::Shr => write!(f, ">>="),
        }
    }
}

// endregion: ----- AssignKind -----

// endregion: ----- Operator -----

// region: ----- Delimiter -----

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Delimiter {
    /// `( ... )`
    Paren,
    /// `{ ... }`
    Brace,
    /// `[ ... ]`
    Bracket,
}

impl Delimiter {
    #[inline(always)]
    pub const fn as_open(&self) -> Kind {
        Kind::OpenDelim(*self)
    }

    #[inline(always)]
    pub const fn as_close(&self) -> Kind {
        Kind::CloseDelim(*self)
    }
}

// endregion: ----- Delimiter -----
