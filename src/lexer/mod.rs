use crate::cursor::{self, Cursor};

mod macros;
pub(crate) use macros::*;

mod token;
pub use token::*;

mod span;
pub use span::*;

mod location;
pub use location::*;

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
            if let Kind::Eof = token.kind {
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
                $([$($cursor:tt)*] => $kind:expr),+,
                _ => $default_kind:expr $(,)?
            } => {
                match self.first().kind {
                    $(
                        cursor::Kind![$($cursor)*] => {
                            self.skip();
                            $kind
                        }
                    ),*,
                    _ => $default_kind,
                }
            };
        }
        
        macro_rules! assign_or_binary {
            (@inner 
                assign: $assign:expr, 
                binary: $binary:expr $(,)?
            ) => {{
                if let cursor::Kind![=] = self.first().kind {
                    self.skip();
                    Kind::Operator(Operator::Assign($assign))
                } else {
                    Kind::Operator(Operator::Binary($binary))
                }
            }};
            [$($tt:tt)*] => { 
                assign_or_binary!(@inner 
                    assign: AssignKind![$($tt)*], 
                    binary: BinaryKind![$($tt)*],
                )
            };
        }
        
        let kind = loop {
            let token = self.next();
            if let cursor::Kind::Eof = token.kind {
                return Token::new(Kind::Eof, self.span);
            }
            break match token.kind {
                cursor::Kind::LineComment
                | cursor::Kind::BlockComment { .. }
                | cursor::Kind::WhiteSpace { .. } => {
                    self.span.consume();
                    continue;
                }
                cursor::Kind::Docs => Kind::Docs,
                cursor::Kind::Ident => self.parse_ident(),
                cursor::Kind::InvalidIdent => self.parse_invalid_ident(),
                cursor::Kind::Lit(kind) => self.parse_lit(kind),
                cursor::Kind![;] => Kind![;],
                cursor::Kind![:] => match_next! {
                    [:] => Kind![::],
                    _   => Kind![:],
                },
                cursor::Kind![,] => Kind![,],
                cursor::Kind![.] => match_next! {
                    [.] => match_next! {
                        [.] => Kind![...],
                        [=] => Kind![..=],
                        _   => Kind![..],
                    },
                    _ => Kind![.],
                },
                cursor::Kind![@] => Kind![@],
                cursor::Kind![#] => Kind![#],
                cursor::Kind![~] => Kind![~],
                cursor::Kind![?] => Kind![?],
                cursor::Kind![$] => Kind![$],
                cursor::Kind![=] => match_next! {
                    [=] => Kind![==],
                    [>] => Kind![=>],
                    _   => Kind![=],
                },
                cursor::Kind![!] => match_next! {
                    [=] => Kind![!=],
                    _   => Kind![!],
                },
                cursor::Kind![<] => match_next! {
                    [=] => Kind![<=],
                    _   => Kind![<],
                },
                cursor::Kind![>] => match_next! {
                    [=] => Kind![>=],
                    _   => Kind![>],
                },
                cursor::Kind![&] => match_next! {
                    [&] => Kind![&&],
                    _ => assign_or_binary![&],
                },
                cursor::Kind![|] => match_next! {
                    [|] => Kind![||],
                    _ => assign_or_binary![|],
                },
                cursor::Kind![+] => assign_or_binary![+],
                cursor::Kind![-] => match_next! {
                    [>] => Kind![->],
                    _ => assign_or_binary![-],
                },
                cursor::Kind![*] => assign_or_binary![*],
                cursor::Kind![/] => assign_or_binary![/],
                cursor::Kind![^] => assign_or_binary![^],
                cursor::Kind![%] => assign_or_binary![%],
                cursor::Kind!['('] => Kind!['('],
                cursor::Kind![')'] => Kind![')'],
                cursor::Kind!['{'] => Kind!['{'],
                cursor::Kind!['}'] => Kind!['}'],
                cursor::Kind!['['] => Kind!['['],
                cursor::Kind![']'] => Kind![']'],
                cursor::Kind::Unknown => Kind::Unknown,
                cursor::Kind::Eof => unreachable!(),
            };
        };
        let token = Token::new(kind, self.span);
        self.span.consume();
        token
    }
}

