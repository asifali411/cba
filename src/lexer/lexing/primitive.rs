use crate::{
  lexer::{
    lexer::Lexer,
    tokens::{Token, TokenKind},
  },
  primitives::span::Span,
};

impl Lexer {
  fn is_at_end(&self) -> bool {
    self.current >= self.source.len()
  }

  fn advance(&mut self) -> char {
    let c = self.source[self.current];
    self.current += 1;
    self.col += 1;
    c
  }

  fn match_next(&mut self, expected: char) -> bool {
    if self.is_at_end() || self.source[self.current] != expected {
      return false;
    }
    self.current += 1;
    self.col += 1;
    true
  }

  fn peek(&self) -> char {
    if self.is_at_end() {
      '\0'
    } else {
      self.source[self.current]
    }
  }

  fn peek_next(&self) -> char {
    if self.current + 1 >= self.source.len() {
      '\0'
    } else {
      self.source[self.current + 1]
    }
  }

  fn add_token(&mut self, kind: TokenKind) {
    self.tokens.push(Token {
      kind,
      span: Span {
        line: self.line,
        col: self.col,
      },
    });
  }
}
