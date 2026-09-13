use std::process::Command;

#[derive(Debug)]
pub enum Shell {
  Sh,
  PowerShell,
  Cmd,
}

pub fn detect_shell() -> Shell {
  #[cfg(not(target_os = "windows"))]
  {
    Shell::Sh
  }

  #[cfg(target_os = "windows")]
  {
    if command_exists("sh") {
      Shell::Sh
    } else if command_exists("pwsh") {
      Shell::PowerShell
    } else if command_exists("powershell") {
      Shell::PowerShell
    } else {
      Shell::Cmd
    }
  }
}

#[cfg(target_os = "windows")]
fn command_exists(command: &str) -> bool {
  Command::new(command).arg("--version").output().is_ok()
}
