mod error;
mod interpreter;
mod lexer;
mod node;
mod parser;
mod token;
mod util;

use error::DiagnosticError;
use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;
use std::{env, fs, path::Path};
use token::{Token, TokenKind};
use util::token_info;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut args = env::args();

  // The first argument is usually the executable name
  let exec = args.next().unwrap();

  let mut print_lexed_tokens = false;
  let mut print_ast = false;
  let mut file_name = None;

  for arg in args {
    if arg == "--print-ast" || arg == "-a" {
      print_ast = true;
    } else if arg == "--print-tokens" || arg == "-t" {
      print_lexed_tokens = true;
    } else if arg == "--help" || arg == "-h" {
      print_help(&exec);
    } else if file_name.is_none() {
      file_name = Some(arg);
    }
  }

  let file_name = file_name.unwrap_or_else(|| {
    println!("expected a file to be passed.");
    std::process::exit(1);
  });
  let src = fs::read_to_string(&file_name)?;

  // Lex the input, handling invalid tokens
  let mut lexer = Lexer::new(&src);
  let tokens = lexer.lex();
  let lex_errors = get_lexer_errors(&src, &tokens);

  if !lex_errors.is_empty() {
    handle_error(&file_name, lex_errors);
  }

  if print_lexed_tokens {
    println!("The lexed tokens of the program are:\n{:#?}", &tokens);
  }

  // Parse the program using the lexed tokens
  let mut parser = Parser::from_tokens(&src, tokens);
  let ast = parser
    .parse()
    .unwrap_or_else(|err| handle_error(&file_name, err));

  if print_ast {
    println!("The AST of the program is:\n{:#?}", &ast);
  }

  // Run the program
  let mut interpreter = Interpreter::new(&src, ast);

  match interpreter.evaluate() {
    Ok(()) => {
      println!("The result of the program is:\n");

      interpreter.dump();
    }
    Err(errors) => handle_error(&file_name, errors),
  }

  Ok(())
}

fn print_help(exec_path: &str) -> ! {
  let path = Path::new(exec_path);

  println!(
    "An interpreter for a toy language.\n\n\
USAGE: {} [OPTIONS] <file>\n\nOPTIONS:\n\
\t--print-tokens, -a\n\t\tPrints the lexed tokens of the source file.\n\n\
\t--print-ast, -t\n\t\tPrints the AST of the source file.\n\n\
\t--print-help, -h\n\t\tPrints this message.",
    path.file_name().unwrap().to_string_lossy()
  );

  std::process::exit(0)
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
