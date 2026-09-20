use crate::lexer::{
  lexer::Lexer,
  tokens::{Token, TokenKind},
};

impl Lexer {
  pub(crate) fn is_at_end(&self) -> bool {
    self.current.pos >= self.source.len()
  }

  pub(crate) fn advance(&mut self) -> char {
    let c = self.source[self.current.pos];
    self.current.col += 1;
    self.current.pos += 1;
    c
  }

  pub(crate) fn peek(&self) -> char {
    if self.is_at_end() {
      '\0'
    } else {
      self.source[self.current.pos]
    }
  }

  pub(crate) fn add_token(&mut self, kind: TokenKind) {
    self.tokens.push(Token {
      kind,
      span: self.start.clone(),
    });
  }
}
