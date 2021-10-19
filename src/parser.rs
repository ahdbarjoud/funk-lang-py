pub mod parser {
  pub use structs::structs::*;
  // use std::iter::Peekable;
  // use std::slice::Iter;

  #[derive(Debug)]
  pub struct Parser {
    pub pos: usize,
    pub last_pos: usize,
    pub current_token: Option<Token>,
    pub next_token: Option<Token>,
    pub tokens: Vec<Token>
  }

  impl Parser {
    fn next(&mut self) -> Option<Token> {
      if self.pos > self.last_pos {
        self.current_token = None;
        self.next_token = None;
        return None;
      }
      self.current_token = Some(self.tokens[self.pos].clone());

      if self.pos + 1 > self.last_pos {
        self.next_token = None;
        self.pos += 1;
        return None;
      }
      self.next_token = Some(self.tokens[self.pos+1].clone());

      self.pos += 1;

      return Some(self.tokens[self.pos].clone());
    }

    pub fn parse(&mut self) -> Vec<AST> {
      let mut program: Vec<AST> = Vec::new();
      self.next();

      while self.current_token != None {
        if self.current_token.as_ref().unwrap().typ == TokenType::Newline {
          self.next();
          continue;
        }
        program.push(self.parse_top());

        self.next();
      }

      return program;
    }

    fn parse_top(&mut self) -> AST {

      if self.current_token.as_ref().unwrap().typ != TokenType::Keyword {
        return self.parse_expr();
      } else {
        return AST::Number(0);
      }
    }

    fn parse_expr(&mut self) -> AST {
      let mut result = self.parse_term();

      if self.current_token != None && self.current_token.as_ref().unwrap().typ == TokenType::Operator && ["+".to_string(), "-".to_string()].contains(&self.current_token.as_ref().unwrap().value) {
        let op: &String = &self.current_token.as_ref().unwrap().value.clone();
        self.next();
        result = AST::BiOpAst(op.to_string(), Box::new(result), Box::new(self.parse_expr()));
      }

      result
    }

    fn parse_term(&mut self) -> AST {
      let mut result = self.parse_factor();

      if self.current_token != None && self.current_token.as_ref().unwrap().typ == TokenType::Operator && ["*".to_string(), "/".to_string()].contains(&self.current_token.as_ref().unwrap().value) {
        let op = &self.current_token.as_ref().unwrap().value.clone();
        self.next();
        result = AST::BiOpAst(op.to_string(), Box::new(result), Box::new(self.parse_factor()));
      }

      result
    }

    fn parse_factor(&mut self) -> AST{
      if self.current_token.as_ref().unwrap().typ == TokenType::Number {
        if self.current_token.as_ref().unwrap().value.contains('.') {
          let val = self.current_token.as_ref().unwrap().value.parse::<f64>().unwrap();
          self.next();
          AST::Decminal(val)
        } else {
          let val = self.current_token.as_ref().unwrap().value.parse::<i64>().unwrap();
          self.next();
          AST::Number(val)
        }
      } else {
        panic!("Unknown token on line {}.", self.current_token.as_ref().unwrap().line);
      }
    }
  }
}