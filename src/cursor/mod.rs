use std::str::Chars;

mod token;
pub use token::*;

mod macros;
pub(crate) use macros::*;

/// Lightweight iterator for character sequence
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct Cursor<'a>(Chars<'a>);

pub const EOF: char = '\0';

impl<'a> Cursor<'a> {
    pub fn new(source: &'a str) -> Self {
        Self(source.chars())
    }
}

impl<'a> Cursor<'a> {
    pub fn advance_token(&mut self) -> Token {
        let start_len = self.len();
        let kind = self.match_kind();
        let len = self.token_len(start_len);
        Token::new(kind, len)
    }

    fn match_kind(&mut self) -> Kind {
        let Some(first) = self.next() else {
            return Kind::Eof;
        };
        match first {
            '/' => match self.advance() {
                '/' => self.line_comment(),
                '*' => self.block_comment(),
                _ => Kind::Slash,
            },
            c if c.is_whitespace() => self.whitespace(c),
            c if c.is_ident_start() => self.ident(),
            c if !c.is_ascii() => self.invalid_ident(),
            '\'' => Kind::Lit(self.char()),
            '"' => Kind::Lit(self.str()),
            '0' => {
                let base = match self.parse_base() {
                    Ok(base) => base,
                    Err(JustZero) => {
                        return Kind::Lit(LitKind::Int {
                            base: Base::Decimal,
                            empty: false,
                        })
                    }
                };
                Kind::Lit(self.parse_number(base))
            }
            '1'..='9' => Kind::Lit(self.parse_decimal_number()),
            ';' => Kind![;],
            ':' => Kind![:],
            ',' => Kind![,],
            '.' => Kind![.],
            '@' => Kind![@],
            '#' => Kind![#],
            '~' => Kind![~],
            '?' => Kind![?],
            '$' => Kind![$],
            '=' => Kind![=],
            '!' => Kind![!],
            '<' => Kind![<],
            '>' => Kind![>],
            '-' => Kind![-],
            '&' => Kind![&],
            '|' => Kind![|],
            '+' => Kind![+],
            '*' => Kind![*],
            '^' => Kind![^],
            '%' => Kind![%],
            '(' => Kind!['('],
            ')' => Kind![')'],
            '{' => Kind!['{'],
            '}' => Kind!['}'],
            '[' => Kind!['['],
            ']' => Kind![']'],
            _ => self.unknown(),
        }
    }
}

impl<'a> Cursor<'a> {
    fn len(&self) -> usize {
        self.0.as_str().len()
    }

    fn lexeme(&self, start_len: usize) -> &str {
        let end = self.token_len(start_len) as usize;
        &self.0.as_str()[..end]
    }

    fn token_len(&self, start_len: usize) -> u32 {
        (start_len - self.0.as_str().len()) as u32
    }

    fn line_comment(&mut self) -> Kind {
        // Check if comment is docs: `/// docs`
        // `//// comment` - not docs
        let kind = if self.first() == '/' && self.second() != '/' {
            Kind::Docs
        } else {
            Kind::LineComment
        };
        // Consume comment
        self.skip_while(|c| c != '\n');
        // Skip `\n`
        self.skip();
        kind
    }

    fn block_comment(&mut self) -> Kind {
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

        Kind::BlockComment { terminated }
    }

    fn whitespace(&mut self, first: char) -> Kind {
        let mut newline = first == '\n';
        self.skip_while(|c| {
            if c == '\n' {
                newline = true;
                return true;
            }
            c.is_whitespace()
        });
        Kind::WhiteSpace { newline }
    }
}

impl<'a> Cursor<'a> {
    fn ident(&mut self) -> Kind {
        self.skip_while(is_ident_continue);
        if !self.first().is_ascii() {
            return self.invalid_ident();
        }
        Kind::Ident
    }

