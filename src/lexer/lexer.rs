//! # Lexer
//!
//! Converts raw source code into a flat stream of [`Token`]s, which are then
//! consumed by the parser to build an AST.
//!
//! The lexer works by repeatedly scanning characters from the source buffer,
//! grouping them into lexemes (e.g. identifiers, keywords, punctuation), and
//! emitting a corresponding [`Token`] with its [`Span`] (line/column) for
//! error reporting.
//!
//! Scanning is fallible: unrecognized characters produce a [`LexError`].

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
      current: Span {
        line: 1,
        col: 1,
        pos: 0,
        len: 0,
      },
      start: Span {
        line: 1,
        col: 1,
        pos: 0,
        len: 0,
      },
      tokens: Vec::new(),
    }
  }

  /// Scans the entire source and returns the resulting list of tokens,
  /// terminated with an [`TokenKind::Eof`] token.
  pub fn tokenize(&mut self) -> LResult<&Vec<Token>> {
    while !self.is_at_end() {
      self.start = self.current.clone();
      self.scan_token()?;
    }
    self.add_token(TokenKind::Eof);
    Ok(&self.tokens)
  }

  /// Scans a single token starting at `self.start`, consuming as many
  /// characters as needed and pushing the resulting token (if any) via
  /// [`Lexer::add_token`]. Whitespace and newlines are consumed but do not
  /// produce tokens; newlines update line/column tracking instead.
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
      '&' => self.add_token(TokenKind::And),

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

  /// Skips comments by repeatedly skipping the next characters
  /// until it encounters a newline character.
  fn skip_comment(&mut self) {
    while self.peek() != '\n' && !self.is_at_end() {
      self.advance();
    }
  }

  /// Consumes characters while they form a valid identifier (alphanumeric),
  /// then emits either a keyword token or a [`TokenKind::Ident`]
  /// token depending on whether the lexeme matches a reserved keyword.
  fn scan_identifier(&mut self) {
    while self.peek().is_ascii_alphanumeric() {
      self.advance();
    }

    let lexeme: String = self.source[self.start.pos..self.current.pos]
      .iter()
      .collect();
    let kind = Self::keyword(&lexeme).unwrap_or(TokenKind::Ident(lexeme));
    self.add_token(kind);
  }

  /// Scans a quoted f-string literal starting after the opening `quote`
  /// character, producing a [`TokenKind::FString`] made up of interleaved
  /// [`FStringPart::Text`] and [`FStringPart::Ident`] parts.
  ///
  /// Handles backslash escapes (`\n`, `\t`, `\r`, `\"`, `\'`, `\\`, `\{`,
  /// `\}`) and `{ident}` interpolations, where `ident` must start with an
  /// alphabetic character or underscore and continue with alphanumeric
  /// characters or underscores. Returns a [`LexError`] if an escape is
  /// invalid, an interpolation is malformed (missing identifier or closing
  /// `}`), or the string is left unterminated before the closing quote.
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

  /// Maps a raw identifier string to a reserved keyword's [`TokenKind`],
  /// or `None` if it isn't a keyword (i.e. it's a plain identifier).
  fn keyword(s: &str) -> Option<TokenKind> {
    match s {
      "var" => Some(TokenKind::Var),
      "run" => Some(TokenKind::Run),
      "task" => Some(TokenKind::Task),
      "needs" => Some(TokenKind::Needs),
      "match" => Some(TokenKind::Match),
      _ => None,
    }
  }
}
