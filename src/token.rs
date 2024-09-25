use std::ops::Range;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
  /// The kind of token it is.
  kind: TokenKind,
  /// The span of the token.
  range: Range<usize>,
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
  /// The literal character `/`.
  Slash,
  /// The literal character `-`
  Minus,
  /// The literal character `+`
  Plus,
  /// The literal character `;`
  Semicolon,
  /// End of the input source.
  EndOfFile,
  /// A whitespace token.
  ///
  /// This is any one of these characters, `\n` & `\r`, `\t`, ` `, `\xOC`.
  Whitespace,
  /// Unrecognized tokens.
  Unknown,
}

impl Token {
  /// Creates a new [Token]
  pub fn new(kind: TokenKind, range: Range<usize>) -> Self {
    Token { kind, range }
  }

  /// Returns the [TokenKind] of this token.
  pub fn kind(&self) -> TokenKind {
    self.kind
  }

  /// Returns the range of this token.
  ///
  /// The lower bound is inclusive, the upper bound is exclusive.
  #[allow(dead_code)]
  pub fn range(&self) -> &Range<usize> {
    &self.range
  }
}
