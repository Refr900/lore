use crate::cursor::{self, Cursor};

mod macros;
pub(super) use macros::*;

mod token;
pub use token::*;

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    pub(crate) source: &'a str,
    pub(crate) errors: Vec<LexerError>,
    pub(crate) cursor: Cursor<'a>,
    pub(crate) span: Span,
}

#[derive(Debug, Clone, Copy)]
pub struct LexerError {
    pub span: Span,
    pub kind: LexerErrorKind,
}

impl LexerError {
    pub fn new(span: Span, kind: LexerErrorKind) -> Self {
        Self { span, kind }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LexerErrorKind {
    InvalidIdent,
    InvalidSuffix,
    EmptyInt,
    UnterminatedStr,
    UnterminatedChar,
}

/*
 * let source = "0b0100u4 * 0b0100u4"; // Debug -> panic!, Release -> UB (Wrapping)
 * let source = "0b0100u4 *% 0b0100u4"; // Wrapping
 * let source = "0b0100u4 *# 0b0100u4"; // Always panic!
 * let source = "0b0100u4 *? 0b0100u4"; // Return Result
 */

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            errors: Vec::new(),
            cursor: Cursor::new(source),
            span: Span::zero(),
        }
    }

    pub fn tokenize(source: &'a str) -> (Vec<Token>, Vec<LexerError>) {
        let mut lexer = Self::new(source);
        let mut tokens = Vec::new();
        loop {
            let token = lexer.advance_token();
            if let TokenKind::Eof = token.kind {
                break;
            } else {
                tokens.push(token);
            }
        }
        (tokens, lexer.errors)
    }
}

impl<'a> Lexer<'a> {
    pub fn advance_token(&mut self) -> Token {
        macro_rules! match_next {
            {
                $($cursor:path => $kind:expr),+;
                _ => $default_kind:expr $(,)?
            } => {
                match self.first().kind {
                    $(
                        $cursor => {
                            self.skip();
                            $kind
                        }
                    ),*,
                    _ => $default_kind,
                }
            };
        }

        macro_rules! bin_op {
            ($op:path) => {{
                if let cursor::Eq = self.first().kind {
                    self.skip();
                    TokenKind::BinaryEq($op)
                } else {
                    TokenKind::Binary($op)
                }
            }};
        }

        let kind = loop {
            let token = self.next();
            if let cursor::Eof = token.kind {
                return Token::new(TokenKind::Eof, self.span);
            }
            break match token.kind {
                cursor::Whitespace { .. } | cursor::LineComment | cursor::BlockComment { .. } => {
                    self.span.consume();
                    continue;
                }
                cursor::Docs => TokenKind::Docs,
                cursor::Ident => match self.lexeme() {
                    "pub" => TokenKind::Keyword(Keyword::Pub),
                    "let" => TokenKind::Keyword(Keyword::Let),
                    "const" => TokenKind::Keyword(Keyword::Const),
                    "mut" => TokenKind::Keyword(Keyword::Mut),
                    "if" => TokenKind::Keyword(Keyword::If),
                    "else" => TokenKind::Keyword(Keyword::Else),
                    "while" => TokenKind::Keyword(Keyword::While),
                    "for" => TokenKind::Keyword(Keyword::For),
                    "in" => TokenKind::Keyword(Keyword::In),
                    "fn" => TokenKind::Keyword(Keyword::Fn),
                    "false" | "true" => TokenKind::Literal(Literal::bool()),
                    _ => TokenKind::Ident,
                },
                cursor::InvalidIdent => {
                    self.push_error(LexerErrorKind::InvalidIdent);
                    TokenKind::Ident
                }
                cursor::Literal(kind) => {
                    let kind = match kind {
                        cursor::Int { base, empty } => {
                            if empty {
                                self.push_error(LexerErrorKind::EmptyInt);
                            }
                            LiteralKind::Int { base }
                        }
                        cursor::Float { base } => LiteralKind::Float { base },
                        cursor::Char { terminated } => {
                            if !terminated {
                                self.push_error(LexerErrorKind::UnterminatedChar);
                            }
                            LiteralKind::Char
                        }
                        cursor::Str { terminated } => {
                            if !terminated {
                                self.push_error(LexerErrorKind::UnterminatedStr);
                            }
                            LiteralKind::Str
                        }
                    };
                    let first = self.first();
                    let suffix_len = match first.kind {
                        cursor::Ident => {
                            self.skip();
                            first.len
                        }
                        cursor::InvalidIdent => {
                            self.skip();
                            self.push_error(LexerErrorKind::InvalidSuffix);
                            first.len
                        }
                        _ => 0,
                    };
                    TokenKind::Literal(Literal { kind, suffix_len })
                }
                cursor::Semi => TokenKind::Semi,
                cursor::Colon => match_next! {
                    cursor::Colon => TokenKind::PathSep;
                    _ => TokenKind::Colon,
                },
                cursor::Comma => TokenKind::Comma,
                cursor::Dot => match_next! {
                    cursor::Dot => match_next! {
                        cursor::Dot => TokenKind::DotDotDot,
                        cursor::Eq => TokenKind::DotDotEq;
                        _ => TokenKind::DotDot,
                    };
                    _ => TokenKind::Dot,
                },
                cursor::At => TokenKind::At,
                cursor::Pound => TokenKind::Pound,
                cursor::Tilde => TokenKind::Tilde,
                cursor::Question => TokenKind::Question,
                cursor::Dollar => TokenKind::Dollar,
                cursor::Eq => match_next! {
                    cursor::Eq => TokenKind::EqEq,
                    cursor::Gt => TokenKind::FatArrow;
                    _ => TokenKind::Eq,
                },
                cursor::Bang => match_next! {
                    cursor::Eq => TokenKind::Ne;
                    _ => TokenKind::Not,
                },
                cursor::Lt => match_next! {
                    cursor::Eq => TokenKind::Le;
                    _ => TokenKind::Lt,
                },
                cursor::Gt => match_next! {
                    cursor::Eq => TokenKind::Ge;
                    _ => TokenKind::Gt,
                },
                cursor::And => match_next! {
                    cursor::And => TokenKind::AndAnd;
                    _ => bin_op!(BinaryToken::And),
                },
                cursor::Or => match_next! {
                    cursor::Or => TokenKind::OrOr;
                    _ => bin_op!(BinaryToken::Or),
                },
                cursor::Plus => bin_op!(BinaryToken::Plus),
                cursor::Minus => match_next! {
                    cursor::Gt => TokenKind::RArrow;
                    _ => bin_op!(BinaryToken::Minus),
                },
                cursor::Star => bin_op!(BinaryToken::Star),
                cursor::Slash => bin_op!(BinaryToken::Slash),
                cursor::Caret => bin_op!(BinaryToken::Caret),
                cursor::Percent => bin_op!(BinaryToken::Percent),
                cursor::OpenParen => TokenKind::OpenDelim(Delimiter::Paren),
                cursor::CloseParen => TokenKind::CloseDelim(Delimiter::Paren),
                cursor::OpenBrace => TokenKind::OpenDelim(Delimiter::Brace),
                cursor::CloseBrace => TokenKind::CloseDelim(Delimiter::Brace),
                cursor::OpenBracket => TokenKind::OpenDelim(Delimiter::Bracket),
                cursor::CloseBracket => TokenKind::CloseDelim(Delimiter::Bracket),
                cursor::Unknown => TokenKind::Unknown,
                cursor::Eof => unreachable!(),
            };
        };
        let token = Token::new(kind, self.span);
        self.span.consume();
        token
    }
}

