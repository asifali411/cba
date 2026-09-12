use std::process::ExitCode;

use crate::{analyzer::analyzer::Analyzer, executor::executor::Executor, lexer::lexer::Lexer, parser::parser::Parser};

mod analyzer;
mod executor;
mod lexer;
mod parser;
mod primitives;

pub fn run(source: String) -> ExitCode {
  let mut lexer = Lexer::new(source);

  match lexer.tokenize() {
    Ok(tokens) => {
      let mut parser = Parser::new(tokens);
      match parser.parse() {
        Ok(stmts) => {
          let mut analyzer = Analyzer::new(&stmts);
          match analyzer.analyze() {
            Ok(tasks) => {
              
              let mut executor = Executor::new(tasks);
            }
            Err(err) => {
              eprintln!("{}", err);
              return ExitCode::FAILURE;
            }
          }
        }
        Err(err) => {
          eprintln!("{}", err);
          return ExitCode::FAILURE;
        }
      }
    }
    Err(err) => {
      eprintln!("{}", err);
      return ExitCode::FAILURE;
    }
  };

  ExitCode::SUCCESS
}
