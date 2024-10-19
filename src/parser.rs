use crate::{
  error::DiagnosticError,
  lexer::Lexer,
  node::{IdentifierNode, LiteralNode, Node, Operator},
  token::{Token, TokenKind},
  token_matches,
};

#[derive(Debug)]
pub struct Parser<'a> {
  src: &'a str,
  tokens: Vec<Token>,
  token_pos: usize,
}

#[derive(Debug, Clone)]
pub struct TokenInfo<'a> {
  pub line: usize,
  pub column: usize,
  pub literal: &'a str,
}

impl<'a> Parser<'a> {
  /// Creates a new [Parser] from the source string.
  pub fn new(src: &'a str) -> Self {
    Self::from_tokens(src, Lexer::new(src).lex())
  }

  /// Creates a new [Parser] from the vec of [Token]s.
  pub fn from_tokens(src: &'a str, tokens: Vec<Token>) -> Self {
    Self {
      src,
      tokens,
      token_pos: 0,
    }
  }

  /// Parses the vector into a [Node], with the root being [Node::Program]
  pub fn parse(&mut self) -> Result<Node, Vec<DiagnosticError>> {
    let mut errors = Vec::new();

    let program = self.parse_program(&mut errors);

    if errors.is_empty() {
      Ok(program)
    } else {
      Err(errors)
    }
  }

  fn parse_program(&mut self, errors: &mut Vec<DiagnosticError>) -> Node {
    let mut assignments = Vec::new();

    while let Some(tok) = self.next_token() {
      if matches!(tok.kind(), TokenKind::EndOfFile) {
        break;
      }

      match self.parse_assignment(&tok) {
        Ok(res) => assignments.push(res),
        Err(err) => errors.push(err),
      }
    }

    Node::Program(assignments)
  }

  fn parse_assignment(&mut self, curr_token: &Token) -> Result<Node, DiagnosticError> {
    // Handle the identifier first
    let ident = token_matches!(
      Some(curr_token),
      Identifier,
      Self::token_info(self.src, curr_token)
    )
    .map(|tok| {
      Node::Identifier(IdentifierNode {
        literal: self.src.get(tok.range()).unwrap().to_string(),
        range: curr_token.range(),
        line: curr_token.line(),
      })
    })?;

    // We expect an equal sign
    token_matches!(
      self.next_token(),
      Equal,
      Self::token_info(self.src, self.previous_token().unwrap())
    )?;

    // Parse the expression
    let expr = self.parse_expr()?;

    // We expect a semicolon
    match token_matches!(
      self.current_token(),
      Semicolon,
      Self::token_info(self.src, self.previous_token().unwrap())
    ) {
      // Advance the cursor since we saw a semicolon
      Ok(_) => self.advance(),
      Err(err) => return Err(err),
    }

    Ok(Node::Assignment(Box::new(ident), Box::new(expr)))
  }

  fn parse_expr(&mut self) -> Result<Node, DiagnosticError> {
    fn parse_expr_inner(parser: &mut Parser, lhs_term: Node) -> Result<Node, DiagnosticError> {
      match parser.current_token().map(Token::kind) {
        kind if matches!(kind, Some(TokenKind::Plus | TokenKind::Minus)) => {
          // Advance since we saw `+`` or `-`
          parser.advance();

          let rhs_term = parser.parse_term()?;

          // Recurse on the expression as needed
          parse_expr_inner(
            parser,
            Node::Term(
              Box::new(lhs_term),
              if matches!(kind, Some(TokenKind::Plus)) {
                Operator::Plus
              } else {
                Operator::Minus
              },
              Box::new(rhs_term),
            ),
          )
        }
        // If we got any other character besides `+` or `-`, then we're done recursing the expr
        _ => Ok(lhs_term),
      }
    }

    let lhs_term = self.parse_term()?;

    parse_expr_inner(self, lhs_term)
  }

  fn parse_term(&mut self) -> Result<Node, DiagnosticError> {
    fn parse_term_inner(parser: &mut Parser, lhs_fact: Node) -> Result<Node, DiagnosticError> {
      match parser.current_token().map(Token::kind) {
        Some(TokenKind::Star) => {
          // Advance token position since we saw `*`
          parser.advance();

          let rhs_fact = parser.parse_fact()?;

          // Recurse on the term
          parse_term_inner(
            parser,
            Node::Term(Box::new(lhs_fact), Operator::Multiply, Box::new(rhs_fact)),
          )
        }
        // If we got any other token besides `*`, then we got parsed the entire term
        _ => Ok(lhs_fact),
      }
    }

    let lhs_fact = self.parse_fact()?;

    parse_term_inner(self, lhs_fact)
  }

