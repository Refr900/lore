use crate::lexer::Kind;

use super::{AssignExt, BlockExt, ExpressionExt, IfExt, VariableExt};
use super::{ExprKind, Parse, Parser};

mod utils;
pub use utils::*;

mod var;
pub use var::*;

mod assign;
pub use assign::*;

mod block;
pub use block::*;

mod if_stmt;
pub use if_stmt::*;

mod stmts;
pub use stmts::*;

#[derive(Debug, Clone)]
pub enum StmtKind {
    Expr(ExprKind),
    Var(VarStmt),
    Assign(AssignStmt),
    Block(BlockStmt),
    If(IfStmt),
}

#[derive(Debug, Clone)]
pub enum ParseStmtError {
    Expr,
    Var,
    Assign,
    Block,
    If,
    Eof,
}

impl From<()> for ParseStmtError {
    fn from(_: ()) -> Self {
        Self::Eof
    }
}

impl Parse for StmtKind {
    type Parsed = Self;
    type Error = ParseStmtError;

    fn parse(parser: &mut Parser<'_>) -> Result<Self::Parsed, ParseStmtError> {
        let token = parser.peek()?;
        if matches!(token.kind, Kind!['{']) {
            let block = match parser.parse_block() {
                Ok(block) => block,
                Err(_) => return Err(ParseStmtError::Block),
            };
            return Ok(Self::Block(block));
        }

        if matches!(token.kind, Kind![if]) {
            let if_stmt = match parser.parse_if() {
                Ok(if_stmt) => if_stmt,
                Err(_) => return Err(ParseStmtError::If),
            };
            return Ok(Self::If(if_stmt));
        }

        let token = parser.with_frame(|parser| {
            parser.stream.maybe(Kind![pub]);
            parser.peek()
        })?;
        if matches!(token.kind, Kind![const] | Kind![let] | Kind![mut]) {
            let var = match parser.parse_variable() {
                Ok(var) => var,
                Err(_) => return Err(ParseStmtError::Var),
            };
            return Ok(Self::Var(var));
        }

        let expr = parser.parse_expression()?;
        let token = parser.peek()?;

        if matches!(token.kind, Kind::Assign(_)) {
            let assign = match parser.parse_assign() {
                Ok(assign) => assign,
                Err(_) => return Err(ParseStmtError::Assign),
            };
            return Ok(Self::Assign(assign));
        }
        Ok(Self::Expr(expr))
    }
}
