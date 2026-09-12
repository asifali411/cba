use std::collections::HashMap;

use crate::{
  analyzer::task_plan::TaskPlan,
  lexer::tokens::FStringPart,
  parser::stmt::{Stmt, TaskStmt},
  primitives::result::AResult,
};

pub struct Analyzer {
  statements: Vec<Stmt>,
  pub variables: HashMap<String, String>,
  pub tasks: HashMap<String, TaskPlan>,
}

impl Analyzer {
  pub fn new(statements: &[Stmt]) -> Self {
    Self {
      statements: statements.to_vec(),
      variables: HashMap::new(),
      tasks: HashMap::new(),
    }
  }

  pub fn analyze(&mut self) -> AResult<&HashMap<String, TaskPlan>> {
    for stmt in self.statements.clone() {
      match stmt {
        Stmt::Var { name, value } => {
          let value = &self.resolve_fstring(value.to_vec())?;
          self.variables.insert(name.clone(), value.clone());
        }

        Stmt::Task { name, body } => {
          self.resolve_task(&name, &body)?;
        }
      }
    }

    Ok(&self.tasks)
  }

  fn resolve_task(&mut self, name: &String, body: &Vec<TaskStmt>) -> AResult<()> {
    let mut task = TaskPlan::new(name.clone());

    for stmt in body {
      match stmt {
        TaskStmt::Needs(d) => task.dependencies.push(d.into()),
        TaskStmt::Run(c) => task.commands.push(self.resolve_fstring(c.to_vec())?),
      }
    }

    self.tasks.insert(name.clone(), task);
    Ok(())
  }

  fn resolve_fstring(&mut self, fstring: Vec<FStringPart>) -> AResult<String> {
    let mut value = String::new();

    for part in fstring {
      match part {
        FStringPart::Text(t) => value.push_str(&t),
        FStringPart::Ident(i) => match self.variables.get(&i) {
          Some(v) => value.push_str(v),
          None => return Err(format!("Cannot find variable {}", i)),
        },
      };
    }

    Ok(value)
  }
}
