use std::str::Chars;

mod token;
pub use token::*;

pub use LiteralKind::*;
pub use TokenKind::*;

/// Lightweight iterator for character sequence
#[derive(Debug, Clone)]
pub struct Cursor<'a> {
    chars: Chars<'a>,
    remaining_len: usize,
}

pub const EOF: char = '\0';

impl<'a> Cursor<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars(),
            remaining_len: source.len(),
        }
    }
}

impl<'a> Cursor<'a> {
    pub fn advance_token(&mut self) -> Token {
        let Some(c) = self.next() else {
            return Token::eof();
        };

        let token_kind = match c {
            '/' => match self.advance() {
                '/' => self.line_comment(),
                '*' => self.block_comment(),
                _ => Slash,
            },
            c if c.is_whitespace() => self.whitespace(c),
            c if c.is_ident_start() => self.ident(),
            c if !c.is_ascii() => self.invalid_ident(),
            '\'' => Literal(self.char()),
            '"' => Literal(self.str()),
            '0'..='9' => Literal(self.parse_number(c)),
            ';' => Semi,
            ':' => Colon,
            ',' => Comma,
            '.' => Dot,
            '@' => At,
            '#' => Pound,
            '~' => Tilde,
            '?' => Question,
            '$' => Dollar,
            '=' => Eq,
            '!' => Bang,
            '<' => Lt,
            '>' => Gt,
            '-' => Minus,
            '&' => And,
            '|' => Or,
            '+' => Plus,
            '*' => Star,
            '^' => Caret,
            '%' => Percent,
            '(' => OpenParen,
            ')' => CloseParen,
            '{' => OpenBrace,
            '}' => CloseBrace,
            '[' => OpenBracket,
            ']' => CloseBracket,
            _ => self.unknown(),
        };
        let token = Token::new(token_kind, self.token_len());
        self.reset_token_len();
        token
    }
}

impl<'a> Cursor<'a> {
    fn line_comment(&mut self) -> TokenKind {
        // Check if comment is docs: `/// docs`
        // `//// comment` - not docs
        let kind = if self.first() == '/' && self.second() != '/' {
            Docs
        } else {
            LineComment
        };
        // Consume comment
        self.skip_while(|c| c != '\n');
        kind
    }

    fn block_comment(&mut self) -> TokenKind {
        let mut terminated = false;
        while !self.is_eof() {
            if self.first() == '*' && self.second() == '/' {
                // Skip `*`
                self.skip();
                // Skip `/`
                self.skip();
                terminated = true;
                break;
            }
            self.skip();
        }

        BlockComment { terminated }
    }

    fn whitespace(&mut self, first: char) -> TokenKind {
        let mut newline = first == '\n';
        self.skip_while(|c| {
            if c == '\n' {
                newline = true;
                return true;
            }
            c.is_whitespace()
        });
        Whitespace { newline }
    }
}

impl<'a> Cursor<'a> {
    fn ident(&mut self) -> TokenKind {
        self.skip_while(is_ident_continue);
        if !self.first().is_ascii() {
            return self.invalid_ident();
        }
        Ident
    }

    fn invalid_ident(&mut self) -> TokenKind {
        self.skip_while(char::is_alphanumeric);
        InvalidIdent
    }

    fn unknown(&mut self) -> TokenKind {
        self.skip_while(|c| !c.is_ascii() && c.is_alphanumeric());
        Unknown
    }
}

