use crate::lexer::tokens::Token;

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

  pub fn tokenize(&mut self) -> Vec<Token> {
    self.tokens.clone()
  }
}