  fn parse_fact(&mut self) -> Result<Node, DiagnosticError> {
    let fact_token = self.next_token();
    let token = token_matches!(
      &fact_token,
      Literal | Identifier | LeftParen | Minus | Plus,
      Self::token_info(self.src, fact_token.as_ref().unwrap())
    )?;

    match token.kind() {
      // Numeric literals
      TokenKind::Literal => {
        let num_str = self.src.get(token.range()).unwrap();
        if num_str.starts_with('0') && num_str.len() > 1 {
          // Invalid integer errors are recoverable, so advance the index.
          self.advance();

          return Err(DiagnosticError::new(
            format!(
              "the integer, `{}`, is invalid. literals must be either 0 or non-zero digits.",
              num_str
            ),
            token.line(),
            token.range().start + 1 - Self::token_linebreak_index(self.src, token),
          ));
        }

        Ok(Node::Literal(LiteralNode {
          number: num_str.parse().map_err(|_| {
            let token_info = Self::token_info(self.src, token);

            DiagnosticError::new(
              format!("invalid integer, `{}`.", num_str),
              token_info.line,
              token_info.column,
            )
          })?,
          range: token.range(),
          line: token.line(),
        }))
      }
      // Identifiers (variables for this language)
      TokenKind::Identifier => Ok(Node::Identifier(IdentifierNode {
        literal: self.src.get(token.range()).unwrap().to_string(),
        line: token.line(),
        range: token.range(),
      })),
      // Left parenthesis
      TokenKind::LeftParen => {
        let expr = self.parse_expr()?;

        token_matches!(
          self.next_token(),
          RightParen,
          Self::token_info(self.src, self.previous_token().unwrap())
        )?;

        Ok(Node::Fact(Box::new(expr)))
      }
      // Unary operations
      TokenKind::Minus => {
        let fact = self.parse_fact()?;

        Ok(Node::Fact(Box::new(Node::UnaryOperator(
          Operator::Minus,
          Box::new(fact),
        ))))
      }
      TokenKind::Plus => {
        let fact = self.parse_fact()?;

        Ok(Node::Fact(Box::new(Node::UnaryOperator(
          Operator::Plus,
          Box::new(fact),
        ))))
      }
      // Unexpected token
      other => {
        let token_info = Self::token_info(self.src, token);

        Err(DiagnosticError::new(
          format!(
            "unexpected `{}` ({}) found when parsing fact.",
            other, token_info.literal,
          ),
          token_info.line,
          token_info.column,
        ))
      }
    }
  }

  /// Returns the current [Token] and advances the position.
  pub fn next_token(&mut self) -> Option<Token> {
    let tok = self.tokens.get(self.token_pos).cloned();

    self.token_pos += 1;

    tok
  }

  /// Returns the current [Token]
  pub fn current_token(&self) -> Option<&Token> {
    self.tokens.get(self.token_pos)
  }

  /// Advances the internal position of the current [Token].
  pub fn advance(&mut self) {
    if self.token_pos < self.tokens.len() {
      self.token_pos += 1;
    }
  }

  /// Returns the previous [Token].
  pub fn previous_token(&self) -> Option<&Token> {
    self.tokens.get(self.token_pos - 1)
  }

  /// Returns information about this [Token].
  pub fn token_info<'b>(src: &'b str, token: &Token) -> TokenInfo<'b> {
    TokenInfo {
      column: token.range().end + 1 - Self::token_linebreak_index(src, token),
      line: token.line(),
      literal: src.get(token.range()).unwrap(),
    }
  }

  pub fn token_linebreak_index(src: &str, token: &Token) -> usize {
    src[..token.range().start]
      .rfind('\n')
      .map(|i| i + 1)
      .unwrap_or(0)
  }
}

#[macro_export]
macro_rules! token_matches {
  ($token:expr, $expected:pat, $token_info:expr) => {{
    use $crate::error::DiagnosticError;
    use $crate::token::TokenKind::*;

    let tok = match $token {
      Some(t) => Ok(t),
      None => Err(DiagnosticError::new(
        format!(
          "expected `{}`, but got end of input.",
          stringify!($expected)
        ),
        $token_info.line,
        $token_info.column,
      )),
    }?;

    match tok.kind() {
      x if matches!(x, $expected) => Ok(tok),
      other => Err(DiagnosticError::new(
        format!(
          "expected `{}` after `{}`, but got `{}`.",
          stringify!($expected),
          $token_info.literal,
          other
        ),
        $token_info.line,
        $token_info.column,
      )),
    }
  }};
}
