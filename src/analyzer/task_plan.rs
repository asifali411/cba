use crate::primitives::range::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct TaskPlan {
  pub name: String,
  pub range: Range,
  pub dependencies: Vec<String>,
  pub commands: Vec<String>,
}

impl TaskPlan {
  pub fn new(name: String, range: Range) -> Self {
    Self {
      name,
      range,
      dependencies: Vec::new(),
      commands: Vec::new(),
    }
  }
}
