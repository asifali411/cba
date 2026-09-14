use crate::errors::lex_error::LexError;

pub type LResult<T> = Result<T, LexError>;
pub type PResult<T> = Result<T, String>;
pub type AResult<T> = Result<T, String>;
pub type EResult<T> = Result<T, String>;
