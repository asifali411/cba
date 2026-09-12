use std::collections::{HashMap, HashSet};

use crate::{analyzer::task_plan::TaskPlan, executor, primitives::result::EResult};

pub struct Executor {
  tasks: HashMap<String, TaskPlan>,
  executed: HashSet<String>,
}

impl Executor {
  pub fn new(tasks: &HashMap<String, TaskPlan>) -> Self {
    Self {
      tasks: tasks.to_owned(),
      executed: HashSet::new(),
    }
  }

  pub fn execute_task(&mut self, task_name: &str) -> EResult<()> {
    if self.executed.contains(task_name) {
      return Ok(());
    }

    let task = self.tasks
      .get(task_name)
      .ok_or_else(|| format!("unknown task: {task_name}"))?.clone();

    for dependency in &task.dependencies {
      self.execute_task(&dependency)?;
    }

    for command in &task.commands {
      self.run_command(command)?;
    }

    self.executed.insert(task_name.to_string());

    Ok(())
  }

  fn run_command(&self, command: &str) -> EResult<()> {
    Ok(())
  }
}
