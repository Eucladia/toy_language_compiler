use std::num::IntErrorKind;

use crate::{
  error::DiagnosticError,
  lexer::Lexer,
  node::{IdentifierNode, LiteralNode, Node, Operator},
  token::{Token, TokenKind},
  util::{linebreak_index, token_info},
};

#[derive(Debug)]
pub struct Parser<'a> {
  src: &'a str,
  lexer: LexerManager,
}

#[derive(Debug)]
struct LexerManager {
  tokens: Vec<Token>,
  token_pos: usize,
}

impl<'a> Parser<'a> {
  /// Creates a new [Parser] from the source string.
  #[allow(dead_code)]
  pub fn new(src: &'a str) -> Self {
    Self::from_tokens(src, Lexer::new(src).lex())
  }

  /// Creates a new [Parser] from the vec of [Token]s.
  pub fn from_tokens(src: &'a str, tokens: Vec<Token>) -> Self {
    Self {
      src,
      lexer: LexerManager {
        tokens,
        token_pos: 0,
      },
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

    self.parse_assignment(&mut assignments, errors);

    // The last token should be an EndOfFile one
    assert_eq!(
      self.lexer.current_token().map(Token::kind),
      Some(TokenKind::EndOfFile)
    );

    Node::Program(assignments)
  }

  fn parse_assignment(&mut self, assignments: &mut Vec<Node>, errors: &mut Vec<DiagnosticError>) {
    let ident_token = self.lexer.current_token().cloned();

    // No more assignments to parse.
    if ident_token.is_none()
      || matches!(
        ident_token.as_ref().map(Token::kind),
        Some(TokenKind::EndOfFile)
      )
    {
      return;
    }

    let ident_token = ident_token.unwrap();
    let ident_token_info = token_info(self.src, &ident_token);

    let identifier_node = if matches!(ident_token.kind(), TokenKind::Identifier) {
      // Only advance if we see a valid identifier, for better error diagonstics
      self.lexer.advance();

      Some(Node::Identifier(IdentifierNode {
        literal: ident_token_info.literal.into(),
        range: ident_token.range(),
        line: ident_token.line(),
      }))
    } else {
      errors.push(DiagnosticError::new(
        format!(
          "Expected an `Identifier`, but found `{}` ({})",
          &ident_token_info.literal,
          ident_token.kind()
        ),
        ident_token_info.line,
        ident_token_info.column,
      ));

      None
    };

    // Parse the equal sign
    match self.lexer.current_token() {
      Some(tok) if matches!(tok.kind(), TokenKind::Equal) => {
        self.lexer.advance();
      }
      Some(next_token) if !matches!(next_token.kind(), TokenKind::EndOfFile) => {
        let next_info = token_info(self.src, next_token);

        errors.push(DiagnosticError::new(
          format!(
            "Expected an `Equal` token, but found `{}` ({}).",
            next_info.literal,
            next_token.kind()
          ),
          ident_token_info.line,
          // If the identifier token and next token are on the same line, then
          // point to the start of the next token
          if next_token.line() == ident_token.line() {
            next_token.range().start + 1 - linebreak_index(self.src, ident_token.range())
          } else {
            ident_token.range().end + 1 - linebreak_index(self.src, ident_token.range())
          },
        ));
      }
      // Either no token or we got an `EOF`
      _ => {
        errors.push(DiagnosticError::new(
          "Expected an `Equal` token.".to_string(),
          ident_token_info.line,
          ident_token.range().end + 1 - linebreak_index(self.src, ident_token.range()),
        ));
      }
    }

    // Parse the expression
    let expr_node = match self.parse_expr() {
      Ok(node) => Some(node),
      Err(e) => {
        errors.push(e);

        // Try to recover from the lack of expression, except for cases where the
        // current token is `EndOfFile` or `Semicolon`
        if !matches!(
          self.lexer.current_token().map(Token::kind),
          Some(TokenKind::EndOfFile | TokenKind::Semicolon)
        ) {
          self.lexer.token_pos -= 1;
        }

        None
      }
    };

    let expr_token = self.lexer.previous_token().cloned().unwrap();
    let expr_token_info = token_info(self.src, &expr_token);

    // We expect a semicolon
    match self.lexer.current_token().cloned() {
      Some(tok) if matches!(tok.kind(), TokenKind::Semicolon) => {
        self.lexer.advance();
      }
      Some(tok) => {
        errors.push(DiagnosticError::new(
          format!(
            "Expected a `Semicolon` after `{}`, but found `{}` ({}).",
            expr_token_info.literal,
            self.src.get(tok.range()).unwrap(),
            tok.kind()
          ),
          expr_token_info.line,
          // The column should be after the expression
          expr_token.range().end + 1 - linebreak_index(self.src, expr_token.range()),
        ));
      }
      None => {
        errors.push(DiagnosticError::new(
          format!(
            "Expected `{}` after `{}`.",
            TokenKind::Semicolon,
            expr_token_info.literal,
          ),
          expr_token_info.line,
          // The column should be after the expression
          expr_token.range().end + 1 - linebreak_index(self.src, expr_token.range()),
        ));

        return;
      }
    }

    if let (Some(ident), Some(expr)) = (identifier_node, expr_node) {
      assignments.push(Node::Assignment(Box::new(ident), Box::new(expr)));
    }

    self.parse_assignment(assignments, errors);
  }

  fn parse_expr(&mut self) -> Result<Node, DiagnosticError> {
    fn parse_expr_inner(parser: &mut Parser, lhs_term: Node) -> Result<Node, DiagnosticError> {
      match parser.lexer.current_token().map(Token::kind) {
        kind if matches!(kind, Some(TokenKind::Plus | TokenKind::Minus)) => {
          // Advance since we saw `+`` or `-`
          parser.lexer.advance();

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

    Ok(Node::Expression(Box::new(parse_expr_inner(
      self, lhs_term,
    )?)))
  }

  fn parse_term(&mut self) -> Result<Node, DiagnosticError> {
    fn parse_term_inner(parser: &mut Parser, lhs_fact: Node) -> Result<Node, DiagnosticError> {
      match parser.lexer.current_token().map(Token::kind) {
        Some(TokenKind::Star) => {
          // Advance token position since we saw `*`
          parser.lexer.advance();

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
    let fact_token = self.lexer.current_token().cloned();

    match fact_token {
      Some(x)
        if !matches!(
          x.kind(),
          TokenKind::Literal
            | TokenKind::Identifier
            | TokenKind::LeftParen
            | TokenKind::Minus
            | TokenKind::Plus
        ) =>
      {
        let eof = matches!(x.kind(), TokenKind::EndOfFile);

        // Only advance if we're not at the end
        if !eof {
          self.lexer.advance();
        }

        let token_info = token_info(self.src, &x);

        Err(DiagnosticError::new(
          format!(
            "Expected either `+`, `-`, `(`, an `Identifier`, or a `Literal`, but found `{}` ({})",
            &token_info.literal,
            x.kind()
          ),
          token_info.line,
          // If we're at the end, then the fact is expected at the next column
          if eof {
            token_info.column + 1
          } else {
            token_info.column
          },
        ))
      }

      Some(x) if matches!(x.kind(), TokenKind::Literal) => {
        self.lexer.advance();

        let token_info = token_info(self.src, &x);
        let num_str = token_info.literal;

        if num_str.starts_with('0') && num_str.len() > 1 {
          return Err(DiagnosticError::new(
            format!(
              "The integer, `{}`, is invalid. literals must be either 0 or non-zero digits.",
              num_str
            ),
            x.line(),
            // Point to the start of the invalid integer
            x.range().start + 1 - linebreak_index(self.src, x.range()),
          ));
        }

        match num_str.parse() {
          Ok(num) => Ok(Node::Literal(LiteralNode { value: num })),
          Err(e) => {
            match e.kind() {
              IntErrorKind::NegOverflow | IntErrorKind::PosOverflow => Err(DiagnosticError::new(
                format!(
                  "The integer,`{}`, is invalid. integers must be in the range [{}, {}].",
                  num_str,
                  isize::MIN,
                  isize::MAX
                ),
                x.line(),
                // Point to the start of the invalid integer
                x.range().start + 1 - linebreak_index(self.src, x.range()),
              )),
              // Any other cases shouldn't be reachable
              _ => unreachable!("invalid integer"),
            }
          }
        }
      }

      Some(x) if matches!(x.kind(), TokenKind::Identifier) => {
        self.lexer.advance();

        Ok(Node::Identifier(IdentifierNode {
          literal: self.src.get(x.range()).unwrap().to_string(),
          line: x.line(),
          range: x.range(),
        }))
      }

      Some(x) if matches!(x.kind(), TokenKind::LeftParen) => {
        self.lexer.advance();

        let expr = self.parse_expr()?;

        match self.lexer.current_token().cloned() {
          Some(x) if matches!(x.kind(), TokenKind::RightParen) => {
            self.lexer.advance();
          }
          Some(x) => {
            self.lexer.advance();

            let expr_token = self.lexer.tokens.get(self.lexer.token_pos - 1).unwrap();
            let expr_token_info = token_info(self.src, expr_token);
            let curr_token_info = token_info(self.src, &x);

            return Err(DiagnosticError::new(
              format!(
                "Expected a `)` after `{}`, but found `{}`",
                expr_token_info.literal, curr_token_info.literal
              ),
              curr_token_info.line,
              curr_token_info.column,
            ));
          }
          None => {
            let expr_token = self.lexer.tokens.get(self.lexer.token_pos - 1).unwrap();
            let expr_token_info = token_info(self.src, expr_token);

            return Err(DiagnosticError::new(
              format!("Expected a `)` after `{}`.", expr_token_info.literal),
              x.line(),
              expr_token.range().end - linebreak_index(self.src, expr_token.range()),
            ));
          }
        }

        Ok(Node::Fact(Box::new(expr)))
      }

      // Unary operations
      Some(x) if matches!(x.kind(), TokenKind::Minus) => {
        self.lexer.advance();

        let fact = self.parse_fact()?;

        Ok(Node::Fact(Box::new(Node::UnaryOperator(
          Operator::Minus,
          Box::new(fact),
        ))))
      }
      Some(x) if matches!(x.kind(), TokenKind::Plus) => {
        self.lexer.advance();

        let fact = self.parse_fact()?;

        Ok(Node::Fact(Box::new(Node::UnaryOperator(
          Operator::Plus,
          Box::new(fact),
        ))))
      }

      Some(other) => {
        self.lexer.advance();

        let token_info = token_info(self.src, &other);

        Err(DiagnosticError::new(
          format!(
            "Unexpected `{}` ({}) found when parsing fact.",
            other.kind(),
            token_info.literal,
          ),
          token_info.line,
          token_info.column,
        ))
      }

      None => {
        let sec_last = self.lexer.tokens.get(self.lexer.token_pos - 2).unwrap();
        let sec_last_info = token_info(self.src, sec_last);

        Err(DiagnosticError::new(
          format!(
            "Expected either `+`, `-`, `(`, an `Identifier`, or a `Literal` after `{}`",
            &sec_last_info.literal
          ),
          sec_last.line(),
          sec_last_info.column + 1,
        ))
      }
    }
  }
}

impl LexerManager {
  /// Returns the current [Token]
  pub fn current_token(&self) -> Option<&Token> {
    self.tokens.get(self.token_pos)
  }

  /// Returns the previous [Token].
  pub fn previous_token(&self) -> Option<&Token> {
    self.tokens.get(self.token_pos - 1)
  }

  /// Advances the internal position of the current [Token].
  pub fn advance(&mut self) {
    if self.token_pos < self.tokens.len() {
      self.token_pos += 1;
    }
  }
}
