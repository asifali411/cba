use crate::lexer::{
  lexer::Lexer,
  tokens::{Token, TokenKind},
};

impl Lexer {
  /// Returns `true` once the cursor has passed the last character of the source
  pub(crate) fn is_at_end(&self) -> bool {
    self.current.pos >= self.source.len()
  }

  /// Consumes and returns the current character, advancing the cursor and
  /// column position by one.
  pub(crate) fn advance(&mut self) -> char {
    let c = self.source[self.current.pos];
    self.current.col += 1;
    self.current.pos += 1;
    c
  }

  /// Returns the next character without consuming it, or `'\0'` if at the
  /// end of the source.
  pub(crate) fn peek(&self) -> char {
    if self.is_at_end() {
      '\0'
    } else {
      self.source[self.current.pos]
    }
  }

  /// Pushes a new token of the given kind, tagged with the lexer's start
  /// [`Span`].
  pub(crate) fn add_token(&mut self, kind: TokenKind) {
    self.tokens.push(Token {
      kind,
      span: self.start.clone(),
    });
  }
}