impl<'a> Lexer<'a> {
    // fn parse_space(&mut self) -> Kind {
    //     self.skip_while(|token| {
    //         matches!(
    //             token.kind,
    //             cursor::Kind::LineComment
    //                 | cursor::Kind::BlockComment { .. }
    //                 | cursor::Kind::WhiteSpace { .. }
    //         )
    //     });
    //     Kind::Space
    // }

    fn parse_ident(&mut self) -> Kind {
        match self.lexeme() {
            "pub" => Kind::Keyword(Keyword::Pub),
            "let" => Kind::Keyword(Keyword::Let),
            "const" => Kind::Keyword(Keyword::Const),
            "mut" => Kind::Keyword(Keyword::Mut),
            "if" => Kind::Keyword(Keyword::If),
            "else" => Kind::Keyword(Keyword::Else),
            "while" => Kind::Keyword(Keyword::While),
            "for" => Kind::Keyword(Keyword::For),
            "in" => Kind::Keyword(Keyword::In),
            "fn" => Kind::Keyword(Keyword::Fn),
            "false" | "true" => Kind::Literal(Literal::bool()),
            _ => Kind::Ident,
        }
    }

    fn parse_invalid_ident(&mut self) -> Kind {
        self.push_error(LexerErrorKind::InvalidIdent);
        // return valid ident for parser
        Kind::Ident
    }

    fn parse_lit(&mut self, kind: cursor::LitKind) -> Kind {
        let kind = match kind {
            cursor::LitKind::Int { base, empty } => {
                if empty {
                    self.push_error(LexerErrorKind::EmptyInt);
                }
                LiteralKind::Int { base }
            }
            cursor::LitKind::Float { base } => LiteralKind::Float { base },
            cursor::LitKind::Char { terminated } => {
                if !terminated {
                    self.push_error(LexerErrorKind::UnterminatedChar);
                }
                LiteralKind::Char
            }
            cursor::LitKind::Str { terminated } => {
                if !terminated {
                    self.push_error(LexerErrorKind::UnterminatedStr);
                }
                LiteralKind::Str
            }
        };
        let first = self.first();
        let suffix_len = match first.kind {
            cursor::Kind::Ident => {
                self.skip();
                first.len
            }
            cursor::Kind::InvalidIdent => {
                self.skip();
                self.push_error(LexerErrorKind::InvalidSuffix);
                first.len
            }
            _ => 0,
        };
        let suffix_len = suffix_len.try_into().unwrap();
        Kind::Literal(Literal { kind, suffix_len })
    }
}

impl<'a> Lexer<'a> {
    fn push_error(&mut self, kind: LexerErrorKind) {
        self.errors.push(LexerError::new(self.span, kind));
    }
}

impl<'a> Lexer<'a> {
    fn skip_while<F>(&mut self, mut predicate: F)
    where
        F: FnMut(cursor::Token) -> bool,
    {
        while !self.is_eof() && predicate(self.first()) {
            self.skip();
        }
    }

    fn is_eof(&self) -> bool {
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

    pub fn two(&mut self) -> [cursor::Token; 2] {
        let mut cursor = self.cursor.clone();
        let first = cursor.advance_token();
        let second = cursor.advance_token();
        [first, second]
    }
}

#[test]
fn test() {
    let source = r#"
const MAX_HEALTH = 100.0 * 1.2;
// Comment 1
// Comment 2
// Comment 3
/* Block Comment */
fn main() {
    let mut health = MAX_HEALTH - 10 * 2;
    let damage = 25.0;
    let debaff = 1.25;
    health -= damage * debaff
    print(health);
}
"#
    .trim();
    let mut lexer = Lexer::new(source);
    let mut count = 1u32;
    loop {
        let token = lexer.advance_token();
        let lexeme = &source[token.span.as_range()];
        println!("{:>4}: {:?}", count, token.kind);
        println!("    |  span: {:?}", token.span.as_range());
        println!("    |  lexeme: {:?}", lexeme);

        if let Kind::Eof = token.kind {
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
