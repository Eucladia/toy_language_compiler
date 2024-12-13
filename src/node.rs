use std::ops::Range;

/// The nodes of this language.
#[derive(Debug)]
pub enum Node {
  /// Vec of `Assignment` nodes.
  Program(Vec<Node>),
  /// An `Identifier` node and an `Expression` node.
  Assignment(Box<Node>, Box<Node>),
  /// A node containing a `Term` node.
  Expression(Box<Node>),
  /// A node applying an operation to two other nodes.
  Term(Box<Node>, Operator, Box<Node>),
  /// A node that may contain another node that has a `+` or `-` preceding it.
  Fact(Box<Node>),
  /// A node that either has `+` or `-` before another node.
  UnaryOperator(Operator, Box<Node>),
  /// A node containing an `Identifier` node.
  Identifier(IdentifierNode),
  /// A node containing a `Literal` node.
  Literal(LiteralNode),
}

/// The operators of this language.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Operator {
  Plus,
  Minus,
  Multiply,
}

/// An identifier node.
#[derive(Debug, Clone)]
pub struct IdentifierNode {
  /// The source string of this node.
  pub literal: String,
  // Store the range and line to make error diagnostics easier
  /// The range of this node in the source file.
  pub range: Range<usize>,
  /// The line of this node in the souce file.
  pub line: usize,
}

// A literal node.
#[derive(Debug, Clone)]
pub struct LiteralNode {
  /// The number for this node.
  pub value: isize,
}
