use crate::errors::{
  analyze_error::AnalyzeError, execute_error::ExecuteError, lex_error::LexError,
  parse_error::ParseError,
};

pub enum LangError {
  LexError(LexError),
  ParseError(ParseError),
  AnalyzeError(AnalyzeError),
  ExecuteError(ExecuteError),
}

impl From<LexError> for LangError {
  fn from(err: LexError) -> Self {
    LangError::LexError(err)
  }
}

impl From<ParseError> for LangError {
  fn from(err: ParseError) -> Self {
    LangError::ParseError(err)
  }
}

impl From<AnalyzeError> for LangError {
  fn from(err: AnalyzeError) -> Self {
    LangError::AnalyzeError(err)
  }
}

impl From<ExecuteError> for LangError {
  fn from(err: ExecuteError) -> Self {
    LangError::ExecuteError(err)
  }
}

impl LangError {
  pub fn display(&self, source: &str) {
    match self {
      LangError::LexError(e) => e.display(),
      LangError::ParseError(e) => e.display(),
      LangError::AnalyzeError(e) => e.display(source),
      LangError::ExecuteError(e) => e.display(),
    }
  }
}
