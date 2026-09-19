use crate::errors::{lex_error::LexError, parse_error::ParseError};

pub type LResult<T> = Result<T, LexError>;
pub type PResult<T> = Result<T, ParseError>;
pub type AResult<T> = Result<T, String>;
pub type EResult<T> = Result<T, String>;
