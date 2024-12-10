use crate::parser::{Parse, Parser, PathExpr, ValueExt};

use super::ExprKind;

#[derive(Debug, Clone)]
pub struct CallExpr {
    pub path: PathExpr,
}

impl CallExpr {
    pub fn new(path: PathExpr) -> Self {
        Self { path }
    }
}

impl Parse for CallExpr {
    type Parsed = ExprKind;
    type Error = ();

    fn parse(parser: &mut Parser<'_>) -> Result<ExprKind, ()> {
        let mut snapshot = parser.clone();
        let call = match snapshot.parse::<PathExpr>() {
            Ok(path) => {
                *parser = snapshot;
                ExprKind::Path(path)
            }
            Err(_) => parser.parse_value()?,
        };
        Ok(call)
    }
}

