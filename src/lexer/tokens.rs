use std::fmt;

use crate::primitives::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum FStringPart {
  Text(String),
  Ident(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
  Equal,
  LeftBrace,
  RightBrace,
  SemiColon,

  Var,
  Task,
  Run,
  Needs,
  Ident(String),

  FString(Vec<FStringPart>),
  Eof,
}

#[derive(Clone, PartialEq)]
pub struct Token {
  pub kind: TokenKind,
  pub span: Span,
}

impl Token {
  pub fn to_string(&self) -> String {
    let string = match &self.kind {
      TokenKind::Equal => "=",
      TokenKind::LeftBrace => "{",
      TokenKind::RightBrace => "}",
      TokenKind::SemiColon => ";",
      TokenKind::Var => "var",
      TokenKind::Task => "task",
      TokenKind::Run => "run",
      TokenKind::Needs => "needs",
      TokenKind::Ident(n) => n,
      TokenKind::FString(_) => "string",
      TokenKind::Eof => "eof",
    };

    string.into()
  }

  pub fn is_keyword(&self) -> bool {
    match &self.kind {
      TokenKind::Var | TokenKind::Task | TokenKind::Run | TokenKind::Needs => true,
      _ => false,
    }
  }
}

impl fmt::Debug for Token {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Display::fmt(self, f)
  }
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", format!("{:?}", &self.kind))
  }
}
