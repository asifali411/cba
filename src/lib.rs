use std::process::ExitCode;

use crate::{
  analyzer::analyzer::Analyzer, executor::executor::Executor, lexer::lexer::Lexer,
  parser::parser::Parser,
};

mod analyzer;
mod executor;
mod lexer;
mod parser;
mod primitives;

pub fn run(source: String) -> ExitCode {
  if let Err(err) = try_run(source) {
    eprintln!("{err}");
    ExitCode::FAILURE
  } else {
    ExitCode::SUCCESS
  }
}

fn try_run(source: String) -> Result<(), Box<dyn std::error::Error>> {
  let mut lexer = Lexer::new(source);
  let tokens = lexer.tokenize()?;

  let mut parser = Parser::new(tokens);
  let stmts = parser.parse()?;

  let mut analyzer = Analyzer::new(&stmts);
  let tasks = analyzer.analyze()?;

  let mut executor = Executor::new(tasks);

  Ok(())
}
