use std::process::ExitCode;

use crate::lexer::lexer::Lexer;

mod lexer;
mod primitives;

pub fn run(source: String) -> ExitCode {
  let lexer = Lexer::new(source);

  ExitCode::SUCCESS
}
