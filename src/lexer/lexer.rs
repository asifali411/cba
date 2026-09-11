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

      ';' => self.add_token(TokenKind::SemiColon),
      '=' => self.add_token(TokenKind::Equal),

      '{' => self.add_token(TokenKind::LeftBrace),
      '}' => self.add_token(TokenKind::RightBrace),

      '#' => self.skip_comment(),

      '\'' => self.scan_text('\'')?,
      '"' => self.scan_text('"')?,

      c if c.is_ascii_alphabetic() => self.scan_identifier(),

      _ => {}
    }

    Ok(())
  }

  fn skip_comment(&mut self) {
    while self.peek() != '\n' && !self.is_at_end() {
      self.advance();
    }
  }

  fn scan_identifier(&mut self) {
    while self.peek().is_ascii_alphanumeric() {
      self.advance();
    }

    let lexeme: String = self.source[self.start..self.current].iter().collect();
    let kind = Self::keyword(&lexeme).unwrap_or(TokenKind::Ident(lexeme));
    self.add_token(kind);
  }

  fn scan_text(&mut self, quote: char) -> LResult<()> {
    let mut value = String::new();

    while !self.is_at_end() {
      match self.peek() {
        '\\' => {
          self.advance();

          let escaped = match self.peek() {
            'n' => '\n',
            't' => '\t',
            'r' => '\r',
            '"' => '"',
            '\'' => '\'',
            '\\' => '\\',
            other => {
              return Err(format!("Invalid escape character '{}'", other));
            }
          };

          value.push(escaped);
          self.advance();
        }

        c if c == quote => break,

        c => {
          value.push(c);
          self.advance();
        }
      }
    }

    if self.is_at_end() {
      return Err(format!("unterminated string, '{quote}' was never closed"));
    }

    self.advance();
    self.add_token(TokenKind::Text(value));
    Ok(())
  }

  fn keyword(s: &str) -> Option<TokenKind> {
    match s {
      "var" => Some(TokenKind::Var),
      "run" => Some(TokenKind::Run),
      "task" => Some(TokenKind::Task),
      "needs" => Some(TokenKind::Needs),
      _ => None,
    }
  }
}
