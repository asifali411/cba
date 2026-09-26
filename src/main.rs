use std::{fs, process::ExitCode};

use cba;
use colored::Colorize;

const DEFAULT_PATH: &str = "./cba";
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn print_help() {
  println!(
    "{} - a lightweight build automation tool that lets you define build tasks, dependencies, variables, compiler flags, and shell commands in a simple configuration file.

{}
    cba [OPTIONS] [TOOL_ARGS...] [-- COMMAND_ARGS...]

{}
    {}, {}              Print help information
    {}, {}           Print version information
    {}, {} <PATH>       Path to the .cba file (default: {})

Everything before '--' that isn't consumed by an option above is
forwarded as tool args. Everything after '--' is forwarded as
command args.

{}
    cba --help
    cba test
    cba test -- -Wall -Wextra
    cba -p \"./test/proj/.cba\" build test -- -Wall -Wextra
",
    "cba".bold().cyan(),
    "USAGE:".bold().yellow(),
    "OPTIONS:".bold().yellow(),
    "-h".green(),
    "--help".green(),
    "-v".green(),
    "--version".green(),
    "-p".green(),
    "--path".green(),
    DEFAULT_PATH.italic(),
    "EXAMPLES:".bold().yellow(),
  );
}

fn print_version() {
  println!("{} {}", "cba".bold().cyan(), VERSION.green());
}

fn split_on_double_dash(args: Vec<String>) -> (Vec<String>, Vec<String>) {
  match args.iter().position(|arg| arg == "--") {
    Some(pos) => {
      let before = args[..pos].to_vec();
      let after = args[pos + 1..].to_vec();
      (before, after)
    }
    None => (args, Vec::new()),
  }
}

enum ParseOutcome {
  Continue {
    path: String,
    tool_args: Vec<String>,
  },
  Exit(ExitCode),
}

fn parse_pre_dash_args(args: Vec<String>) -> ParseOutcome {
  let mut path: Option<String> = None;
  let mut tool_args: Vec<String> = Vec::new();

  let mut iter = args.into_iter();
  while let Some(arg) = iter.next() {
    match arg.as_str() {
      "-h" | "--help" => {
        print_help();
        return ParseOutcome::Exit(ExitCode::SUCCESS);
      }
      "-v" | "--version" => {
        print_version();
        return ParseOutcome::Exit(ExitCode::SUCCESS);
      }
      "-p" | "--path" => match iter.next() {
        Some(value) => path = Some(value),
        None => {
          eprintln!(
            "{} '{}' expects a value (path to file)",
            "error:".red().bold(),
            arg.yellow()
          );
          return ParseOutcome::Exit(ExitCode::FAILURE);
        }
      },
      other if other.starts_with('-') => {
        eprintln!(
          "{} Unknown flag '{}'",
          "error:".red().bold(),
          other.yellow()
        );
        return ParseOutcome::Exit(ExitCode::FAILURE);
      }
      other => tool_args.push(other.to_string()),
    }
  }

  ParseOutcome::Continue {
    path: path.unwrap_or_else(|| DEFAULT_PATH.to_string()),
    tool_args,
  }
}

fn main() -> ExitCode {
  let raw_args: Vec<String> = std::env::args().skip(1).collect();
  let (pre_dash_args, command_args) = split_on_double_dash(raw_args);

  let (path, tool_args) = match parse_pre_dash_args(pre_dash_args) {
    ParseOutcome::Continue { path, tool_args } => (path, tool_args),
    ParseOutcome::Exit(code) => return code,
  };

  match fs::read_to_string(&path) {
    Ok(source) => cba::run(source, tool_args, command_args),
    Err(err) => {
      eprintln!("{} {}", "error:".red().bold(), err);
      ExitCode::FAILURE
    }
  }
}
