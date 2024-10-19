mod error;
mod lexer;
mod node;
mod parser;
mod token;

use error::DiagnosticError;
use lexer::Lexer;
use parser::Parser;
use std::{env, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut args = env::args();

  // The first argument is usually the executable name
  args.next();

  let file_name = args.next();

  let string = match file_name {
    Some(ref file) => fs::read_to_string(file)?,
    None => {
      println!("expected a file to be passed.");
      std::process::exit(1)
    }
  };

  let file_name = file_name.unwrap();
  let mut lexer = Lexer::new(&string);
  let tokens = lexer.lex();

  let mut parser = Parser::from_tokens(&string, tokens);
  let parse_res = parser.parse();

  match parse_res {
    Ok(ast) => println!("The AST of the program is:\n{:#?}", ast),
    Err(errs) => handle_error(&file_name, errs),
  }

  Ok(())
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
