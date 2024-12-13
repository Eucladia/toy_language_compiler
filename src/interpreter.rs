use crate::{
  error::DiagnosticError,
  node::{Node, Operator},
  util::linebreak_index,
};
use std::collections::HashMap;

/// An interpreter for the toy language.
pub struct Interpreter<'a> {
  src: &'a str,
  root: Node,
  variables: HashMap<&'a str, isize>,
}

impl<'a> Interpreter<'a> {
  /// Creates a new interpreter from the souce string and root node.
  ///
  /// The source string is needed for better error diagnostics such as reporting
  /// uninitialized variables.
  pub fn new(src: &'a str, root: Node) -> Self {
    Self {
      src,
      root,
      variables: HashMap::new(),
    }
  }

  /// Evaluates the results, updating the set variables in memory.
  ///
  /// # Returns
  /// Returns all diagnostics errors in the case of failure.
  pub fn evaluate(&mut self) -> Result<(), Vec<DiagnosticError>> {
    let mut errors = Vec::new();

    evaluate_node(self.src, &self.root, &mut self.variables, &mut errors);

    if errors.is_empty() {
      Ok(())
    } else {
      Err(errors)
    }
  }

  /// Prints the set variables in memory
  pub fn dump(&self) {
    for (k, v) in &self.variables {
      println!("{} => {}", k, v);
    }
  }
}

fn evaluate_node<'a>(
  src: &'a str,
  node: &Node,
  variables: &mut HashMap<&'a str, isize>,
  errors: &mut Vec<DiagnosticError>,
) -> isize {
  match node {
    Node::Program(nodes) => {
      for node in nodes {
        evaluate_node(src, node, variables, errors);
      }

      // Doesn't really matter what number return in this case
      0
    }
    Node::Assignment(var_node, expr) => {
      // Identifiers are the only possible Node here
      if let Node::Identifier(ident_node) = &**var_node {
        let rhs = evaluate_node(src, expr, variables, errors);

        variables.insert(src.get(ident_node.range.clone()).unwrap(), rhs);
      }

      // Doesn't really matter what number return in this case
      0
    }
    Node::Expression(expr) => evaluate_node(src, expr, variables, errors),
    Node::Term(lhs, op, rhs) => match op {
      Operator::Plus => {
        evaluate_node(src, lhs, variables, errors) + evaluate_node(src, rhs, variables, errors)
      }
      Operator::Minus => {
        evaluate_node(src, lhs, variables, errors) - evaluate_node(src, rhs, variables, errors)
      }
      Operator::Multiply => {
        evaluate_node(src, lhs, variables, errors) * evaluate_node(src, rhs, variables, errors)
      }
    },
    Node::Fact(fact) => evaluate_node(src, fact, variables, errors),
    Node::UnaryOperator(op, rhs) => match op {
      Operator::Minus => -evaluate_node(src, rhs, variables, errors),
      Operator::Plus => evaluate_node(src, rhs, variables, errors),
      // `* Fact` is not allowed in the grammar
      Operator::Multiply => unreachable!("`* Fact` should be unreachable."),
    },
    Node::Identifier(var_node) => {
      match variables.get(var_node.literal.as_str()).copied() {
        Some(num) => num,
        None => {
          let node_range = var_node.range.clone();

          errors.push(DiagnosticError::new(
            format!(
              "The identifier `{}`, has not yet been initialized.",
              &var_node.literal
            ),
            var_node.line,
            node_range.start + 1 - linebreak_index(src, node_range),
          ));

          // Continue recursing to handle multiple errors at once
          0
        }
      }
    }
    Node::Literal(lit) => lit.value,
  }
}
