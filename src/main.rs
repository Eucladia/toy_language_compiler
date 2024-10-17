mod lexer;
mod token;

use lexer::Lexer;
use std::{env, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut args = env::args();

  // The first argument is usually the executable name
  args.next();

  let bytes = match args.next() {
    Some(file) => fs::read_to_string(file)?,
    None => {
      println!("expected a file to be passed.");
      std::process::exit(1)
    }
  };

  let mut lexer = Lexer::new(&bytes);
  let tokens = lexer.lex();

  println!("The lexed tokens:\n{:#?}", tokens);

  Ok(())
}
