use crate::{
  errors::parse_error::ParseError,
  lexer::tokens::{Token, TokenKind},
  parser::stmt::{Stmt, TaskStmt},
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

  pub fn parse(&mut self) -> PResult<Vec<Stmt>> {
    let mut statements: Vec<Stmt> = Vec::new();

    while !self.is_empty() {
      statements.push(self.declaration()?);
    }

    Ok(statements)
  }

  fn declaration(&mut self) -> PResult<Stmt> {
    match self.peek().cloned().ok_or(ParseError::UnexpectedEof)?.kind {
      TokenKind::Var => self.var_declaration(),
      TokenKind::Task => self.task_declaration(),
      _ => self.statement(),
    }
  }

  fn var_declaration(&mut self) -> PResult<Stmt> {
    self.advance();
    let name = self.expect_ident("Expected variable name")?;

    self.consume(TokenKind::Equal, "expect '=' after variable name")?;

    let value = match self.peek() {
      Some(tok) => match &tok.kind {
        TokenKind::FString(s) => s.clone(),
        _ => {
          return Err(ParseError::Expected {
            message: format!(
              "Expected string as variable value, but found '{}'{}",
              tok.to_string(),
              if tok.is_keyword() { " keyword" } else { "" }
            ),
            span: tok.span.clone(),
          });
        }
      },
      None => return Err(ParseError::UnexpectedEof),
    };

    self.advance();

    self.consume(
      TokenKind::SemiColon,
      "expect ';' after variable declaration",
    )?;
    Ok(Stmt::Var { name, value })
  }

  fn task_declaration(&mut self) -> PResult<Stmt> {
    self.advance();
    let name = self.expect_ident("Expected task name")?;

    let body = self.task_statement()?;
    Ok(Stmt::Task { name, body })
  }

  fn statement(&mut self) -> PResult<Stmt> {
    match self.peek().cloned().ok_or(ParseError::UnexpectedEof)?.kind {
      _ => self.expression(),
    }
  }

  fn task_statement(&mut self) -> PResult<Vec<TaskStmt>> {
    self.consume(TokenKind::LeftBrace, "Expect '{' before block")?;

    let mut statements: Vec<TaskStmt> = Vec::new();
    while !self.is_empty() && !self.compare(TokenKind::RightBrace) {
      match self.peek() {
        Some(tok) => match tok.kind {
          TokenKind::Needs => statements.push(self.need_statement()?),
          TokenKind::Run => statements.push(self.run_statement()?),
          _ => {
            return Err(ParseError::Expected {
              message: format!(
                "Expected statement, but found '{}'{}",
                tok.to_string(),
                if tok.is_keyword() { " keyword" } else { "" }
              ),
              span: tok.span.clone(),
            });
          }
        },
        _ => unreachable!("This is unreachable"),
      };
    }

    self.consume(TokenKind::RightBrace, "Expect '}' after block")?;
    Ok(statements)
  }

  fn need_statement(&mut self) -> PResult<TaskStmt> {
    self.advance();
    let task = self.expect_ident("Expected task name")?;

    self.consume(
      TokenKind::SemiColon,
      "expect semicolon after 'needs' statement",
    )?;

    Ok(TaskStmt::Needs(task))
  }

  fn run_statement(&mut self) -> PResult<TaskStmt> {
    self.advance();
    let command = self.expect_string("Expected string")?;

    self.consume(
      TokenKind::SemiColon,
      "expect semicolon after 'run' statement",
    )?;
    Ok(TaskStmt::Run(command))
  }

  fn expression(&mut self) -> PResult<Stmt> {
    self.advance();
    Ok(Stmt::Task {
      name: String::new(),
      body: vec![],
    })
  }
}
