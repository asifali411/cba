use crate::{
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
    match self
      .peek()
      .cloned()
      .ok_or(String::from("unexpected end of file"))?
      .kind
    {
      TokenKind::Var => self.var_declaration(),
      TokenKind::Task => self.task_declaration(),
      _ => self.statement(),
    }
  }

  fn var_declaration(&mut self) -> PResult<Stmt> {
    self.advance();

    let name = match self.peek() {
      Some(tok) => match &tok.kind {
        TokenKind::Ident(v) => v.clone(),
        _ => {
          return Err(format!(
            "expected variable name\nat line: {}, col: {}",
            tok.span.line, tok.span.col
          ));
        }
      },
      None => return Err(String::from("unexpected end of file")),
    };

    self.advance();
    self.consume(TokenKind::Equal, "expect '=' after variable name")?;

    let value = match self.peek() {
      Some(tok) => match &tok.kind {
        TokenKind::FString(s) => s.clone(),
        _ => return Err(String::from("expect string as variable value")),
      },
      None => return Err(String::from("expect variable value after '='")),
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

    let name = match self.peek() {
      Some(tok) => match &tok.kind {
        TokenKind::Ident(t) => t.clone(),
        _ => {
          return Err(format!(
            "expected task name\nat line: {}, col: {}",
            tok.span.line, tok.span.col
          ));
        }
      },
      None => return Err(String::from("unexpect end of file")),
    };

    self.advance();

    let body = self.task_statement()?;
    Ok(Stmt::Task { name, body })
  }

  fn statement(&mut self) -> PResult<Stmt> {
    match self
      .peek()
      .cloned()
      .ok_or(String::from("unexpected end of file"))?
      .kind
    {
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
            println!("{:?}", tok);
            return Err(format!(
              "expect statement\nat line: {}, col: {}",
              tok.span.line, tok.span.col
            ));
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
    let task = match self.peek() {
      Some(tok) => match &tok.kind {
        TokenKind::Ident(t) => t.clone(),
        _ => {
          return Err(format!(
            "expect a identifier\nat line: {}, col: {}",
            tok.span.line, tok.span.col
          ));
        }
      },
      None => return Err(String::from("unexpected end of file")),
    };
    self.advance();

    self.consume(
      TokenKind::SemiColon,
      "expect semicolon after 'needs' statement",
    )?;

    Ok(TaskStmt::Needs(task))
  }

  fn run_statement(&mut self) -> PResult<TaskStmt> {
    self.advance();
    let command = match self.peek() {
      Some(tok) => match &tok.kind {
        TokenKind::FString(s) => s.clone(),
        _ => {
          return Err(format!(
            "expected a string\nat line: {}, col: {}",
            tok.span.line, tok.span.col
          ));
        }
      },
      None => return Err(String::from("unexpected end of file")),
    };
    self.advance();

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
