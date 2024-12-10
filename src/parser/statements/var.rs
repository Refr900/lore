use crate::{
    lexer::{Kind, TokenId},
    parser::{
        Parse, ParseError, Parser, Span, StatementExt, SyntaxError, TypePathExpr, TypePathExt,
    },
};

use super::{Mut, StmtKind, Vis};

#[derive(Debug, Clone)]
pub struct VarStmt {
    pub span: Span,
    pub vis: Vis,
    pub kind: VarKind,
    pub mutability: Mut,
    pub name: TokenId,
    pub type_path: Option<TypePathExpr>,
    pub stmt_start: TokenId,
    pub stmt: Option<Box<StmtKind>>,
}

impl Parse for VarStmt {
    type Parsed = Self;
    type Error = ();

    fn parse(parser: &mut Parser<'_>) -> Result<Self, ()> {
        let mut start = None;
        let vis = match parser.stream.maybe(Kind![pub]) {
            Some(id) => {
                start = Some(id);
                Vis::Public
            }
            None => Vis::Private,
        };

        let token = parser.peek()?;
        parser.stream.skip();
        let start = if let Some(start) = start {
            start
        } else {
            token.id
        };

        let kind = match token.kind {
            Kind![const] => VarKind::Const,
            Kind![let] => VarKind::Let,
            _ => return Err(()),
        };

        let mutability = match parser.stream.maybe(Kind![mut]) {
            Some(_) => Mut::Yes,
            None => Mut::No,
        };

        let name = match parser.stream.expect(Kind::Ident) {
            Ok(id) => id,
            Err(err) => {
                parser.errors.push(err);
                return Err(());
            }
        };

        let type_path = match parser.stream.maybe(Kind![:]) {
            Some(_) => Some(parser.parse_type_path()?),
            None => None,
        };

        let token = parser.stream.first();
        if !matches!(token.kind, Kind![=]) {
            return Ok(Self {
                span: Span::new(start, token.id),
                vis,
                mutability,
                kind,
                name,
                type_path: None,
                stmt_start: token.id,
                stmt: None,
            });
        }
        parser.stream.skip();
        let stmt_start = parser.stream.current_id();
        let Ok(stmt) = parser.parse_statement() else {
            return Err(());
        };
        let end = parser.stream.current_id();
        if let StmtKind::Var(var) = &stmt {
            let value = ParseError::Syntax(SyntaxError::unvalid_assignment(var.span));
            parser.errors.push(value);
        }

        Ok(Self {
            span: Span::new(start, end),
            vis,
            mutability,
            kind,
            name,
            type_path,
            stmt_start,
            stmt: Some(Box::new(stmt)),
        })
    }
}

#[derive(Debug, Clone)]
pub enum VarKind {
    Const,
    Let,
    Unknown,
}