impl<'a> Lexer<'a> {
    fn push_error(&mut self, kind: LexerErrorKind) {
        self.errors.push(LexerError::new(self.span, kind));
    }
}

impl<'a> Lexer<'a> {
    pub fn skip_while<F>(&mut self, mut predicate: F)
    where
        F: FnMut(cursor::Token) -> bool,
    {
        while !self.is_eof() && predicate(self.first()) {
            self.next();
        }
    }

    pub fn is_eof(&self) -> bool {
        self.cursor.is_eof()
    }
}

impl<'a> Lexer<'a> {
    fn lexeme(&self) -> &str {
        &self.source[self.span.as_range()]
    }
}

impl<'a> Lexer<'a> {
    fn next(&mut self) -> cursor::Token {
        let token = self.cursor.advance_token();
        self.span.end += token.len as usize;
        token
    }

    fn skip(&mut self) {
        self.next();
    }

    fn first(&self) -> cursor::Token {
        self.cursor.clone().advance_token()
    }
}

#[test]
fn test() {
    let source = r#"while true { ... }"#;
    let mut lexer = Lexer::new(source);
    let mut count = 1u32;
    loop {
        let token = lexer.advance_token();
        let lexeme = &source[token.span.as_range()];
        if let TokenKind::Literal(lit) = token.kind {
            println!("{:>4}: {:?}", count, token.kind);
            println!("    |  span: {:?}", token.span.as_range());
            println!("    |  lexeme: {:?}", lexeme);
        } else {
            println!("{:>4}: {:?}", count, token.kind);
            println!("    |  span: {:?}", token.span.as_range());
            println!("    |  lexeme: {:?}", lexeme);
        }

        if let TokenKind::Eof = token.kind {
            break;
        }
        count += 1;
    }
    println!();
    for error in lexer.errors.iter() {
        let span = error.span.as_range();
        println!(" err: {:?}", error.kind);
        println!("    | span: {:?}", span);
        println!("    | lexeme: {}", &source[span]);
    }
}
