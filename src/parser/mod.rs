use crate::lexer::{BinaryToken, Delimiter, Keyword, Lexer, Span, Token, TokenId, TokenKind};

mod declarations;
mod expressions;
mod statements;

pub use declarations::*;
pub use expressions::*;
pub use statements::*;

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    source: &'a str,
    tokens: &'a [Token],
    errors: Vec<ParseError>,
    current: usize,
}

#[derive(Debug, Clone)]
pub enum ParseError {
    Expected {
        expected: TokenKind,
        found: TokenKind,
    },
    Unexpected(Token),
    Eof,
}

impl<'a> Parser<'a> {
    pub fn from_tokens(source: &'a str, tokens: &'a [Token]) -> Self {
        Self {
            source,
            tokens,
            errors: Vec::new(),
            current: 0,
        }
    }
}

impl<'a> Parser<'a> {
    pub fn parse(mut self) -> (Vec<DeclKind>, Vec<ParseError>) {
        let mut declarations = Vec::new();
        loop {
            match self.parse_decl() {
                Ok(decl) => declarations.push(decl),
                Err(ParseError::Eof) => break,
                Err(err) => self.errors.push(err),
            }
        }
        (declarations, self.errors)
    }

    pub fn parse_decl(&mut self) -> Result<DeclKind, ParseError> {
        Ok(DeclKind::Statement(self.parse_stmt()?))
    }
}

// region: ---- Statement ----

impl<'a> Parser<'a> {
    pub fn parse_stmts(&mut self) -> Result<Vec<StmtKind>, ParseError> {
        self.parse_punctuated(Token![;], Self::parse_stmt)
    }

    pub fn parse_stmt(&mut self) -> Result<StmtKind, ParseError> {
        let token = self.peek()?;
        let stmt = match token.kind {
            Token![let] | Token![const] => self.parse_variable()?,
            TokenKind::Ident => {
                if self.lexeme(token.span) == "print" {
                    self.skip();
                    StmtKind::Print(self.parse_expr_in_parens()?)
                } else if let Ok(assign) = self.clone().parse_assign() {
                    assign
                } else {
                    StmtKind::Expr(self.parse_expr()?)
                }
            }
            _ => {
                self.skip();
                return Err(ParseError::Unexpected(token));
            }
        };
        self.expect(Token![;])?;
        Ok(stmt)
    }

    fn parse_variable(&mut self) -> Result<StmtKind, ParseError> {
        let visibility = self.parse_visibility()?;
        let mutability = self.parse_variable_mutability()?;
        let name = self.expect(TokenKind::Ident)?;
        self.expect(TokenKind::Eq)?;
        let expr = self.parse_expr()?;
        Ok(StmtKind::Variable(Variable {
            visibility,
            mutability,
            name,
            expr,
        }))
    }

    fn parse_assign(&mut self) -> Result<StmtKind, ParseError> {
        let call = self.parse_call()?;
        let token = self.peek()?;
        let kind = match token.kind {
            TokenKind::BinaryEq(binary) => binary,
            _ => return Err(ParseError::Unexpected(token)),
        };
        let op = AssignOp { kind };
        Ok(StmtKind::Assign(Assign {
            call,
            op,
            expr: self.parse_expr()?,
        }))
    }

    fn parse_visibility(&mut self) -> Result<Visibility, ParseError> {
        Ok(if self.skip_if_peek_eq(Token![pub])? {
            Visibility::Public
        } else {
            Visibility::Private
        })
    }

    fn parse_variable_mutability(&mut self) -> Result<Mutability, ParseError> {
        let token = self.peek()?;
        let mutability = match token.kind {
            Token![let] => {
                self.skip();
                self.parse_mutability()?
            }
            Token![const] => {
                self.skip();
                Mutability::Const
            }
            _ => return Err(ParseError::Unexpected(token)),
        };
        Ok(mutability)
    }