impl<'a> Cursor<'a> {
    fn parse_number(&mut self, first: char) -> LiteralKind {
        let base = if first == '0' {
            // Check special character
            let base = match self.first() {
                'b' => Base::Binary,
                'o' => Base::Octal,
                'x' => Base::Hexadecimal,
                // number continue
                '0'..='9' => Base::Decimal,
                // Just zero
                _ => {
                    return Int {
                        base: Base::Decimal,
                        empty: false,
                    }
                }
            };
            if base != Base::Decimal {
                // Skip special character
                self.skip();
            }
            base
        } else {
            Base::Decimal
        };

        let has_digits = if base <= Base::Decimal {
            self.skip_decimal()
        } else {
            self.skip_hex()
        };

        if base != Base::Decimal && matches!(has_digits, HasDigits::No) {
            return Int { base, empty: true };
        }

        let [first, second] = self.two();
        if first == '.' && second != '.' && !second.is_ascii_alphabetic() {
            self.skip();
            self.skip_decimal();
            Float { base }
        } else {
            Int { base, empty: false }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum HasDigits {
    Yes,
    No,
}

impl<'a> Cursor<'a> {
    fn skip_decimal(&mut self) -> HasDigits {
        let mut has_digits = HasDigits::No;
        loop {
            match self.first() {
                '_' => self.skip(),
                '0'..='9' => {
                    has_digits = HasDigits::Yes;
                    self.skip();
                }
                _ => break,
            }
        }
        has_digits
    }

    fn skip_hex(&mut self) -> HasDigits {
        let mut has_digits = HasDigits::No;
        loop {
            match self.first() {
                '_' => self.skip(),
                '0'..='9' | 'a'..='f' | 'A'..='F' => {
                    has_digits = HasDigits::Yes;
                    self.skip();
                }
                _ => break,
            }
        }
        has_digits
    }
}

impl<'a> Cursor<'a> {
    fn char(&mut self) -> LiteralKind {
        if self.second() == '\'' && self.first() != '\\' {
            // skip char
            self.skip();
            // skip '\''
            self.skip();
            return Char { terminated: true };
        }

        loop {
            if self.is_eof() {
                break;
            }

            match self.first() {
                // end '\''
                '\'' => {
                    self.skip();
                    return Char { terminated: true };
                }
                // newline is not supported
                '\n' => break,
                // escaped character
                '\\' => {
                    self.skip();
                    self.skip();
                }
                // just character
                _ => self.skip(),
            }
        }

        Char { terminated: false }
    }

    fn str(&mut self) -> LiteralKind {
        loop {
            if self.is_eof() {
                break;
            }

            match self.first() {
                // end `"`
                '"' => {
                    self.skip();
                    return Char { terminated: true };
                }
                // escaped character
                '\\' => {
                    self.skip();
                    self.skip();
                }
                // just character
                _ => self.skip(),
            }
        }

        Str { terminated: false }
    }
}

impl<'a> Cursor<'a> {
    pub fn token_len(&self) -> u32 {
        (self.remaining_len - self.chars.as_str().len()) as u32
    }

    pub fn reset_token_len(&mut self) {
        self.remaining_len = self.chars.as_str().len();
    }
}

impl<'a> Cursor<'a> {
    pub fn skip_while<F>(&mut self, mut predicate: F)
    where
        F: FnMut(char) -> bool,
    {
        while !self.is_eof() && predicate(self.first()) {
            self.skip();
        }
    }

    pub fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }
}

impl<'a> Cursor<'a> {
    /// Returns the first character and moving to the next character.
    #[inline]
    pub fn next(&mut self) -> Option<char> {
        self.chars.next()
    }

    /// Returns the first character without moving to the next character.
    #[inline]
    pub fn peek(&mut self) -> Option<char> {
        self.chars.clone().next()
    }
}

// For convenience
impl<'a> Cursor<'a> {
    /// Skip the current character.
    #[inline(always)]
    pub fn skip(&mut self) {
        self.next();
    }

    /// Skip the some characters.
    #[inline]
    pub fn skip_by(&mut self, count: usize) {
        for _ in 0..count {
            self.skip();
        }
    }

    /// Like [Cursor::next] but if `None` return `'\0'`.
    pub fn advance(&mut self) -> char {
        self.next().unwrap_or(EOF)
    }

    /// Like [Cursor::peek] but if `None` return `'\0'`.
    pub fn first(&mut self) -> char {
        self.peek().unwrap_or(EOF)
    }

    pub fn second(&self) -> char {
        let mut chars = self.chars.clone();
        chars.next();
        chars.next().unwrap_or(EOF)
    }

    pub fn two(&mut self) -> [char; 2] {
        let mut chars = self.chars.clone();
        let first = chars.next().unwrap_or(EOF);
        let second = chars.next().unwrap_or(EOF);
        [first, second]
    }

    pub fn third(&self) -> char {
        let mut chars = self.chars.clone();
        chars.next();
        chars.next().unwrap_or(EOF)
    }
}

trait IsIdentStartExt {
    fn is_ident_start(&self) -> bool;
}

impl IsIdentStartExt for char {
    #[inline]
    fn is_ident_start(&self) -> bool {
        is_ident_start(*self)
    }
}

#[inline]
fn is_ident_start(c: char) -> bool {
    // we support only ascii for identifiers
    c.is_ascii_alphabetic() || c == '_'
}

#[inline]
fn is_ident_continue(c: char) -> bool {
    // we support only ascii for identifiers
    c.is_ascii_alphanumeric() || c == '_'
}

#[test]
fn test() {
    let source = r#"
/// Docs 1
/// Docs 2
/* Block comment */
fn pretty_fun() {
    println!("Pretty fun!");
}
"#;
    let source = r#"1;"#;
    let mut cursor = Cursor::new(source);
    let mut index = 0;
    loop {
        let token = cursor.advance_token();
        let end = index + token.len as usize;
        let span = index..end;
        println!("{:?} {:?}", token.kind, &source[span]);
        if let Eof = token.kind {
            break;
        }
        index = end;
    }
}
