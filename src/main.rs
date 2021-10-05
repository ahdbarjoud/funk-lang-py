use std::fs;
use std::env;

mod lexer;
mod structs;
mod parser;
pub use crate::lexer::lexer::Lexer;
pub use crate::parser::parser::Parser;
pub use crate::structs::structs::Token;
pub use crate::structs::structs::AstType;
pub use crate::structs::structs::BiOpAst;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    panic!("Forgot to mention file to run."
  );
  }
  let filename = &args[1];
  let code = fs::read_to_string(filename)
    .expect("Could not read file");

  let mut lexer = Lexer{ code: String::new(), last_pos: 0, pos: 0, line: 1, line_pos: 0};
  let tokens: Vec<Token> = lexer.lex(code);
  for i in &tokens {
    println!("{:?}", i);
  }

  let mut parser = Parser{tokens: tokens};
}