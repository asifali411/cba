use colored::Colorize;
use std::{fs, path::PathBuf, process::ExitCode};

use crate::{
  analyzer::analyzer::Analyzer, errors::lang_error::LangError, executor::executor::Executor,
  lexer::lexer::Lexer, parser::parser::Parser,
};

mod analyzer;
mod errors;
mod executor;
mod lexer;
mod parser;
mod primitives;

pub fn run(root: String, tool_args: Vec<String>, command_args: Vec<String>) -> ExitCode {
  let mut path = PathBuf::from(&root);

  if path.file_name().map_or(true, |name| name != "cba.txt") {
    path.push("cba.txt");
  }

  match path.try_exists() {
    Ok(true) => {}
    Ok(false) => {
      eprintln!(
        "{} '{}' File not found in the directory '{}'",
        "error:".red().bold(),
        "cba.txt".yellow(),
        root.yellow()
      );
      return ExitCode::FAILURE;
    }
    Err(err) => {
      eprintln!(
        "{} Could not check '{}': {}",
        "error:".red().bold(),
        path.display(),
        err
      );
      return ExitCode::FAILURE;
    }
  }

  let source = match fs::read_to_string(&path) {
    Ok(source) => source,
    Err(err) => {
      eprintln!("{} {}", "error:".red().bold(), err);
      return ExitCode::FAILURE;
    }
  };

  if let Err(err) = try_run(&source, tool_args, command_args) {
    err.display(&source);
    ExitCode::FAILURE
  } else {
    ExitCode::SUCCESS
  }
}

fn try_run(
  source: &String,
  tool_args: Vec<String>,
  command_args: Vec<String>,
) -> Result<(), LangError> {
  let mut lexer = Lexer::new(source);
  let tokens = lexer.tokenize()?;

  let mut parser = Parser::new(tokens);
  let stmts = parser.parse()?;

  let mut analyzer = Analyzer::new(&stmts);
  let tasks = analyzer.analyze(command_args)?;

  let mut executor = Executor::new(tasks);

  for task in tool_args {
    executor.execute_task(&task)?;
  }

  Ok(())
}
