use crate::{
    lexer::{self, Kind, BinaryKind},
    parser::{
        BinAndExt, BinOrExt, ComparisonExt, EqualityExt, FactorExt, LogicAndExt, Parse, Parser,
        PowerExt, TermExt, UnaryExt,
    },
};

use super::ExprKind;

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub op: BinaryKind,
    pub rhs: Box<ExprKind>,
    pub lhs: Box<ExprKind>,
}

impl BinaryExpr {
    pub const fn new(op: BinaryKind, rhs: Box<ExprKind>, lhs: Box<ExprKind>) -> Self {
        Self { op, rhs, lhs }
    }
}

macro_rules! binary {
    {
        name: $name:ident,
        fun: $fun:expr,
        operators: [$($op:expr),*] $(,)?
    } => {paste::paste! {
        #[derive(Debug, Clone, Copy)]
        pub struct [<$name Expr>];

        impl Parse for [<$name Expr>] {
            type Parsed = ExprKind;
            type Error = ();

            fn parse(parser: &mut Parser<'_>) -> Result<ExprKind, ()> {
                parser.parse_binary($fun, &[$($op),*])
            }
        }
    }};
}

binary! {
    name: LogicOr,
    fun: Parser::parse_logic_and,
    operators: [Kind![||]],
}

binary! {
    name: LogicAnd,
    fun: Parser::parse_equality,
    operators: [Kind![&&]],
}

binary! {
    name: Equality,
    fun: Parser::parse_comparison,
    operators: [Kind![==], Kind![!=]],
}

binary! {
    name: Comparison,
    fun: Parser::parse_term,
    operators: [Kind![<], Kind![>], Kind![<=], Kind![>=]],
}

binary! {
    name: Term,
    fun: Parser::parse_factor,
    operators: [Kind![+], Kind![-]],
}

binary! {
    name: Factor,
    fun: Parser::parse_bin_or,
    operators: [Kind![*], Kind![/]],
}

binary! {
    name: BinOr,
    fun: Parser::parse_bin_and,
    operators: [Kind![|]],
}

binary! {
    name: BinAnd,
    fun: Parser::parse_power,
    operators: [Kind![&]],
}

binary! {
    name: Power,
    fun: Parser::parse_unary,
    operators: [Kind![^]],
}
