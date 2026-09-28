//! # Parser
//!
//! Converts a flat stream of [`Token`]s (produced by the lexer) into an AST
//! of [`Stmt`]s. The grammar recognized is a flat sequence of top-level
//! declarations — `var` bindings and `task` blocks — where each task body
//! is itself a sequence of `needs` and `run` statements.

use crate::{
  errors::parse_error::ParseError,
  lexer::tokens::{Token, TokenKind},
  parser::{
    expr::{Expr, ExprKind},
    stmt::{Stmt, StmtKind, TaskStmt},
  },
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

  /// Parses a single top-level declaration — a `var` binding (see
  /// [`Parser::var_declaration`]), a `task` block (see
  /// [`Parser::task_declaration`]), or, for any other leading token, a
  /// bare statement (see [`Parser::statement`]) — and wraps the
  /// resulting [`StmtKind`] together with its source [`Range`] into a
  /// [`Stmt`]. Returns [`ParseError::UnexpectedEof`] if the stream is
  /// empty; other errors are propagated from the sub-parser that was
  /// dispatched to.
  fn declaration(&mut self) -> PResult<Stmt> {
    let (kind, range) = self.with_range(|p| match p.peek() {
      Some(tok) => match tok.kind {
        TokenKind::Var => p.var_declaration(),
        TokenKind::Task => p.task_declaration(),
        _ => p.statement(),
      },
      None => Err(ParseError::UnexpectedEof),
    })?;

    Ok(Stmt { kind, range })
  }

  /// Parses a `var` declaration of the form `var <ident> = <expr>;`,
  /// assuming the leading `var` keyword is the current token. The
  /// right-hand side is parsed with [`Parser::expression`].
  fn var_declaration(&mut self) -> PResult<StmtKind> {
    self.advance();

    let name = self.expect_ident("Expected a variable name")?;
    self.consume(TokenKind::Equal, "Expected '=' after the variable name")?;

    let expr = self.expression()?;

    self.consume(
      TokenKind::SemiColon,
      "Expected ';' after the variable declaration",
    )?;
    Ok(StmtKind::Var { name, expr })
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

  /// Parses a top-level statement that isn't a `var` or `task`
  /// declaration. Currently the only such form is an expression
  /// statement (see [`Parser::expression_statement`]).
  fn statement(&mut self) -> PResult<StmtKind> {
    Ok(self.expression_statement()?)
  }

  /// Parses an expression followed by a terminating `;`, producing a
  /// [`StmtKind::Expr`]. Returns a [`ParseError`] if the expression is
  /// malformed or the semicolon is missing.
  fn expression_statement(&mut self) -> PResult<StmtKind> {
    let expr = self.expression()?;
    self.consume(TokenKind::SemiColon, "Expect ';' after an expression")?;
    Ok(StmtKind::Expr(expr))
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

  /// Parses a `needs` statement of the form `needs <ident> (& <ident>)*;`,
  /// assuming the leading `needs` keyword is the current token. Produces
  /// a [`TaskStmt::Needs`] listing every dependency task in order. An
  /// empty list (`needs;`) is accepted and yields no dependencies.
  fn need_statement(&mut self) -> PResult<TaskStmt> {
    self.advance();

    let mut dependencies: Vec<String> = Vec::new();

    if !self.compare(TokenKind::SemiColon) {
      dependencies.push(self.expect_ident("Expected task name after 'needs'")?);

      while self.compare(TokenKind::And) {
        self.advance();

        dependencies.push(self.expect_ident("Expected task name after '&'")?);
      }
    }

    self.consume(
      TokenKind::SemiColon,
      "Expected ';' after the 'needs' statement",
    )?;

    Ok(TaskStmt::Needs(dependencies))
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

  /// Parses an expression and wraps the resulting [`ExprKind`] together
  /// with its source [`Range`] into an [`Expr`]. Currently every
  /// expression is a single primary (see [`Parser::primary`]); there are
  /// no operators or precedence levels yet.
  fn expression(&mut self) -> PResult<Expr> {
    let (kind, range) = self.with_range(|p| p.primary())?;

    Ok(Expr { kind, range })
  }

  /// Parses a primary expression by consuming one token. Recognized forms:
  ///
  /// - an f-string literal, producing [`ExprKind::FStringExpr`];
  /// - `match <string>`, producing [`ExprKind::MatchExpr`] with the
  ///   string as its pattern.
  ///
  /// Returns [`ParseError::UnexpectedEof`] if the stream is empty, or an
  /// "Expected a string or match keyword" error for any other token.
  fn primary(&mut self) -> PResult<ExprKind> {
    let tok = self.advance().ok_or(ParseError::UnexpectedEof)?.clone();

    match &tok.kind {
      TokenKind::FString(fstring) => Ok(ExprKind::FStringExpr(fstring.to_vec())),
      TokenKind::Match => {
        let pattern = self.expect_string("Expect string as pattern")?;
        Ok(ExprKind::MatchExpr(pattern))
      }
      _ => Err(ParseError::Expected {
        message: format!(
          "Expected a string or match keyword, but found '{}'{}",
          tok.to_string(),
          if tok.is_keyword() { " keyword" } else { "" }
        ),
        span: tok.span.clone(),
      }),
    }
  }
}
