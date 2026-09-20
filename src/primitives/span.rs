#[derive(Debug, Clone, PartialEq)]
pub struct Span {
  pub line: usize,
  pub col: usize,
  pub pos: usize,
}
