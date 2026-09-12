use crate::{
  lexer::tokens::{Token, TokenKind},
  parser::stmt::Stmt,
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
      .ok_or(String::from("unexpect end of file"))?
      .kind
    {
      TokenKind::Var => self.var_declaration(),
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

  fn statement(&mut self) -> PResult<Stmt> {
    self.expression()
  }

  fn expression(&mut self) -> PResult<Stmt> {
    self.advance();
    Ok(Stmt::Task {
      name: String::new(),
      body: vec![],
    })
  }
}
