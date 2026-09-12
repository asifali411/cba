#[cfg(target_os = "windows")]
use std::process::Command;
use std::{
  collections::{HashMap, HashSet},
  process::ExitStatus,
};

use crate::{analyzer::task_plan::TaskPlan, primitives::result::EResult};

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

    let task = self
      .tasks
      .get(task_name)
      .ok_or_else(|| format!("unknown task: {task_name}"))?
      .clone();

    for dependency in &task.dependencies {
      self.execute_task(&dependency)?;
    }

    let mut exit_code = 0;
    for command in &task.commands {
      let exit_status = self.run_command(command)?;
      exit_code = exit_status.code().unwrap_or(-1);
    }

    self.executed.insert(task_name.to_string());

    if exit_code != 0 {
      return Err(format!(
        "process didn't exit successfully (exit code: {})",
        exit_code
      ));
    }

    Ok(())
  }

  fn run_command(&self, command: &str) -> EResult<ExitStatus> {
    #[cfg(target_os = "windows")]
    let status = Command::new("cmd").args(["/C", command]).status();

    #[cfg(not(target_os = "windows"))]
    let status = Command::new("sh").args(["-c", command]).status();

    match status {
      Err(e) => Err(e.to_string()),
      Ok(e) => Ok(e),
    }
  }
}
