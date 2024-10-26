use crate::token::Token;
use std::ops::Range;

/// Extra information about a [Token].
#[derive(Debug, Clone)]
pub struct TokenInfo<'a> {
  /// The line that this token is on.
  pub line: usize,
  /// The column on the line that this [Token] is on.
  pub column: usize,
  /// The source literal of this [Token].
  pub literal: &'a str,
}

/// Returns information about this [Token].
///
/// Notes:
/// This function panics if the token's range isn't in source string.
pub fn token_info<'b>(src: &'b str, token: &Token) -> TokenInfo<'b> {
  TokenInfo {
    column: token.range().end - linebreak_index(src, token.range()),
    line: token.line(),
    literal: src.get(token.range()).unwrap(),
  }
}

/// Returns the index of the last linebreak before the given start of the given [Range].
pub fn linebreak_index(src: &str, range: Range<usize>) -> usize {
  src
    .get(..range.start)
    .and_then(|s| s.rfind('\n'))
    .map_or(0, |i| i + 1)
}
