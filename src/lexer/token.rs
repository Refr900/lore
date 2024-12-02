use std::ops::Range;

use crate::cursor::Base;

#[derive(Debug, Clone)]
pub struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) span: Span,
}

impl Token {
    pub const fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenId(pub usize);

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub(crate) start: usize,
    pub(crate) end: usize,
}

impl Span {
    pub const fn new(start: usize, end: usize) -> Self {
        debug_assert!(start <= end);
        Self { start, end }
    }

    pub const fn zero() -> Self {
        Self { start: 0, end: 0 }
    }
}

impl Span {
    pub fn consume(&mut self) {
        self.start = self.end;
    }

    pub const fn as_range(&self) -> Range<usize> {
        self.start..self.end
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Docs,
    /// `=`
    Eq,
    /// `==`
    EqEq,
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
    /// `&&`
    AndAnd,
    /// `||`
    OrOr,
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
    Literal(Literal),
    Binary(BinaryToken),
    BinaryEq(BinaryToken),
    OpenDelim(Delimiter),
    CloseDelim(Delimiter),
    Unknown,
    Eof,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryToken {
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
    /// `&`
    And,
    /// `|`
    Or,
    /// '<<'
    Shl,
    /// '>>'
    Shr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    /// `pub`
    Pub,
    /// `let`
    Let,
    /// `const`
    Const,
    /// `Mut`
    Mut,
    ///`if`
    If,
    /// `else`
    Else,
    /// `while`
    While,
    /// `for`
    For,
    /// `in`
    In,
    /// `fn`
    Fn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Literal {
    pub kind: LiteralKind,
    pub suffix_len: u32,
}

impl Literal {
    pub const fn new(kind: LiteralKind, suffix_len: u32) -> Self {
        Self { kind, suffix_len }
    }

    pub const fn bool() -> Self {
        Self::new(LiteralKind::Bool, 0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiteralKind {
    Bool,
    Int { base: Base },
    Float { base: Base },
    Char,
    Str,
}

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
    pub const fn as_open(&self) -> TokenKind {
        TokenKind::OpenDelim(*self)
    }

    #[inline(always)]
    pub const fn as_close(&self) -> TokenKind {
        TokenKind::CloseDelim(*self)
    }
}
