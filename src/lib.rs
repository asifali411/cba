use std::process::ExitCode;

use crate::{lexer::lexer::Lexer, parser::parser::Parser};

mod lexer;
mod parser;
mod primitives;

pub fn run(source: String) -> ExitCode {
  let mut lexer = Lexer::new(source);

  match lexer.tokenize() {
    Ok(tokens) => {
      println!("{:?}", tokens);

      let mut parser = Parser::new(tokens);
    }
    Err(err) => {
      eprintln!("{}", err);
      return ExitCode::FAILURE;
    }
  };

  ExitCode::SUCCESS
}
