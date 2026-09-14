use std::collections::HashMap;

use crate::{
  analyzer::task_plan::TaskPlan,
  lexer::tokens::FStringPart,
  parser::stmt::{Stmt, TaskStmt},
  primitives::{result::AResult, visit_state::VisitState},
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

  pub fn analyze(&mut self, args: Vec<String>) -> AResult<&HashMap<String, TaskPlan>> {
    let args = args.join(" ");
    self.variables.insert("args".into(), args);

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

    self.detect_cycles()?;

    Ok(&self.tasks)
  }

  fn resolve_task(&mut self, name: &String, body: &Vec<TaskStmt>) -> AResult<()> {
    let mut task = TaskPlan::new(name.clone());

    if self.tasks.contains_key(name) {
      return Err(format!(
        "cannot redeclare task. task '{}' is declared multiple times",
        name
      ));
    }

    for stmt in body {
      match stmt {
        TaskStmt::Needs(d) => {
          if d == name {
            return Err(format!(
              "a task cannot depend on itself. task '{}' depends on itself",
              d
            ));
          }
          task.dependencies.push(d.into())
        }
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

  fn detect_cycles(&self) -> AResult<()> {
    let mut states: HashMap<String, VisitState> = HashMap::new();
    let mut path = Vec::new();

    for name in self.tasks.keys() {
      if !states.contains_key(name) {
        self.visit_task(name, &mut states, &mut path)?;
      }
    }

    Ok(())
  }

  fn visit_task(
    &self,
    name: &str,
    states: &mut HashMap<String, VisitState>,
    path: &mut Vec<String>,
  ) -> AResult<()> {
    match states.get(name) {
      Some(VisitState::Visiting) => {
        let start = path.iter().position(|n| n == name).unwrap_or(0);

        let mut cycle = path[start..].to_vec();
        cycle.push(name.to_string());

        return Err(format!(
          "circular dependency detected: {}",
          cycle.join(" -> ")
        ));
      }

      Some(VisitState::Visited) => {
        return Ok(());
      }

      None => {}
    }

    states.insert(name.to_string(), VisitState::Visiting);
    path.push(name.to_string());

    let task = self.tasks.get(name).unwrap();

    for dependency in &task.dependencies {
      if !self.tasks.contains_key(dependency) {
        return Err(format!(
          "task '{}' depends on unknown task '{}'",
          name, dependency
        ));
      }

      self.visit_task(dependency, states, path)?;
    }

    path.pop();
    states.insert(name.to_string(), VisitState::Visited);

    Ok(())
  }
}
