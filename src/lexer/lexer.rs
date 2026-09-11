use crate::{
  lexer::tokens::{Token, TokenKind},
  primitives::result::LResult,
};

pub struct Lexer {
  pub(crate) source: Vec<char>,
  pub(crate) start: usize,
  pub(crate) current: usize,
  pub(crate) line: usize,
  pub(crate) col: usize,
  pub(crate) tokens: Vec<Token>,
}

impl Lexer {
  pub fn new(source: String) -> Self {
    Self {
      source: source.chars().collect(),
      start: 0,
      current: 0,
      line: 1,
      col: 1,
      tokens: Vec::new(),
    }
  }

  pub fn tokenize(&mut self) -> LResult<&Vec<Token>> {
    while !self.is_at_end() {
      self.start = self.current;
      self.scan_token()?;
    }
    self.add_token(TokenKind::Eof);
    Ok(&self.tokens)
  }

  fn scan_token(&mut self) -> LResult<()> {
    let c = self.advance();

    match c {
      ' ' | '\t' | '\r' => {}
      '\n' => {
        self.line += 1;
        self.col = 1;
      }

      '#' => self.skip_comment(),

      _ => {}
    }

    Ok(())
  }

  fn skip_comment(&mut self) {
    while self.peek() != '\n' && !self.is_at_end() {
      self.advance();
    }
  }
}
