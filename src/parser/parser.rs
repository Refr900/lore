use crate::lexer::{BinaryKind, Kind, TokenId};

use super::{
    BinaryExpr, ExpectedItem, ExprKind, Item, ItemKind, ItemSequence, Parse, ParseError, Token,
    TokenStream,
};

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    pub stream: TokenStream<'a>,
    pub errors: Vec<ParseError>,
}

#[derive(Debug, Clone, Copy)]
pub struct ParserFrame {
    current: TokenId,
    error_count: usize,
}

impl ParserFrame {
    pub fn new(current: TokenId, error_count: usize) -> Self {
        Self {
            current,
            error_count,
        }
    }
}

impl<'a> Parser<'a> {
    pub fn new(kinds: &'a [Kind]) -> Self {
        Self {
            stream: TokenStream::new(kinds),
            errors: Vec::new(),
        }
    }
}

impl<'a> Parser<'a> {
    pub fn parse<P: Parse>(&mut self) -> Result<P::Parsed, P::Error> {
        P::parse(self)
    }

    pub fn push_error<E>(&mut self, error: E)
    where
        E: Into<ParseError>,
    {
        self.errors.push(error.into());
    }
}

impl<'a> Parser<'a> {
    pub fn set_frame(&mut self, frame: ParserFrame) {
        self.stream.set_current(frame.current);
        debug_assert!(self.errors.len() >= frame.error_count);
        while self.errors.len() != frame.error_count {
            self.errors.pop();
        }
    }

    pub fn frame(&self) -> ParserFrame {
        ParserFrame::new(self.stream.current_id(), self.errors.len())
    }

    pub fn with_frame<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut Parser<'a>) -> R,
    {
        let frame = self.frame();
        let result = f(self);
        self.set_frame(frame);
        result
    }
}

impl<'a> Parser<'a> {
    pub fn parse_binary<'r, F>(&mut self, mut f: F, operators: &'r [Kind]) -> Result<ExprKind, ()>
    where
        F: FnMut(&mut Parser<'a>) -> Result<ExprKind, ()>,
    {
        let mut node = f(self)?;
        loop {
            let token = match self.stream.expect_any(operators) {
                Ok(token) => token,
                Err(_) => return Ok(node),
            };

            match token {
                Token {
                    id: _,
                    kind: Kind::Binary(kind),
                } => {
                    let Ok(rhs) = f(self) else {
                        self.push_error(ExpectedItem::after(
                            // binary operators work with statements
                            ItemSequence::StmtWithReturnValue,
                            Item::from_token(self.stream.second()),
                        ));
                        return Ok(node);
                    };
                    node = ExprKind::Binary(BinaryExpr::new(kind, Box::new(node), Box::new(rhs)));
                }
                _ => return Ok(node),
            }
        }
    }
}

impl<'a> Parser<'a> {
    pub fn expect(&mut self, kind: Kind) -> Result<TokenId, ()> {
        self.stream.expect(kind)
    }

    pub fn peek(&mut self) -> Result<Token, ()> {
        let token = self.stream.first();
        if let Kind::Eof = token.kind {
            return Err(());
        }
        Ok(token)
    }
}

