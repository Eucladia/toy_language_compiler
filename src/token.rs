use std::ops::Range;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
  /// The kind of token it is.
  kind: TokenKind,
  /// The span of the token.
  range: Range<usize>,
  /// The line of the token.
  line_number: usize,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
  /// Integer literals.
  Literal,
  /// Identifiers.
  ///
  /// Identifiers start with a letter, but can be followed with digits
  Identifier,
  /// The literal character `=`.
  Equal,
  /// The literal character `(`.
  LeftParen,
  /// The literal character `)`.
  RightParen,
  /// The literal character `*`.
  Star,
  /// The literal character `-`
  Minus,
  /// The literal character `+`
  Plus,
  /// The literal character `;`
  Semicolon,
  /// A whitespace token.
  ///
  /// This is any one of these characters, `\n` & `\r`, `\t`, ` `, `\xOC`.
  Whitespace,
  /// Unrecognized tokens.
  Unknown,
  /// End of the input source.
  EndOfFile,
}

impl Token {
  /// Creates a new [Token]
  pub fn new(kind: TokenKind, range: Range<usize>, line: usize) -> Self {
    Token {
      kind,
      range,
      line_number: line,
    }
  }

  /// Returns the [TokenKind] of this token.
  pub fn kind(&self) -> TokenKind {
    self.kind
  }

  /// Returns the range, exclusive on the upper bound, of this token.
  pub fn range(&self) -> Range<usize> {
    self.range.clone()
  }

  /// The line of the [Token]
  pub fn line(&self) -> usize {
    self.line_number
  }
}

impl std::fmt::Display for TokenKind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}
