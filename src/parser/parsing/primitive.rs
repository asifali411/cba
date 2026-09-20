use crate::{
  errors::parse_error::ParseError,
  lexer::tokens::{FStringPart, Token, TokenKind},
  parser::parser::Parser,
  primitives::{range::Range, result::PResult},
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

  pub(crate) fn expect_ident(&mut self, message: &str) -> PResult<String> {
    let tok = self.peek().ok_or(ParseError::UnexpectedEof)?;

    match &tok.kind {
      TokenKind::Ident(value) => {
        let value = value.clone();
        self.advance();
        Ok(value)
      }
      _ => Err(ParseError::Expected {
        message: format!(
          "{}, but found '{}'{}",
          message,
          tok.to_string(),
          if tok.is_keyword() { " keyword" } else { "" }
        ),
        span: tok.span.clone(),
      }),
    }
  }

  pub(crate) fn expect_string(&mut self, message: &str) -> PResult<Vec<FStringPart>> {
    let tok = self.peek().ok_or(ParseError::UnexpectedEof)?;

    match &tok.kind {
      TokenKind::FString(value) => {
        let value = value.clone();
        self.advance();
        Ok(value)
      }
      _ => Err(ParseError::Expected {
        message: format!(
          "{}, but found '{}'{}",
          message,
          tok.to_string(),
          if tok.is_keyword() { " keyword" } else { "" }
        ),
        span: tok.span.clone(),
      }),
    }
  }

  pub(crate) fn with_range<T>(
    &mut self,
    f: impl FnOnce(&mut Self) -> PResult<T>,
  ) -> PResult<(T, Range)> {
    let start = self.tokens[self.current].span.pos;
    let value = f(self)?;
    let end = self.tokens[self.current].span.pos;

    Ok((value, Range { start, end }))
  }
}
