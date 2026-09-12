#[derive(Debug, Clone, PartialEq)]
pub struct TaskPlan {
  name: String,
  dependencies: Vec<String>,
  commands: Vec<String>,
}