    fn parse_mutability(&mut self) -> Result<Mutability, ParseError> {
        Ok(if self.skip_if_peek_eq(Token![mut])? {
            Mutability::Mut
        } else {
            Mutability::Let
        })
    }
}

// endregion: ---- Statement ----

// region: ---- Expressions ----

impl<'a> Parser<'a> {
    pub fn parse_expr(&mut self) -> Result<ExprKind, ParseError> {
        self.parse_logic_or()
    }
}

// Binary Operators
impl<'a> Parser<'a> {
    fn parse_logic_or(&mut self) -> Result<ExprKind, ParseError> {
        self.parse_binary(Self::parse_logic_and, &[Token![||]])
    }

    fn parse_logic_and(&mut self) -> Result<ExprKind, ParseError> {
        self.parse_binary(Self::parse_equality, &[Token![&&]])
    }

    fn parse_equality(&mut self) -> Result<ExprKind, ParseError> {
        self.parse_binary(Self::parse_comparison, &[Token![==], Token![!=]])
    }

    fn parse_comparison(&mut self) -> Result<ExprKind, ParseError> {
        self.parse_binary(
            Self::parse_term,
            &[Token![<], Token![>], Token![<=], Token![>=]],
        )
    }

    fn parse_term(&mut self) -> Result<ExprKind, ParseError> {
        self.parse_binary(Self::parse_factor, &[Token![+], Token![-]])
    }

    fn parse_factor(&mut self) -> Result<ExprKind, ParseError> {
        self.parse_binary(Self::parse_power, &[Token![*], Token![/]])
    }

    fn parse_power(&mut self) -> Result<ExprKind, ParseError> {
        self.parse_binary(Self::parse_unary, &[Token![^]])
    }

    fn parse_binary<F>(&mut self, mut f: F, operators: &[TokenKind]) -> Result<ExprKind, ParseError>
    where
        F: FnMut(&mut Parser<'a>) -> Result<ExprKind, ParseError>,
    {
        let mut node = f(self)?;
        loop {
            match self.expect_any(operators) {
                Ok((_, TokenKind::Binary(token))) => {
                    let rhs = f(self)?;
                    let kind = BinaryKind::from_token(token);
                    node = ExprKind::Binary(BinaryExpr::new(
                        BinaryOp::new(kind),
                        Box::new(node),
                        Box::new(rhs),
                    ));
                }
                _ => return Ok(node),
            }
        }
    }
}

impl<'a> Parser<'a> {
    fn parse_unary(&mut self) -> Result<ExprKind, ParseError> {
        let mut operators = Vec::new();
        let expr = loop {
            let token = self.peek()?;
            let op = match token.kind {
                TokenKind::Not => UnaryOp::Not,
                TokenKind::Binary(token) => UnaryOp::from_binary(token),
                _ => break self.parse_call()?,
            };
            operators.push(op);
            self.skip();
        };

        Ok(if !operators.is_empty() {
            let unary = UnaryExpr::new(operators, Box::new(expr));
            ExprKind::Unary(unary)
        } else {
            expr
        })
    }

    fn parse_call(&mut self) -> Result<ExprKind, ParseError> {
        match self.parse_path() {
            Ok(value) => Ok(value),
            Err(_) => self.parse_value(),
        }
    }

    fn parse_path(&mut self) -> Result<ExprKind, ParseError> {
        let start = self.expect(TokenKind::Ident)?;
        let mut count = 1;
        loop {
            if self.expect(Token![::]).is_err() {
                break;
            }
            self.expect(TokenKind::Ident)?;
            count += 1;
        }

        Ok(ExprKind::Path(PathExpr::new(start, count)))
    }

    fn parse_value(&mut self) -> Result<ExprKind, ParseError> {
        if let Ok(expr) = self.parse_expr_in_parens() {
            return Ok(expr);
        }
        self.parse_lit()
    }

