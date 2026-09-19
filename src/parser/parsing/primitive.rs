use crate::{
  errors::parse_error::ParseError,
  lexer::tokens::{Token, TokenKind},
  parser::parser::Parser,
  primitives::result::PResult,
};

impl Parser {
  pub(crate) fn is_empty(&self) -> bool {
    self.peek().is_none()
  }

  pub(crate) fn peek(&self) -> Option<&Token> {
    self
      .tokens
      .get(self.current)
      .filter(|t| t.kind != TokenKind::Eof)
  }

  pub(crate) fn advance(&mut self) -> Option<&Token> {
    if self.is_empty() {
      return None;
    }
    let tok = &self.tokens[self.current];
    self.current += 1;
    Some(tok)
  }

  pub(crate) fn consume(&mut self, token_kind: TokenKind, message: &str) -> PResult<()> {
    match self.peek() {
      Some(tok) if tok.kind == token_kind => {
        self.current += 1;
        Ok(())
      }
      Some(tok) => Err(ParseError::Expected {
        message: message.into(),
        span: tok.span.clone(),
      }),
      None => Err(ParseError::UnexpectedEof),
    }
  }

  pub(crate) fn compare(&self, token_kind: TokenKind) -> bool {
    matches!(self.peek(), Some(tok) if tok.kind == token_kind)
  }
}