    fn invalid_ident(&mut self) -> Kind {
        self.skip_while(char::is_alphanumeric);
        Kind::InvalidIdent
    }
}

#[derive(Debug, Clone, Copy)]
struct JustZero;

impl<'a> Cursor<'a> {
    fn parse_base(&mut self) -> Result<Base, JustZero> {
        // We've already missed the starting `0`
        // Check special character
        let base = match self.first() {
            'b' => Base::Binary,
            'o' => Base::Octal,
            'x' => Base::Hexadecimal,
            // number continue
            '0'..='9' => return Ok(Base::Decimal),
            // Just zero
            _ => return Err(JustZero),
        };
        // Skip special character
        self.skip();
        Ok(base)
    }

    fn parse_number(&mut self, base: Base) -> LitKind {
        // We already have at least one number
        if matches!(base, Base::Decimal) {
            self.skip_decimal();
        } else {
            if matches!(self.skip_hex(), HasDigits::No) {
                return LitKind::empty_int();
            }
        }
        self.parse_maybe_float(base)
    }

    fn parse_decimal_number(&mut self) -> LitKind {
        self.skip_decimal();
        self.parse_maybe_float(Base::Decimal)
    }

    fn parse_maybe_float(&mut self, base: Base) -> LitKind {
        let [first, second] = self.two();
        if first == '.' && second != '.' && !second.is_ascii_alphabetic() {
            self.skip();
            self.skip_decimal();
            LitKind::Float { base }
        } else {
            LitKind::Int { base, empty: false }
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
                '0'..='9' => has_digits = HasDigits::Yes,
                '_' => (),
                _ => break,
            }
            self.skip();
        }
        has_digits
    }

    fn skip_hex(&mut self) -> HasDigits {
        let mut has_digits = HasDigits::No;
        loop {
            match self.first() {
                '0'..='9' | 'a'..='f' | 'A'..='F' => has_digits = HasDigits::Yes,
                '_' => (),
                _ => break,
            }
            self.skip();
        }
        has_digits
    }
}

impl<'a> Cursor<'a> {
    fn char(&mut self) -> LitKind {
        if self.second() == '\'' && self.first() != '\\' {
            // skip char
            self.skip();
            // skip '\''
            self.skip();
            return LitKind::Char { terminated: true };
        }

        loop {
            if self.is_eof() {
                break;
            }

            match self.first() {
                // end '\''
                '\'' => {
                    self.skip();
                    return LitKind::Char { terminated: true };
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

        LitKind::Char { terminated: false }
    }

    fn str(&mut self) -> LitKind {
        loop {
            if self.is_eof() {
                break;
            }

            match self.first() {
                // end `"`
                '"' => {
                    self.skip();
                    return LitKind::Str { terminated: true };
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

        LitKind::Str { terminated: false }
    }
}

impl<'a> Cursor<'a> {
    fn unknown(&mut self) -> Kind {
        self.skip_while(|c| !c.is_ascii() && c.is_alphanumeric());
        Kind::Unknown
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
        self.0.as_str().is_empty()
    }
}

impl<'a> Cursor<'a> {
    /// Returns the first character and moving to the next character.
    #[inline]
    pub fn next(&mut self) -> Option<char> {
        self.0.next()
    }

    /// Returns the first character without moving to the next character.
    #[inline]
    pub fn peek(&mut self) -> Option<char> {
        self.0.clone().next()
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
        let mut chars = self.0.clone();
        chars.next();
        chars.next().unwrap_or(EOF)
    }

    pub fn two(&mut self) -> [char; 2] {
        let mut chars = self.0.clone();
        let first = chars.next().unwrap_or(EOF);
        let second = chars.next().unwrap_or(EOF);
        [first, second]
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
    let source = r#"a * b + c"#.trim();
    let mut cursor = Cursor::new(source);
    let mut index = 0;
    loop {
        let token = cursor.advance_token();
        let end = index + token.len as usize;
        let span = index..end;
        println!("{:?} {:?}", token.kind, &source[span]);
        if let Kind::Eof = token.kind {
            break;
        }
        index = end;
    }
}
