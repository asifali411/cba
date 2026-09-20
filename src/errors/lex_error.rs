use crate::primitives::span::Span;
use colored::Colorize;

pub enum LexError {
  UndefinedCharacter { char: char, span: Span },
  ExpectedCharacter { message: String, span: Span },
  InvalidEscapeCharacter { char: char, span: Span },
  UnterminatedString { qoute: char, span: Span },
  ExpectedIdentifier { found: String, span: Span },
}

impl LexError {
  fn location(&self) -> (usize, usize) {
    match self {
      Self::UndefinedCharacter { span, .. }
      | Self::ExpectedCharacter { span, .. }
      | Self::InvalidEscapeCharacter { span, .. }
      | Self::UnterminatedString { span, .. }
      | Self::ExpectedIdentifier { span, .. } => (span.line, span.col - 1),
    }
  }

  fn detail(&self) -> String {
    match self {
      Self::UndefinedCharacter { char, .. } => {
        format!("Undefined character '{}'", char.to_string().yellow())
      }
      Self::ExpectedCharacter { message, .. } => message.into(),
      Self::InvalidEscapeCharacter { char, .. } => {
        format!("Invalid escape character '\\{}'", char.to_string().yellow())
      }
      Self::UnterminatedString { qoute, .. } => format!(
        "Unterminated string expression, '{}' was never closed",
        qoute.to_string().yellow()
      ),
      Self::ExpectedIdentifier { found, .. } => {
        format!("Expected Identifier but found '{}'", found.yellow())
      }
    }
  }

  pub fn display(&self) {
    let (line, col) = self.location();
    let prefix = "error".red().bold();
    let detail = self.detail();
    let loc = format!(" at line: {line}, col: {col}");

    eprintln!("{prefix}: {detail}\n{loc}");
  }
}
