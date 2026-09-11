use crate::{
  lexer::tokens::{Token},
};

pub struct Parser {
  pub tokens: Vec<Token>,
  pub current: usize,
}

impl Parser {
  pub fn new(tokens: &[Token]) -> Self {
    Self {
      tokens: tokens.to_vec(),
      current: 0,
    }
  }
}
