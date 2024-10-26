mod error;
mod lexer;
mod node;
mod parser;
mod token;
mod util;

use error::DiagnosticError;
use lexer::Lexer;
use parser::Parser;
use std::{env, fs};
use token::{Token, TokenKind};
use util::token_info;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut args = env::args();

  // The first argument is usually the executable name
  args.next();

  let file_name = args.next();
  let src = match file_name {
    Some(ref file) => fs::read_to_string(file)?,
    None => {
      println!("expected a file to be passed.");
      std::process::exit(1)
    }
  };

  let file_name = file_name.unwrap();
  // Lex the input, handling invalid tokens
  let mut lexer = Lexer::new(&src);
  let tokens = lexer.lex();
  let lex_errors = get_lexer_errors(&src, &tokens);

  if !lex_errors.is_empty() {
    handle_error(&file_name, lex_errors);
  }

  let mut parser = Parser::from_tokens(&src, tokens);
  let parse_res = parser.parse();

  match parse_res {
    Ok(ast) => println!("The AST of the program is:\n{:#?}", ast),
    Err(errs) => handle_error(&file_name, errs),
  }

  Ok(())
}

fn get_lexer_errors(src: &str, tokens: &[Token]) -> Vec<DiagnosticError> {
  let mut errors = Vec::new();

  for tok in tokens {
    if matches!(tok.kind(), TokenKind::Unknown) {
      let info = token_info(src, tok);

      errors.push(DiagnosticError::new(
        format!("The token, `{}`, is invalid.", info.literal),
        info.line,
        info.column,
      ))
    }
  }

  errors
}

fn handle_error(file_name: &str, errors: Vec<DiagnosticError>) -> ! {
  let mut index = 1;
  let num_errors = errors.len();
  eprintln!("The program has {} error(s):\n", num_errors);

  for err in errors.into_iter() {
    eprintln!(
      "{:>2}) {}:{}:{}\n\t{}",
      index,
      file_name,
      err.line(),
      err.column(),
      err
    );

    if index != num_errors {
      eprintln!();
    }

    index += 1;
  }

  std::process::exit(1)
}
