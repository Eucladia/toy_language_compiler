use crate::token::{Token, TokenKind};

pub struct Lexer<'a> {
  src: &'a [u8],
  curr: usize,
  is_eof: bool,
  line_number: usize,
}

impl<'a> Lexer<'a> {
  /// Creates a new Lexer from a [str].
  pub fn new(src: &'a str) -> Self {
    Self::from_bytes(src.as_bytes())
  }

  /// Creates a new Lexer from a slice of bytes.
  pub fn from_bytes(src: &'a [u8]) -> Self {
    Self {
      src,
      curr: 0,
      is_eof: false,
      line_number: 1,
    }
  }

  /// Lexes the input source into a [`Vec<Token>`].
  ///
  /// Note: This **does not** preserve whitespace tokens! If whitespace is necessary, use [Lexer::lex_with_whitespace].
  pub fn lex(&mut self) -> Vec<Token> {
    let mut tokens = Vec::new();

    while let Some(token) = self.lex_token() {
      if !matches!(token.kind(), TokenKind::Whitespace) {
        tokens.push(token);
      }
    }

    tokens
  }

  /// Lexes the input source into a [`Vec<Token>`].
  ///
  /// This function preserves whitespace.
  #[allow(dead_code)]
  pub fn lex_with_whitespace(&mut self) -> Vec<Token> {
    let mut tokens = Vec::new();

    while let Some(token) = self.lex_token() {
      tokens.push(token);
    }

    tokens
  }

  /// Lexes a single token.
  pub fn lex_token(&mut self) -> Option<Token> {
    use TokenKind::*;

    if self.is_eof {
      return None;
    }

    // Add the EOF token if we're at the end of the input source
    if self.curr >= self.src.len() {
      self.is_eof = true;

      return Some(Token::new(
        EndOfFile,
        self.curr..self.curr,
        self.line_number,
      ));
    }

    // We bounds check above, so unwrapping directly is fine
    let byte = self.current_byte().unwrap();
    // Unwrapping is also fine here because the lookup table has all possible 256 values (size of u8)
    let token_type = BYTE_TOKEN_LOOKUP.get(byte as usize).copied().unwrap();
    let starting_index = self.curr;
    let line_number = self.line_number;

    let token_kind = match token_type {
      // Single character tokens
      ByteTokenType::EQUAL => self.advance_and_return(Equal),
      ByteTokenType::L_PAREN => self.advance_and_return(LeftParen),
      ByteTokenType::R_PAREN => self.advance_and_return(RightParen),
      ByteTokenType::STAR => self.advance_and_return(Star),
      ByteTokenType::PLUS => self.advance_and_return(Plus),
      ByteTokenType::MINUS => self.advance_and_return(Minus),
      ByteTokenType::SEMICOLON => self.advance_and_return(Semicolon),
      ByteTokenType::LINEBREAK => {
        self.line_number += 1;
        self.advance_and_return(Whitespace)
      }
      ByteTokenType::WHITESPACE => self.advance_and_return(Whitespace),
      ByteTokenType::INVALID => self.advance_and_return(Unknown),

      // Multi-character tokens
      ByteTokenType::NUMBER => self.consume_and_return(|b| b.is_ascii_digit(), Literal),
      ByteTokenType::LETTER => {
        self.consume_and_return(|b| b.is_ascii_alphanumeric() || b == b'_', Identifier)
      }
    };

    Some(Token::new(
      token_kind,
      starting_index..self.curr,
      line_number,
    ))
  }

  // Consumes while the provided function is true and return the specified `TokenKind`
  fn consume_and_return<F>(&mut self, func: F, ret_token: TokenKind) -> TokenKind
  where
    F: Fn(u8) -> bool,
  {
    while self.next_byte().map_or(false, &func) {}

    ret_token
  }

  #[inline]
  fn advance_and_return(&mut self, ret_token: TokenKind) -> TokenKind {
    self.advance();

    ret_token
  }

  // Advances the cursor and returns that underlying byte
  #[inline]
  fn next_byte(&mut self) -> Option<u8> {
    self.curr += 1;
    self.src.get(self.curr).copied()
  }

  // Returns the current byte
  #[inline]
  fn current_byte(&self) -> Option<u8> {
    self.src.get(self.curr).copied()
  }

