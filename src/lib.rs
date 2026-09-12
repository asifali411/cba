use std::process::ExitCode;

use crate::{analyzer::analyzer::Analyzer, lexer::lexer::Lexer, parser::parser::Parser};

mod analyzer;
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
          println!("{:?}", stmts);

          let mut analyzer = Analyzer::new(&stmts);
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
