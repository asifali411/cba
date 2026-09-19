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
    match self.peek() {
      Some(tok) => match tok.kind {
        TokenKind::Var => self.var_declaration(),
        TokenKind::Task => self.task_declaration(),
        _ => Err(ParseError::Expected {
          message: format!("Expected 'var' or 'task', but found '{}'", tok.to_string(),),
          span: tok.span.clone(),
        }),
      },
      None => Err(ParseError::UnexpectedEof),
    }
  }

  fn var_declaration(&mut self) -> PResult<Stmt> {
    self.advance();

    let name = self.expect_ident("Expected a variable name")?;
    self.consume(TokenKind::Equal, "Expected '=' after the variable name")?;

    let value = self.expect_string("Expected a string as the variable value")?;

    self.consume(
      TokenKind::SemiColon,
      "Expected ';' after the variable declaration",
    )?;
    Ok(Stmt::Var { name, value })
  }

  fn task_declaration(&mut self) -> PResult<Stmt> {
    self.advance();
    let name = self.expect_ident("Expected a task name")?;

    let body = self.task_statement()?;
    Ok(Stmt::Task { name, body })
  }

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

  fn need_statement(&mut self) -> PResult<TaskStmt> {
    self.advance();
    let task = self.expect_ident("Expected task name after 'needs'")?;

    self.consume(
      TokenKind::SemiColon,
      "Expected ';' after the 'needs' statement",
    )?;

    Ok(TaskStmt::Needs(task))
  }

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
