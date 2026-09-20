use std::process::ExitCode;

use crate::{
  analyzer::analyzer::Analyzer, executor::executor::Executor, lexer::lexer::Lexer,
  parser::parser::Parser,
};

mod analyzer;
mod errors;
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

fn split_args() -> (Vec<String>, Vec<String>) {
  let args: Vec<String> = std::env::args().skip(1).collect();

  match args.iter().position(|arg| arg == "--") {
    Some(pos) => {
      let first = args[..pos].to_vec();
      let second = args[pos + 1..].to_vec();
      (first, second)
    }
    None => (args, Vec::new()),
  }
}

fn try_run(source: String) -> Result<(), Box<dyn std::error::Error>> {
  let (tool_args, command_args) = split_args();

  let mut lexer = Lexer::new(&source);
  let tokens = match lexer.tokenize() {
    Ok(toks) => toks,
    Err(e) => {
      e.display();
      return Err("".into());
    }
  };

  let mut parser = Parser::new(tokens);
  let stmts = match parser.parse() {
    Ok(stmts) => stmts,
    Err(e) => {
      e.display();
      return Err("".into());
    }
  };

  let mut analyzer = Analyzer::new(&stmts);
  let tasks = match analyzer.analyze(command_args) {
    Ok(tasks) => tasks,
    Err(e) => {
      e.display(&source);
      return Err("".into());
    }
  };

  let mut executor = Executor::new(tasks);

  for task in tool_args {
    executor.execute_task(&task)?;
  }

  Ok(())
}
