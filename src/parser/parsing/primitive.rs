use crate::{
  errors::parse_error::ParseError,
  lexer::tokens::{FStringPart, Token, TokenKind},
  parser::parser::Parser,
  primitives::{range::Range, result::PResult},
};

impl Parser {
  /// Returns `true` if there are no more meaningful tokens left to
  /// consume, i.e. [`Parser::peek`] returns `None`.
  pub(crate) fn is_empty(&self) -> bool {
    self.peek().is_none()
  }

  /// Returns a reference to the current token without consuming it,
  /// or `None` if the current position is past the end of the token
  /// stream or sitting on [`TokenKind::Eof`].
  pub(crate) fn peek(&self) -> Option<&Token> {
    self
      .tokens
      .get(self.current)
      .filter(|t| t.kind != TokenKind::Eof)
  }

  /// Consumes and returns the current token, advancing the parser's
  /// position by one. Returns `None` if there is no current token to
  /// consume (see [`Parser::is_empty`]).
  pub(crate) fn advance(&mut self) -> Option<&Token> {
    if self.is_empty() {
      return None;
    }
    let tok = &self.tokens[self.current];
    self.current += 1;
    Some(tok)
  }

  /// Consumes the current token if its kind matches `token_kind`,
  /// advancing the parser. Returns a [`ParseError::Expected`] with the
  /// given `message` if the current token doesn't match, or
  /// [`ParseError::UnexpectedEof`] if there is no current token.
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

  /// Returns `true` if the current token's kind matches `token_kind`,
  /// without consuming it.
  pub(crate) fn compare(&self, token_kind: TokenKind) -> bool {
    matches!(self.peek(), Some(tok) if tok.kind == token_kind)
  }

  /// Consumes the current token if it is a [`TokenKind::Ident`],
  /// returning its inner string value. Returns a [`ParseError::Expected`]
  /// with the given `message` (noting if the found token was a keyword)
  /// if the current token isn't an identifier, or
  /// [`ParseError::UnexpectedEof`] if there is no current token.
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

  /// Consumes the current token if it is a [`TokenKind::FString`],
  /// returning its parsed [`FStringPart`]s. Returns a
  /// [`ParseError::Expected`] with the given `message` (noting if the
  /// found token was a keyword) if the current token isn't a string, or
  /// [`ParseError::UnexpectedEof`] if there is no current token.
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

  /// Runs the given parsing function `f` and captures the [`Range`] of
  /// source positions it consumed, from the position of the current
  /// token before `f` runs to the position of the current token after.
  /// Returns `f`'s result paired with that range.
  pub(crate) fn with_range<T>(
    &mut self,
    f: impl FnOnce(&mut Self) -> PResult<T>,
  ) -> PResult<(T, Range)> {
    let start = self.tokens[self.current].span.pos;
    let value = f(self)?;
    // TODO: we are considering the 'end' to be the position of the next unconsumed token.
    // instead we need to take the end position of the last consumed token.
    let end = self.tokens[self.current].span.pos;

    Ok((value, Range { start, end }))
  }
}
