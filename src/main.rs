use std::fs;
use std::env;

mod lexer;
pub use crate::lexer::lexer::Lexer;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    panic!("Forgot to mention file to run."
  );
  }

  let filename = &args[1];

  let code = fs::read_to_string(filename)
    .expect("Could not read file");
  let curr = code.chars().nth(0);
  let count = code.chars().count();

  let mut lexer: Lexer = Lexer{ code: code, current_char: curr, next_char: None, pos: 0,
    last_pos: count, tokens:  Vec::new()};

  lexer.parse();
}