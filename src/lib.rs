use std::process::ExitCode;

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

pub fn run(source: String, tool_args: Vec<String>, command_args: Vec<String>) -> ExitCode {
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
