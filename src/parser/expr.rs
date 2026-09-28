use crate::{lexer::tokens::FStringPart, primitives::range::Range};

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
  MatchExpr(Vec<FStringPart>),
  FStringExpr(Vec<FStringPart>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
  pub kind: ExprKind,
  pub range: Range,
}
