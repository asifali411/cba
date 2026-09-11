use crate::{lexer::tokens::Token, primitives::result::LResult};

pub struct Lexer {
  source: Vec<char>,
  current: usize,
  tokens: Vec<Token>,
}

impl Lexer {
  pub fn new(source: String) -> Self {
    Self {
      source: source.chars().collect(),
      current: 0,
      tokens: Vec::new(),
    }
  }

  pub fn tokenize(&mut self) -> LResult<&Vec<Token>> {
    Ok(&self.tokens)
  }
}
