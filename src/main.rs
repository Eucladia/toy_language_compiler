mod lexer;
mod token;

use lexer::Lexer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let content = std::fs::read_to_string("./sample_input/4.txt")?;
  let mut lexer = Lexer::new(&content);
  let tokens = lexer.lex();

  println!("{:#?}", tokens);

  Ok(())
}
