use crate::lexer::tokens::FStringPart;

#[derive(Debug, Clone, PartialEq)]
pub enum TaskStmt {
  Run(Vec<FStringPart>),
  Needs(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
  Var {
    name: String,
    value: Vec<FStringPart>,
  },

  Task {
    name: String,
    body: Vec<TaskStmt>,
  },
}
