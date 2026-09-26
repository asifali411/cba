use colored::Colorize;

pub enum ExecuteError {
  UnknownTask { name: String },
  ProcessDidntExitSuccessfully { exit_code: i32 },
  ProcessError { message: String },
}

impl ExecuteError {
  fn detail(&self) -> String {
    match self {
      Self::UnknownTask { name } => {
        format!("Unknown task '{}', task '{}' is not defined", name, name)
      }
      Self::ProcessDidntExitSuccessfully { exit_code } => format!(
        "Process didn't exit successfully (exit code: {})",
        exit_code
      ),
      Self::ProcessError { message } => message.into(),
    }
  }

  pub fn display(&self) {
    let prefix = "error".red().bold();
    let detail = self.detail();

    eprintln!("{prefix}: {detail}");
  }
}
