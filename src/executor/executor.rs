//! # Executor
//!
//! Executes a resolved task graph (produced by the analyzer) by running
//! each task's dependencies first, then its own commands in a shell
//! appropriate to the host OS. Tasks that have already run are skipped on
//! subsequent requests, so shared dependencies only execute once.

use std::process::Command;
use std::{
  collections::{HashMap, HashSet},
  io::Error,
  process::ExitStatus,
};

use crate::{
  analyzer::task_plan::TaskPlan,
  errors::execute_error::ExecuteError,
  primitives::{
    result::EResult,
    shell::{Shell, detect_shell},
  },
};

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

  /// Executes the named task, recursively executing its dependencies
  /// first (depth-first, in declaration order). Tasks that have already
  /// been executed are skipped. Returns
  /// [`ExecuteError::UnknownTask`] if `task_name` isn't in the task map,
  /// or [`ExecuteError::ProcessDidntExitSuccessfully`] if the task's
  /// last command exits with a non-zero code.
  pub fn execute_task(&mut self, task_name: &str) -> EResult<()> {
    if self.executed.contains(task_name) {
      return Ok(());
    }

    let task = self
      .tasks
      .get(task_name)
      .ok_or_else(|| ExecuteError::UnknownTask {
        name: task_name.into(),
      })?
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
      return Err(ExecuteError::ProcessDidntExitSuccessfully { exit_code });
    }

    Ok(())
  }

  /// Runs a single shell command to completion, choosing the underlying
  /// shell (`sh`, PowerShell, or `cmd`) via [`detect_shell`]. Returns
  /// the process's [`ExitStatus`], or [`ExecuteError::ProcessError`] if
  /// the process couldn't be spawned.
  fn run_command(&mut self, command: &str) -> EResult<ExitStatus> {
    let shell = detect_shell();
    let status: Result<ExitStatus, Error>;

    match shell {
      Shell::Sh => {
        status = Command::new("sh").args(["-c", command]).status();
      }

      Shell::PowerShell => {
        status = Command::new("powershell")
          .args(["-NoProfile", "-Command", command])
          .status();
      }

      Shell::Cmd => {
        status = Command::new("cmd").args(["/C", command]).status();
      }
    };

    status.map_err(|e| ExecuteError::ProcessError {
      message: e.to_string(),
    })
  }
}
