use crate::primitives::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
  Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
  pub kind: TokenKind,
  pub span: Span,
}