    fn parse_expr_in_parens(&mut self) -> Result<ExprKind, ParseError> {
        self.parse_delimiters_with(Delimiter::Paren, |parser| parser.parse_expr())
    }

    pub fn parse_lit(&mut self) -> Result<ExprKind, ParseError> {
        let token = self.peek()?;
        if let TokenKind::Literal(_) = token.kind {
            Ok(ExprKind::Literal(self.advance()))
        } else {
            Err(ParseError::Unexpected(token))
        }
    }
}

// endregion: ---- Expressions ----

impl<'a> Parser<'a> {
    pub fn skip_while<F>(&mut self, mut predicate: F)
    where
        F: FnMut(Token) -> bool,
    {
        while let Ok(token) = self.peek() {
            if !predicate(token) {
                break;
            }
            self.skip();
        }
    }

    fn parse_punctuated<F, R>(
        &mut self,
        punctuation: TokenKind,
        mut f: F,
    ) -> Result<Vec<R>, ParseError>
    where
        F: FnMut(&mut Self) -> Result<R, ParseError>,
    {
        let mut r = Vec::new();
        loop {
            r.push(f(self)?);
            if let Err(_) = self.expect(punctuation) {
                break;
            }
        }
        Ok(r)
    }

    fn parse_delimiters_with<F, R>(&mut self, delimiter: Delimiter, f: F) -> Result<R, ParseError>
    where
        F: FnOnce(&mut Self) -> Result<R, ParseError>,
    {
        self.expect(TokenKind::OpenDelim(delimiter))?;
        let result = f(self)?;
        self.expect(TokenKind::CloseDelim(delimiter))?;
        Ok(result)
    }

    fn expect_any(&mut self, tokens: &[TokenKind]) -> Result<(TokenId, TokenKind), ParseError> {
        let token_id = self.token_id();
        let token = self.peek()?;
        for expected in tokens.iter().copied() {
            if expected == token.kind {
                self.skip();
                return Ok((token_id, token.kind));
            }
        }
        Err(ParseError::Unexpected(token))
    }

    fn expect(&mut self, expected: TokenKind) -> Result<TokenId, ParseError> {
        let token = self.peek()?;
        if expected == token.kind {
            Ok(self.advance())
        } else {
            Err(ParseError::Expected {
                expected,
                found: token.kind,
            })
        }
    }

    fn lexeme(&self, span: Span) -> &str {
        &self.source[span.as_range()]
    }
}

impl<'a> Parser<'a> {
    fn advance_by(&mut self, count: usize) -> TokenId {
        let token_id = self.token_id();
        self.skip_by(count);
        token_id
    }

    fn skip_by(&mut self, count: usize) {
        self.current += count;
    }

    fn advance(&mut self) -> TokenId {
        self.advance_by(1)
    }

    fn skip(&mut self) {
        self.skip_by(1);
    }

    fn token_id(&self) -> TokenId {
        TokenId(self.current)
    }

    fn peek(&self) -> Result<Token, ParseError> {
        self.peek_by(self.current)
    }

    fn skip_if_peek_eq(&mut self, expected: TokenKind) -> Result<bool, ParseError> {
        let token = self.peek()?;
        Ok(if expected == token.kind {
            self.skip();
            true
        } else {
            false
        })
    }

    fn peek_eq(&self, expected: TokenKind) -> Result<bool, ParseError> {
        Ok(expected == self.peek()?.kind)
    }

    fn peek_by(&self, index: usize) -> Result<Token, ParseError> {
        match self.tokens.get(index) {
            Some(token) => Ok(token.clone()),
            None => Err(ParseError::Eof),
        }
    }
}

#[test]
fn test() {
    let source = r#"
let mut health = 50;
health = 10;
"#;
    let (tokens, errors) = Lexer::tokenize(source);
    println!("{:#?}", errors);
    let parser = Parser::from_tokens(&source, &tokens);
    println!("{:#?}", parser.parse());
}
