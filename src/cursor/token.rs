#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub kind: TokenKind,
    pub len: u32,
}

impl Token {
    pub fn is_ident(&self) -> bool {
        self.kind.is_ident()
    }
}

impl Token {
    #[inline]
    pub const fn new(kind: TokenKind, len: u32) -> Self {
        Self { kind, len }
    }

    #[inline]
    pub const fn eof() -> Self {
        Self::new(TokenKind::Eof, 0)
    }
}

impl Token {
    pub const fn is_one_symbol(&self) -> bool {
        self.kind.is_one_symbol()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TokenKind {
    // `// comment`
    LineComment,
    // `/// Docs`
    Docs,
    // Like `/* block comment */`
    // Terminated when comment have end `*/`
    BlockComment {
        terminated: bool,
    },
    /// Any spacing
    Whitespace {
        newline: bool,
    },
    /// `ident`, `enum`, `_var123`
    Ident,
    InvalidIdent,
    /// Look [LiteralKind]
    Literal(LiteralKind),
    // Punctuation
    /// `;`
    Semi,
    /// `:`
    Colon,
    /// `,`
    Comma,
    /// `.`
    Dot,
    /// `@`
    At,
    /// `#`
    Pound,
    /// `~`
    Tilde,
    /// `?`
    Question,
    /// `$`
    Dollar,
    /// `=`
    Eq,
    /// `!`
    Bang,
    /// `<`
    Lt,
    /// `>`
    Gt,
    /// `&`
    And,
    /// `|`
    Or,
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `^`
    Caret,
    /// `%`
    Percent,
    /// `(`
    OpenParen,
    /// `)`
    CloseParen,
    /// `{`
    OpenBrace,
    /// `}`
    CloseBrace,
    /// `[`
    OpenBracket,
    /// `]`
    CloseBracket,
    Unknown,
    Eof,
}

impl TokenKind {
    pub fn is_ident(&self) -> bool {
        matches!(self, Self::Ident | Self::InvalidIdent)
    }
}

impl TokenKind {
    pub const fn as_name(&self) -> &'static str {
        match self {
            Self::LineComment => "LineComment",
            Self::Docs => "Docs",
            Self::BlockComment { .. } => "BlockComment",
            Self::Whitespace { .. } => "Whitespace",
            Self::Ident { .. } => "Ident",
            Self::InvalidIdent => "InvalidIdent",
            Self::Literal { .. } => "Literal",
            Self::Semi => "Semi",
            Self::Colon => "Colon",
            Self::Comma => "Comma",
            Self::Dot => "Dot",
            Self::At => "At",
            Self::Pound => "Pound",
            Self::Tilde => "Tilde",
            Self::Question => "Question",
            Self::Dollar => "Dollar",
            Self::Eq => "Eq",
            Self::Bang => "Bang",
            Self::Lt => "Lt",
            Self::Gt => "Gt",
            Self::Minus => "Minus",
            Self::And => "And",
            Self::Or => "Or",
            Self::Plus => "Plus",
            Self::Star => "Star",
            Self::Slash => "Slash",
            Self::Caret => "Caret",
            Self::Percent => "Percent",
            Self::OpenParen => "OpenParen",
            Self::CloseParen => "CloseParen",
            Self::OpenBrace => "OpenBrace",
            Self::CloseBrace => "CloseBrace",
            Self::OpenBracket => "OpenBracket",
            Self::CloseBracket => "CloseBracket",
            Self::Unknown => "Unknown",
            Self::Eof => "Eof",
        }
    }
    pub const fn is_one_symbol(&self) -> bool {
        use TokenKind::*;
        matches!(
            self,
            Semi | Colon
                | Comma
                | Dot
                | At
                | Pound
                | Tilde
                | Question
                | Dollar
                | Eq
                | Bang
                | Lt
                | Gt
                | Minus
                | And
                | Or
                | Plus
                | Star
                | Slash
                | Caret
                | Percent
                | OpenParen
                | CloseParen
                | OpenBrace
                | CloseBrace
                | OpenBracket
                | CloseBracket
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LiteralKind {
    Int { base: Base, empty: bool },
    Float { base: Base },
    Char { terminated: bool },
    Str { terminated: bool },
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Base {
    B1 = 1,
    Binary,
    B3,
    B4,
    B5,
    B6,
    B7,
    Octal,
    B9,
    Decimal,
    B11,
    B12,
    B13,
    B14,
    B15,
    Hexadecimal,
    B17,
    B18,
    B19,
    B20,
    B21,
    B22,
    B23,
    B24,
    B25,
    B26,
    B27,
    B28,
    B29,
    B30,
    B31,
    B32,
    B33,
    B34,
    B35,
    B36,
}

impl std::fmt::Debug for Base {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Binary => write!(f, "Binary"),
            Self::Octal => write!(f, "Octal"),
            Self::Decimal => write!(f, "Decimal"),
            Self::Hexadecimal => write!(f, "Hexadecimal"),
            _ => write!(f, "Base-{}", *self as u8),
        }
    }
}
