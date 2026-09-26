use crate::errors::{
  analyze_error::AnalyzeError, execute_error::ExecuteError, lex_error::LexError,
  parse_error::ParseError,
};

pub type LResult<T> = Result<T, LexError>;
pub type PResult<T> = Result<T, ParseError>;
pub type AResult<T> = Result<T, AnalyzeError>;
pub type EResult<T> = Result<T, ExecuteError>;
