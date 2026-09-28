use crate::{lexer::tokens::FStringPart, parser::expr::Expr, primitives::range::Range};

#[derive(Debug, Clone, PartialEq)]
pub enum TaskStmt {
  Run(Vec<FStringPart>),
  Needs(Vec<String>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum StmtKind {
  Var { name: String, expr: Expr },

  Task { name: String, body: Vec<TaskStmt> },

  Expr(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Stmt {
  pub kind: StmtKind,
  pub range: Range,
}
