use std::collections::HashMap;

use crate::{
  analyzer::task_plan::TaskPlan,
  errors::analyze_error::AnalyzeError,
  lexer::tokens::FStringPart,
  parser::stmt::{Stmt, StmtKind, TaskStmt},
  primitives::{range::Range, result::AResult, visit_state::VisitState},
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
      match stmt.kind {
        StmtKind::Var { name, value } => {
          let value = &self.resolve_fstring(value.to_vec(), stmt.range)?;
          self.variables.insert(name.clone(), value.clone());
        }

        StmtKind::Task { name, body } => {
          self.resolve_task(&name, &body, &stmt.range)?;
        }
      }
    }

    self.detect_cycles()?;

    Ok(&self.tasks)
  }

  fn resolve_task(&mut self, name: &String, body: &Vec<TaskStmt>, range: &Range) -> AResult<()> {
    let mut task = TaskPlan::new(name.clone(), range.clone());

    if self.tasks.contains_key(name) {
      return Err(AnalyzeError::CannotRedeclareTask {
        name: name.into(),
        range: range.clone(),
      });
    }

    for stmt in body {
      match stmt {
        TaskStmt::Needs(d) => {
          if d == name {
            return Err(AnalyzeError::TaskCannotDependOnItself {
              name: d.into(),
              range: range.clone(),
            });
          }
          task.dependencies.push(d.into())
        }
        TaskStmt::Run(c) => task
          .commands
          .push(self.resolve_fstring(c.to_vec(), range.clone())?),
      }
    }

    self.tasks.insert(name.clone(), task);
    Ok(())
  }

  fn resolve_fstring(&mut self, fstring: Vec<FStringPart>, range: Range) -> AResult<String> {
    let mut value = String::new();

    for part in fstring {
      match part {
        FStringPart::Text(t) => value.push_str(&t),
        FStringPart::Ident(i) => match self.variables.get(&i) {
          Some(v) => value.push_str(v),
          None => return Err(AnalyzeError::CannotFindVariable { name: i, range }),
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

        return Err(AnalyzeError::CircularDependency {
          dependency: cycle.join(" -> "),
        });
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
        return Err(AnalyzeError::TaskDependsOnUnknownTask {
          task: task.name.clone(),
          dependency: dependency.into(),
          range: task.range.clone(),
        });
      }

      self.visit_task(dependency, states, path)?;
    }

    path.pop();
    states.insert(name.to_string(), VisitState::Visited);

    Ok(())
  }
}
