use crate::lexer::{Kind, TokenId};

use super::ParseError;

#[derive(Debug, Clone)]
pub struct TokenStream<'a> {
    kinds: &'a [Kind],
    current: TokenId,
}

#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub id: TokenId,
    pub kind: Kind,
}

impl Token {
    pub fn new(id: TokenId, kind: Kind) -> Self {
        Self { id, kind }
    }
}

impl<'a> TokenStream<'a> {
    pub fn new(kinds: &'a [Kind]) -> Self {
        Self::with_current(kinds, TokenId(0))
    }

    pub fn with_current(kinds: &'a [Kind], current: TokenId) -> Self {
        Self { kinds, current }
    }
}

impl<'a> TokenStream<'a> {
    pub fn set_current(&mut self, current: TokenId) {
        debug_assert!(current.as_index() < self.len());
        self.current = current;
    }
}

impl<'a> TokenStream<'a> {
    pub fn expect_any<'t>(&mut self, kinds: &'t [Kind]) -> Result<Token, ParseError<'t>> {
        let id = self.current_id();
        let token = self.first();
        for expected in kinds.iter().copied() {
            if expected == token.kind {
                self.skip();
                return Ok(Token::new(id, token.kind));
            }
        }
        Err(ParseError::expected_any(kinds, token))
    }

    pub fn expect(&mut self, kind: Kind) -> Result<TokenId, ParseError<'static>> {
        let found = self.first();
        if found.kind == kind {
            Ok(self.next_id())
        } else {
            Err(ParseError::expected(kind, found))
        }
    }

    pub fn maybe(&mut self, kind: Kind) -> Option<TokenId> {
        let found = self.first();
        if found.kind == kind {
            Some(self.next_id())
        } else {
            None
        }
    }
}

impl<'a> TokenStream<'a> {
    pub fn advance(&mut self) -> Token {
        self.skip();
        self.first()
    }

    pub fn peek(&self) -> Result<Token, ParseError<'static>> {
        let token = self.first();
        if let Kind::Eof = token.kind {
            return Err(ParseError::Unexpected(token));
        }
        Ok(token)
    }
}

impl<'a> TokenStream<'a> {
    pub fn next(&mut self) -> Token {
        Token::new(self.current_id(), self.next_kind())
    }

    pub fn next_kind(&mut self) -> Kind {
        let kind = self.first_kind();
        if !matches!(kind, Kind::Eof) {
            self.current.0 += 1;
        }
        kind
    }
}

impl<'a> TokenStream<'a> {
    pub fn two(&self) -> [Token; 2] {
        let mut stream = self.clone();
        [stream.next(), stream.next()]
    }

    pub fn two_kinds(&self) -> [Kind; 2] {
        let mut stream = self.clone();
        [stream.next_kind(), stream.next_kind()]
    }

    pub fn second(&self) -> Token {
        let mut stream = self.clone();
        stream.skip();
        Token::new(self.current_id(), self.first_kind())
    }

    pub fn first(&self) -> Token {
        Token::new(self.current_id(), self.first_kind())
    }

    pub fn first_kind(&self) -> Kind {
        self.kinds.get(self.current.as_index()).copied().unwrap_or(Kind::Eof)
    }
}

impl<'a> TokenStream<'a> {
    pub fn skip(&mut self) {
        self.next_kind();
    }

    pub fn next_id(&mut self) -> TokenId {
        let id = self.current_id();
        self.skip();
        id
    }

    pub fn current_id(&self) -> TokenId {
        self.current
    }
}

impl<'a> TokenStream<'a> {
    pub fn len(&self) -> usize {
        self.kinds.len()
    }

    pub fn remaining_len(&self) -> usize {
        self.remaining().len()
    }

    pub fn remaining(&self) -> &'a [Kind] {
        &self.kinds[self.current.as_index()..]
    }

    pub fn as_slice(&self) -> &'a [Kind] {
        self.kinds
    }
}
