use std::collections::HashMap;

use crate::{analyzer::task_plan::TaskPlan, parser::stmt::Stmt};

pub struct Analyzer {
  statements: Vec<Stmt>,
  pub tasks: HashMap<String, TaskPlan>,
}

impl Analyzer {
  pub fn new(statements: &[Stmt]) -> Self {
    Self {
      statements: statements.to_vec(),
      tasks: HashMap::new(),
    }
  }

  pub fn analyze(&mut self) {

  }
}
