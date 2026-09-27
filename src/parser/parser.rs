//! # Parser
//!
//! Converts a flat stream of [`Token`]s (produced by the lexer) into an AST
//! of [`Stmt`]s. The grammar recognized is a flat sequence of top-level
//! declarations — `var` bindings and `task` blocks — where each task body
//! is itself a sequence of `needs` and `run` statements.

use crate::{
  errors::parse_error::ParseError,
  lexer::tokens::{Token, TokenKind},
  parser::stmt::{Stmt, StmtKind, TaskStmt},
  primitives::result::PResult,
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

  /// Parses the entire token stream into a list of top-level
  /// [`Stmt`]s, repeatedly parsing declarations until the stream is
  /// exhausted.
  pub fn parse(&mut self) -> PResult<Vec<Stmt>> {
    let mut statements: Vec<Stmt> = Vec::new();

    while !self.is_empty() {
      statements.push(self.declaration()?);
    }

    Ok(statements)
  }

  /// Parses a single top-level declaration — either a `var` binding
  /// (see [`Parser::var_declaration`]) or a `task` block (see
  /// [`Parser::task_declaration`]) — and wraps the resulting
  /// [`StmtKind`] together with its source [`Range`] into a [`Stmt`].
  /// Returns a [`ParseError::Expected`] if the current token is neither
  /// `var` nor `task`, or [`ParseError::UnexpectedEof`] if the stream
  /// is empty.
  fn declaration(&mut self) -> PResult<Stmt> {
    let (kind, range) = self.with_range(|p| match p.peek() {
      Some(tok) => match tok.kind {
        TokenKind::Var => p.var_declaration(),
        TokenKind::Task => p.task_declaration(),
        _ => Err(ParseError::Expected {
          message: format!("Expected 'var' or 'task', but found '{}'", tok.to_string(),),
          span: tok.span.clone(),
        }),
      },
      None => Err(ParseError::UnexpectedEof),
    })?;

    Ok(Stmt { kind, range })
  }

  /// Parses a `var` declaration of the form `var <ident> = <string>;`,
  /// assuming the leading `var` keyword is the current token.
  fn var_declaration(&mut self) -> PResult<StmtKind> {
    self.advance();

    let name = self.expect_ident("Expected a variable name")?;
    self.consume(TokenKind::Equal, "Expected '=' after the variable name")?;

    let value = self.expect_string("Expected a string as the variable value")?;

    self.consume(
      TokenKind::SemiColon,
      "Expected ';' after the variable declaration",
    )?;
    Ok(StmtKind::Var { name, value })
  }

  /// Parses a `task` declaration of the form `task <name> { ... }`,
  /// assuming the leading `task` keyword is the current token. The
  /// task name may be either an identifier or the `run` keyword used
  /// as a name (matching the special-cased "run" task).
  fn task_declaration(&mut self) -> PResult<StmtKind> {
    self.advance();
    let name = match self.peek() {
      Some(tok) => match tok.kind {
        TokenKind::Run => {
          self.advance();
          "run".to_string()
        }
        _ => self.expect_ident("Expected a task name")?,
      },
      None => return Err(ParseError::UnexpectedEof),
    };

    let body = self.task_statement()?;
    Ok(StmtKind::Task { name, body })
  }

  /// Parses a brace-delimited task body: `{ (needs|run statement)* }`.
  /// Each statement inside must begin with `needs` or `run`; any other
  /// token produces a [`ParseError::Expected`].
  fn task_statement(&mut self) -> PResult<Vec<TaskStmt>> {
    self.consume(TokenKind::LeftBrace, "Expect '{' before the task body")?;

    let mut statements: Vec<TaskStmt> = Vec::new();
    while !self.is_empty() && !self.compare(TokenKind::RightBrace) {
      let tok = self.peek().ok_or(ParseError::UnexpectedEof)?;
      match &tok.kind {
        TokenKind::Needs => statements.push(self.need_statement()?),
        TokenKind::Run => statements.push(self.run_statement()?),
        _ => {
          return Err(ParseError::Expected {
            message: format!(
              "Expected 'needs' or 'run', but found '{}'{}",
              tok.to_string(),
              if tok.is_keyword() { " keyword" } else { "" }
            ),
            span: tok.span.clone(),
          });
        }
      }
    }

    self.consume(TokenKind::RightBrace, "Expect '}' after the task body")?;
    Ok(statements)
  }

  /// Parses a `needs <ident>;` statement, assuming the leading `needs`
  /// keyword is the current token, producing a [`TaskStmt::Needs`]
  /// naming the dependency task.
  fn need_statement(&mut self) -> PResult<TaskStmt> {
    self.advance();
    let task = self.expect_ident("Expected task name after 'needs'")?;

    self.consume(
      TokenKind::SemiColon,
      "Expected ';' after the 'needs' statement",
    )?;

    Ok(TaskStmt::Needs(task))
  }

  /// Parses a `run <string>;` statement, assuming the leading `run`
  /// keyword is the current token, producing a [`TaskStmt::Run`]
  /// containing the command string.
  fn run_statement(&mut self) -> PResult<TaskStmt> {
    self.advance();
    let command = self.expect_string("Expected a string after 'run'")?;

    self.consume(
      TokenKind::SemiColon,
      "Expected ';' after the 'run' statement",
    )?;
    Ok(TaskStmt::Run(command))
  }
}
