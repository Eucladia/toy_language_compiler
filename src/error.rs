#[derive(Clone, Debug)]
pub struct DiagnosticError {
  msg: String,
  line: usize,
  column: usize,
}

impl DiagnosticError {
  pub const fn new(msg: String, line: usize, col: usize) -> Self {
    Self {
      msg,
      line,
      column: col,
    }
  }

  pub const fn line(&self) -> usize {
    self.line
  }

  pub const fn column(&self) -> usize {
    self.column
  }
}

impl std::fmt::Display for DiagnosticError {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(fmt, "{}", &self.msg)
  }
}
impl std::error::Error for DiagnosticError {}
