use std::fs;
use std::env;

mod lexer;
mod structs;
mod parser;
mod interpreter;
pub use crate::lexer::lexer::Lexer;
pub use crate::parser::parser::Parser;
pub use crate::interpreter::interpreter::Interpreter;
pub use crate::structs::structs::*;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    panic!("Forgot to mention file to run.");
  }
  let filename = &args[1];
  let code = fs::read_to_string(filename)
    .expect("Could not read file");

  let mut lexer = Lexer{ code: String::new(), last_pos: 0, pos: 0, line: 1, line_pos: 0};
  let tokens: Vec<Token> = lexer.lex(code);

  let mut parser = Parser{pos: 0, last_pos: tokens.len() - 1, current_token: None, next_token: None, tokens: tokens};
  let program: Vec<AST> = parser.parse();

  let mut interpreter = Interpreter { pos: 0, next_ast: None, current_ast: None, last_pos: 0, asts: program };

  interpreter.interpret();
}
