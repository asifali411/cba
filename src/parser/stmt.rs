use crate::{lexer::tokens::FStringPart, primitives::range::Range};

#[derive(Debug, Clone, PartialEq)]
pub enum TaskStmt {
  Run(Vec<FStringPart>),
  Needs(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum StmtKind {
  Var {
    name: String,
    value: Vec<FStringPart>,
  },

  Task {
    name: String,
    body: Vec<TaskStmt>,
  },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Stmt {
  pub kind: StmtKind,
  pub range: Range,
}
