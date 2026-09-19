use colored::Colorize;

use crate::primitives::span::Span;

pub enum ParseError {
  UnexpectedEof,
  Expected { message: String, span: Span },
}

impl ParseError {
  fn location(&self) -> Option<(usize, usize)> {
    match self {
      ParseError::UnexpectedEof => None,
      ParseError::Expected { span, .. } => Some((span.line, span.col)),
    }
  }

  fn detail(&self) -> String {
    match self {
      Self::UnexpectedEof => "Unexpected end of file".into(),
      Self::Expected { message, .. } => message.into(),
    }
  }

  pub fn display(&self) {
    let prefix = "error".red().bold();
    let detail = self.detail();

    match self.location() {
      Some((line, col)) => {
        let loc = format!(" at line: {line}, col: {col}");
        eprintln!("{prefix}: {detail}\n{loc}");
      }
      None => {
        eprintln!("{prefix}: {detail}");
      }
    }
  }
}