  // Advances the source index.
  #[inline]
  fn advance(&mut self) {
    if self.curr < self.src.len() {
      self.curr += 1;
    }
  }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
enum ByteTokenType {
  NUMBER,
  LETTER,
  SEMICOLON,
  EQUAL,
  L_PAREN,
  R_PAREN,
  STAR,
  PLUS,
  MINUS,
  LINEBREAK,
  WHITESPACE,
  INVALID,
}

// Array where the index corresponds to the byte received by the Lexer.
//
// The value is the type of token for that byte.
const BYTE_TOKEN_LOOKUP: [ByteTokenType; 256] = {
  let mut default = [ByteTokenType::INVALID; 256];

  // Whitespace characters, taken from `u8::is_ascii_whitespace`
  default[b'\t' as usize] = ByteTokenType::WHITESPACE;
  default[b'\n' as usize] = ByteTokenType::LINEBREAK;
  default[b'\x0C' as usize] = ByteTokenType::WHITESPACE;
  default[b'\r' as usize] = ByteTokenType::LINEBREAK;
  default[b' ' as usize] = ByteTokenType::WHITESPACE;
  // Semicolon
  default[b';' as usize] = ByteTokenType::SEMICOLON;
  // Arithmetic
  default[b'*' as usize] = ByteTokenType::STAR;
  default[b'-' as usize] = ByteTokenType::MINUS;
  default[b'+' as usize] = ByteTokenType::PLUS;
  // Assignment
  default[b'=' as usize] = ByteTokenType::EQUAL;
  // Parenthesis
  default[b'(' as usize] = ByteTokenType::L_PAREN;
  default[b')' as usize] = ByteTokenType::R_PAREN;

  // Numbers
  let mut i = b'0';

  while i <= b'9' {
    default[i as usize] = ByteTokenType::NUMBER;
    i += 1;
  }

  // Alphabet
  i = b'a';

  while i <= b'z' {
    default[i as usize] = ByteTokenType::LETTER;
    i += 1;
  }

  i = b'A';

  while i <= b'Z' {
    default[i as usize] = ByteTokenType::LETTER;
    i += 1;
  }

  default
};

#[cfg(test)]
mod tests {
  use super::*;

  macro_rules! are_tokens_equal {
  ($src:literal, $($token:tt),*) => {
    let tokens = get_tokens!(include_str!(concat!("../sample_input/", $src, ".txt")));
    let expected = vec![$(TokenKind::$token),*];

    assert_eq!(tokens, expected);
  }
}

  macro_rules! get_tokens {
    ($src:expr) => {{
      let mut lexer = Lexer::new($src);
      let mut tokens = lexer
        .lex()
        .into_iter()
        .map(|tok| tok.kind())
        .collect::<Vec<_>>();

      // Remove the `EOF` token
      tokens.pop();

      tokens
    }};
  }

  #[test]
  fn invalid_tokens() {
    let tokens = get_tokens!("____`````><>.,.`,.`");

    assert_eq!(tokens, vec![TokenKind::Unknown]);
  }

  #[test]
  fn one() {
    #[rustfmt::skip]
    are_tokens_equal!(
      "1",
       Identifier, Equal, Literal, Semicolon
    );
  }

  #[test]
  fn two() {
    #[rustfmt::skip]
    are_tokens_equal!(
      "2",
       Identifier, Equal, Literal, Semicolon
    );
  }

  #[test]
  fn three() {
    #[rustfmt::skip]
    are_tokens_equal!(
      "3",
       Identifier, Equal, Literal,
       Identifier, Equal, Identifier, Semicolon,
       Identifier, Equal, Minus, Minus, Minus, LeftParen, Identifier, Plus, Identifier, RightParen, Semicolon
    );
  }

  #[test]
  fn four() {
    #[rustfmt::skip]
    are_tokens_equal!(
      "4",
      Identifier, Equal, Literal, Semicolon,
      Identifier, Equal, Literal, Semicolon,
      Identifier, Equal, Minus, Minus, Minus, LeftParen, Identifier, Plus, Identifier, RightParen, Star, LeftParen, Identifier, Plus, Minus, Identifier, RightParen, Semicolon
    );
  }
}