/*
impl<'a> Parser<'a> {
    pub fn parse(mut self) -> (Vec<DeclKind>, Vec<ParseError<'a>>) {
        let mut declarations = Vec::new();
        let mut errors = Vec::new();
        loop {
            match self.parse_decl() {
                Ok(decl) => declarations.push(decl),
                Err(ParseError {
                    kind: ParseErrorKind::Eof,
                    ..
                }) => break,
                Err(err) => errors.push(err),
            }
        }
        (declarations, errors)
    }

    pub fn parse_decl(&mut self) -> Result<DeclKind, ParseError> {
        let current = self.current;
        let vis = self.parse_visibility()?;
        let token = self.peek()?;
        self.current = current;
        let stmt = match token.kind {
            Kind![let] | Kind![const] | Kind::Ident => match self.parse_stmt() {
                Ok(stmt) => {
                    self.expect(Kind![;])?;
                    DeclKind::Statement(stmt)
                }
                Err(err) => {
                    loop {
                        let token = self.next()?;
                        if matches!(token.kind, Kind![;]) {
                            break;
                        }
                    }
                    return Err(err);
                }
            },
            Kind![fn] => self.parse_fn(vis)?,
            _ => {
                self.skip();
                return Err(ParseError::unexpected(token));
            }
        };
        Ok(stmt)
    }

    fn parse_fn(&mut self, vis: Vis) -> Result<DeclKind, ParseError> {
        self.expect(Kind![fn])?;
        let name = self.expect(Kind::Ident)?;
        self.expect(Kind!['('])?;
        // TODO: parse args
        self.expect(Kind![')'])?;
        self.expect(Kind!['{'])?;
        let mut inner = Vec::new();
        loop {
            let Ok(token) = self.peek() else {
                return Ok(DeclKind::Fn(FnDecl { vis, name, inner }));
            };
            if let Kind!['}'] = token.kind {
                self.skip();
                break;
            }
            match self.parse_stmt() {
                Ok(stmt) => {
                    inner.push(stmt);
                    self.expect(Kind![;])?;
                }
                Err(err) => {
                    self.errors.push(err);
                    loop {
                        let Ok(token) = self.peek() else {
                            return Ok(DeclKind::Fn(FnDecl { vis, name, inner }));
                        };
                        if let Kind![;] = token.kind {
                            break;
                        }
                        self.skip();
                    }
                }
            }
        }
        Ok(DeclKind::Fn(FnDecl { vis, name, inner }))
    }
}

// region: ---- Statement ----

impl<'a> Parser<'a> {
    pub fn parse_stmts(&mut self) -> Result<Vec<StmtKind>, ParseError> {
        let mut inner = Vec::new();
        loop {
            let token = self.peek()?;
            if let Kind!['}'] = token.kind {
                break;
            }
            match Self::parse_stmt(self) {
                Ok(stmt) => {
                    inner.push(stmt);
                    self.expect(Kind![;])?;
                }
                Err(err) => {
                    self.errors.push(err);
                    loop {
                        let token = self.peek()?;
                        if let Kind![;] = token.kind {
                            break;
                        }
                        self.skip();
                    }
                }
            }
        }
        Ok(inner)
    }

    pub fn parse_stmt(&mut self) -> Result<StmtKind, ParseError> {
        let vis = self.parse_visibility()?;
        let token = self.peek()?;
        let stmt = match token.kind {
            Kind![let] | Kind![const] => self.parse_inner_variable(vis)?,
            Kind::Ident => {
                if let Ok(assign) = self.parse_assign() {
                    StmtKind::Assign(assign)
                } else {
                    let expr = self.parse_expr()?;
                    StmtKind::Expr(expr)
                }
            }
            _ => {
                self.skip();
                return Err(ParseError::unexpected(token));
            }
        };
        Ok(stmt)
    }

    fn parse_variable(&mut self) -> Result<StmtKind, ParseError> {
        let vis = self.parse_visibility()?;
        self.parse_inner_variable(vis)
    }

    fn parse_inner_variable(&mut self, vis: Vis) -> Result<StmtKind, ParseError> {
        let token = self.peek()?;
        let kind = match token.kind {
            Kind![let] => VarKind::Let,
            Kind![const] => VarKind::Const,
            _ => return Err(ParseError::unexpected(token)),
        };
        self.skip();
        let mutability = self.parse_mutability()?;
        let name = self.expect(Kind::Ident)?;
        self.expect(Kind::Eq)?;
        let expr = self.parse_expr()?;
        Ok(StmtKind::Var(VarStmt {
            vis,
            kind,
            mutability,
            name,
            expr,
        }))
    }

    fn parse_assign(&mut self) -> Result<Assign, ParseError> {
        let current = self.current;
        let call = self.parse_call()?;
        let token = self.peek()?;
        let kind = match token.kind {
            Kind::Eq => AssignKind::Eq,
            Kind::BinaryEq(token) => AssignKind::from_binary(token),
            _ => {
                self.current = current;
                return Err(ParseError::unexpected(token));
            }
        };
        self.skip();
        let op = AssignOp { kind };
        let expr = self.parse_expr()?;
        self.expect(Kind![;])?;
        Ok(Assign { call, op, expr })
    }

    fn parse_visibility(&mut self) -> Result<Vis, ParseError> {
        Ok(if self.skip_if_peek_eq(Kind![pub])? {
            Vis::Public
        } else {
            Vis::Private
        })
    }

    fn parse_mutability(&mut self) -> Result<Mut, ParseError> {
        Ok(if self.skip_if_peek_eq(Kind![mut])? {
            Mut::Yes
        } else {
            Mut::No
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
        self.parse_binary(Self::parse_logic_and, &[Kind![||]])
    }

    fn parse_logic_and(&mut self) -> Result<ExprKind, ParseError> {
        self.parse_binary(Self::parse_equality, &[Kind![&&]])
    }

    fn parse_equality(&mut self) -> Result<ExprKind, ParseError> {
        self.parse_binary(Self::parse_comparison, &[Kind![==], Kind![!=]])
    }

    fn parse_comparison(&mut self) -> Result<ExprKind, ParseError> {
        self.parse_binary(
            Self::parse_term,
            &[Kind![<], Kind![>], Kind![<=], Kind![>=]],
        )
    }

    fn parse_term(&mut self) -> Result<ExprKind, ParseError> {
        self.parse_binary(Self::parse_factor, &[Kind![+], Kind![-]])
    }

    fn parse_factor(&mut self) -> Result<ExprKind, ParseError> {
        self.parse_binary(Self::parse_power, &[Kind![*], Kind![/]])
    }

    fn parse_power(&mut self) -> Result<ExprKind, ParseError> {
        self.parse_binary(Self::parse_unary, &[Kind![^]])
    }

    fn parse_binary<F>(&mut self, mut f: F, operators: &[Kind]) -> Result<ExprKind, ParseError>
    where
        F: FnMut(&mut Parser<'a>) -> Result<ExprKind, ParseError>,
    {
        let mut node = f(self)?;
        loop {
            match self.expect_any(operators) {
                Ok((_, Kind::Binary(token))) => {
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
                Kind::Not => UnaryOp::Not,
                Kind::Binary(token) => UnaryOp::from_binary(token),
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
        let start = self.expect(Kind::Ident)?;
        let mut mod_len = 0;
        loop {
            if self.expect(Kind![::]).is_err() {
                break;
            }
            self.expect(Kind::Ident)?;
            mod_len += 1;
        }

        let mut var_len = 0;
        loop {
            if self.expect(Kind![.]).is_err() {
                break;
            }
            self.expect(Kind::Ident)?;
            var_len += 1;
        }

        Ok(ExprKind::Path(PathExpr::new(start, mod_len, var_len)))
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
        if let Kind::Lit(_) = token.kind {
            Ok(ExprKind::Literal(self.advance()))
        } else {
            Err(ParseError::unexpected(token))
        }
    }
}

// endregion: ---- Expressions ----

impl<'a> Parser<'a> {
    fn parse_delimiters_with<F, R>(&mut self, delimiter: Delimiter, f: F) -> Result<R, ParseError>
    where
        F: FnOnce(&mut Self) -> Result<R, ParseError>,
    {
        self.expect(Kind::OpenDelim(delimiter))?;
        let result = f(self)?;
        self.expect(Kind::CloseDelim(delimiter))?;
        Ok(result)
    }
}
*/
