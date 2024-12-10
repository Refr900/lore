use super::{
    AssignStmt, BinAndExpr, BinOrExpr, BlockStmt, CallExpr, ComparisonExpr, EqualityExpr, ExprKind,
    FactorExpr, IfStmt, LitExpr, LogicAndExpr, LogicOrExpr, ParseStmtError, PathExpr, PowerExpr,
    StmtKind, TermExpr, TypePathExpr, UnaryExpr, ValueExpr, VarStmt,
};

macro_rules! parser_ext {
    {
        $($name:ident ($item:path) -> Result<$T:ty, $E:ty>);+;
    } => { paste::paste! {
        $(
            pub trait [<$name Ext>] {
                fn [<parse_ $name:snake>](&mut self) -> Result<$T, $E>;
            }
        )+
        $(
            impl<'a> [<$name Ext>] for $crate::parser::Parser<'a> {
                fn [<parse_ $name:snake>](&mut self) -> Result<$T, $E> {
                    self.parse::<$item>()
                }
            }
        )+
    } };
}

parser_ext! {
    // Exprs
    Lit(LitExpr)               -> Result<LitExpr, ()>;
    TypePath(TypePathExpr)     -> Result<TypePathExpr, ()>;
    Path(PathExpr)             -> Result<PathExpr, ()>;
    Value(ValueExpr)           -> Result<ExprKind, ()>;
    Call(CallExpr)             -> Result<ExprKind, ()>;
    Unary(UnaryExpr)           -> Result<ExprKind, ()>;
    // Binary
    Power(PowerExpr)           -> Result<ExprKind, ()>;
    BinAnd(BinAndExpr)         -> Result<ExprKind, ()>;
    BinOr(BinOrExpr)           -> Result<ExprKind, ()>;
    Factor(FactorExpr)         -> Result<ExprKind, ()>;
    Term(TermExpr)             -> Result<ExprKind, ()>;
    Comparison(ComparisonExpr) -> Result<ExprKind, ()>;
    Equality(EqualityExpr)     -> Result<ExprKind, ()>;
    LogicAnd(LogicAndExpr)     -> Result<ExprKind, ()>;
    LogicOr(LogicOrExpr)       -> Result<ExprKind, ()>;
    Expression(ExprKind)       -> Result<ExprKind, ()>;
    // Stmt
    Variable(VarStmt)          -> Result<VarStmt, ()>;
    Assign(AssignStmt)         -> Result<AssignStmt, ()>;
    Block(BlockStmt)           -> Result<BlockStmt, ()>;
    If(IfStmt)                 -> Result<IfStmt, ()>;
    Statement(StmtKind)        -> Result<StmtKind, ParseStmtError>;
    Statements(Vec<StmtKind>)  -> Result<Vec<StmtKind>, ()>;
}
