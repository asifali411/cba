use crate::{lexer::tokens::Token, primitives::result::LResult};

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
    Ok(&self.tokens)
  }
}
