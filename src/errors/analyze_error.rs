use colored::Colorize;

use crate::primitives::range::Range;

pub enum AnalyzeError {
  CannotRedeclareTask {
    name: String,
    range: Range,
  },
  TaskCannotDependOnItself {
    name: String,
    range: Range,
  },
  CannotFindVariable {
    name: String,
    range: Range,
  },
  CircularDependency {
    dependency: String,
  },
  TaskDependsOnUnknownTask {
    task: String,
    dependency: String,
    range: Range,
  },
}

impl AnalyzeError {
  fn detail(&self) -> String {
    match self {
      Self::CannotRedeclareTask { name, .. } => {
        format!(
          "Cannot redeclare task, the task '{}' is declared multiple times",
          name
        )
      }
      Self::TaskCannotDependOnItself { name, .. } => {
        format!(
          "A task cannot depend on itself, the task '{}' depends on itself",
          name
        )
      }
      Self::CannotFindVariable { name, .. } => {
        format!("Cannot find variable {}", name)
      }
      Self::CircularDependency { dependency, .. } => {
        format!("Circular Dependency detected: {}", dependency)
      }
      Self::TaskDependsOnUnknownTask {
        task, dependency, ..
      } => {
        format!("Task '{}' depends on unknown task '{}'", task, dependency)
      }
    }
  }

  fn statement(&self, source: &str) -> Option<String> {
    let string = match self {
      Self::CannotRedeclareTask { range, .. }
      | Self::TaskCannotDependOnItself { range, .. }
      | Self::CannotFindVariable { range, .. }
      | Self::TaskDependsOnUnknownTask { range, .. } => &source[range.start..range.end],

      _ => return None,
    };

    Some(string.into())
  }

  pub fn display(&self, source: &str) {
    let prefix = "error".red().bold();
    let detail = self.detail();

    match self.statement(source) {
      Some(stmt) => {
        eprintln!("{prefix}: {detail}\n\n{stmt}");
      }
      None => {
        eprintln!("{prefix}: {detail}");
      }
    }
  }
}
