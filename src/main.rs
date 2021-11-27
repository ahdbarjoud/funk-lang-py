use std::env;
use std::fs;

mod structs;
mod lexer;
mod parser;
mod interpreter;
use interpreter::Interpreter;

pub use crate::lexer::Lexer;
pub use crate::parser::Parser;
pub use crate::structs::structs::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Forgot to mention file to run.");
    }
    let filename = &args[1];
    let code = fs::read_to_string(filename).expect("Could not read file");

    let mut lexer = Lexer {
        code: code.clone(),
        last_pos: 0,
        pos: 0,
        line: 0,
        line_pos: 0,
        current_char: None,
        next_char: None
    };
    let tokens: Vec<Token> = lexer.lex();

    let mut parser = Parser {
        pos: 0,
        last_pos: tokens.len() - 1,
        current_token: None,
        next_token: None,
        tokens: tokens,
        source: code
    };
    let program: Vec<AST> = parser.parse();

    let mut interpreter = Interpreter { pos: 0, last_pos: program.len() - 1, asts: program, current_ast: None };
    interpreter.interpret();
















    // let mut parser = Parser {
    //     pos: 0,
    //     last_pos: tokens.len() - 1,
    //     current_token: None,
    //     next_token: None,
    //     tokens: tokens,
    // };
    // let program: Vec<AST> = parser.parse();

    // let mut interpreter = Interpreter {
    //     pos: 0,
    //     next_ast: None,
    //     current_ast: None,
    //     last_pos: program.len() - 1,
    //     asts: program,
    //     variables: HashMap::default(),
    // };

    // interpreter.interpret();
}
