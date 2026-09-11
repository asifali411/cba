use std::process::ExitCode;

use crate::lexer::lexer::Lexer;

mod lexer;
mod primitives;

pub fn run(source: String) -> ExitCode {
  let mut lexer = Lexer::new(source);

  match lexer.tokenize() {
    Ok(tokens) => {
      println!("{:?}", tokens);
    }
    Err(err) => {
      eprintln!("{}", err);
      return ExitCode::FAILURE;
    }
  };

  ExitCode::SUCCESS
}
