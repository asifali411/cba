#[derive(Debug, Clone, PartialEq)]
pub struct TaskPlan {
  pub name: String,
  pub dependencies: Vec<String>,
  pub commands: Vec<String>,
}

impl TaskPlan {
  pub fn new(name: String) -> Self {
    Self {
      name,
      dependencies: Vec::new(),
      commands: Vec::new(),
    }
  }
}
