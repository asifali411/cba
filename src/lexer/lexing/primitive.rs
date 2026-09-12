use crate::{
  lexer::{
    lexer::Lexer,
    tokens::{Token, TokenKind},
  },
  primitives::span::Span,
};

impl Lexer {
  pub(crate) fn is_at_end(&self) -> bool {
    self.current >= self.source.len()
  }

  pub(crate) fn advance(&mut self) -> char {
    let c = self.source[self.current];
    self.current += 1;
    self.col += 1;
    c
  }

  pub(crate) fn peek(&self) -> char {
    if self.is_at_end() {
      '\0'
    } else {
      self.source[self.current]
    }
  }

  pub(crate) fn add_token(&mut self, kind: TokenKind) {
    self.tokens.push(Token {
      kind,
      span: Span {
        line: self.line,
        col: self.col,
      },
    });
  }
}
