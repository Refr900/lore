#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub kind: Kind,
    pub len: u32,
}

impl Token {
    #[inline]
    pub const fn new(kind: Kind, len: u32) -> Self {
        Self { kind, len }
    }

    #[inline]
    pub const fn eof() -> Self {
        Self::new(Kind::Eof, 0)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Kind {
    // `/// Docs`
    Docs,
    // `// comment`
    LineComment,
    // Like `/* block comment */`
    // Terminated when comment have end `*/`
    BlockComment {
        terminated: bool,
    },
    /// Any spacing
    WhiteSpace {
        newline: bool,
    },
    /// `ident`, `enum`, `_var123`
    Ident,
    InvalidIdent,
    /// Look [LitKind]
    Lit(LitKind),
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

impl Kind {
    pub fn is_whitespace(&self) -> bool {
        matches!(
            self,
            Self::LineComment | Self::BlockComment { .. } | Self::WhiteSpace { .. }
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LitKind {
    Int { base: Base, empty: bool },
    Float { base: Base },
    Char { terminated: bool },
    Str { terminated: bool },
}

impl LitKind {
    pub fn empty_int() -> Self {
        Self::Int {
            base: Base::Decimal,
            empty: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Base {
    Binary = 2,
    Octal = 8,
    Decimal = 10,
    Hexadecimal = 16,
}
