use std::ops::Range;

#[derive(Debug)]
pub enum Node {
  Program(Vec<Node>),
  Assignment(Box<Node>, Box<Node>),
  Expression(Box<Node>, Operator, Box<Node>),
  Term(Box<Node>, Operator, Box<Node>),
  Fact(Box<Node>),
  Identifier(IdentifierNode),
  Literal(LiteralNode),
  UnaryOperator(Operator, Box<Node>),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Operator {
  Plus,
  Minus,
  Multiply,
}

#[derive(Debug, Clone)]
pub struct IdentifierNode {
  pub literal: String,
  pub range: Range<usize>,
  pub line: usize,
}

#[derive(Debug, Clone)]
pub struct LiteralNode {
  pub number: isize,
  pub range: Range<usize>,
  pub line: usize,
}
