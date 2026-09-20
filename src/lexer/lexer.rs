use crate::{
  errors::lex_error::LexError,
  lexer::tokens::{FStringPart, Token, TokenKind},
  primitives::{result::LResult, span::Span},
};

pub struct Lexer {
  pub(crate) source: Vec<char>,
  pub(crate) current: Span,
  pub(crate) start: Span,
  pub(crate) tokens: Vec<Token>,
}

impl Lexer {
  pub fn new(source: &String) -> Self {
    Self {
      source: source.chars().collect(),
      current: Span { line: 1, col: 1, pos: 0 },
      start: Span { line: 1, col: 1, pos: 0 },
      tokens: Vec::new(),
    }
  }

  pub fn tokenize(&mut self) -> LResult<&Vec<Token>> {
    while !self.is_at_end() {
      self.start = self.current.clone();
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
        self.current.line += 1;
        self.current.col = 1;
      }

      ';' => self.add_token(TokenKind::SemiColon),
      '=' => self.add_token(TokenKind::Equal),

      '{' => self.add_token(TokenKind::LeftBrace),
      '}' => self.add_token(TokenKind::RightBrace),

      '#' => self.skip_comment(),

      '\'' => self.scan_fstring('\'')?,
      '"' => self.scan_fstring('"')?,

      c if c.is_ascii_alphabetic() => self.scan_identifier(),

      other => {
        return Err(LexError::UndefinedCharacter {
          char: other,
          span: self.current.clone(),
        });
      }
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

    let lexeme: String = self.source[self.start.pos..self.current.pos].iter().collect();
    let kind = Self::keyword(&lexeme).unwrap_or(TokenKind::Ident(lexeme));
    self.add_token(kind);
  }

  fn scan_fstring(&mut self, quote: char) -> LResult<()> {
    let mut parts = Vec::new();
    let mut text = String::new();

    while !self.is_at_end() {
      match self.peek() {
        c if c == quote => {
          break;
        }

        '\\' => {
          self.advance();

          let escaped = match self.peek() {
            'n' => '\n',
            't' => '\t',
            'r' => '\r',
            '"' => '"',
            '\'' => '\'',
            '\\' => '\\',
            '{' => '{',
            '}' => '}',
            other => {
              return Err(LexError::InvalidEscapeCharacter {
                char: other,
                span: self.current.clone(),
              });
            }
          };

          text.push(escaped);
          self.advance();
        }

        '{' => {
          if !text.is_empty() {
            parts.push(FStringPart::Text(std::mem::take(&mut text)));
          }

          self.advance();

          if self.is_at_end() {
            return Err(LexError::UnterminatedString {
              qoute: quote,
              span: self.current.clone(),
            });
          }

          let first = self.peek();

          if !first.is_ascii_alphabetic() && first != '_' {
            return Err(LexError::ExpectedIdentifier {
              found: first.to_string(),
              span: self.current.clone(),
            });
          }

          let mut ident = String::new();

          while !self.is_at_end() {
            let c = self.peek();

            if c.is_ascii_alphanumeric() || c == '_' {
              ident.push(c);
              self.advance();
            } else {
              break;
            }
          }

          if self.is_at_end() || self.peek() != '}' {
            return Err(LexError::ExpectedCharacter {
              message: format!("expected '}}' after string expression '{}'", ident),
              span: self.current.clone(),
            });
          }

          self.advance();

          parts.push(FStringPart::Ident(ident));
        }

        c => {
          text.push(c);
          self.advance();
        }
      }
    }

    if self.is_at_end() {
      return Err(LexError::UnterminatedString {
        qoute: quote,
        span: self.current.clone(),
      });
    }

    self.advance();

    if !text.is_empty() {
      parts.push(FStringPart::Text(text));
    }

    self.add_token(TokenKind::FString(parts));

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
