use std::process::ExitCode;

use crate::{lexer::lexer::Lexer, parser::parser::Parser};

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
