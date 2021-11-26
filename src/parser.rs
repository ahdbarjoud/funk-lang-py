// use std::ops::Range;
use structs::structs::*;

pub struct Parser {
  pub pos: usize,
  pub last_pos: usize,
  pub current_token: Option<Token>,
  pub next_token: Option<Token>,
  pub tokens: Vec<Token>,
  pub source: String
}

impl Parser {
  fn next(&mut self) -> Option<Token> {
    if self.pos > self.last_pos {
        // Means we reached the end.
        // Set everything to None.
        self.current_token = None;
        self.next_token = None;
        return None;
    }
    self.current_token = Some(self.tokens[self.pos].clone()); // Update current_token

    if self.pos + 1 > self.last_pos {
        // Means we are on the last token.
        // Set next_token to None.
        self.next_token = None;
        self.pos += 1;
        return None;
    }
    self.next_token = Some(self.tokens[self.pos + 1].clone()); // Update next_token
    self.pos += 1; // Update our position

    return Some(self.tokens[self.pos].clone()); // Return the next token.
  }
  
  fn expect(&mut self, ty: Vec<TokenType>) {
    match self.current_token.clone().unwrap() {
      token if ty.contains(&token.ty) => { self.next(); },
      _ => panic!("Expected [{:?}], instead found [{:?}]", ty, self.current_token.clone().unwrap().ty)
    }
  }
  
  fn skip_newlines(&mut self) {
    while self.current_token != None && self.current_token.clone().unwrap().ty == TokenType::Newline {
      self.next();
    }
  }

  pub fn parse(&mut self) -> Vec<AST> {
    let mut program: Vec<AST> = Vec::new();
    self.next();
    self.skip_newlines();

    while self.current_token != None {
      program.push(self.parse_top());
      self.next();
      self.skip_newlines();
    }
    return program
  }

  fn parse_top(&mut self) -> AST {
    let current = self.current_token.clone().unwrap().clone();

    if let TokenType::Keyword(_) = &current.ty {
      panic!("Keywords not handled yet")
    } else {
      self.parse_expr()
    }
  }

  fn parse_expr(&mut self) -> AST {
    let mut result = self.parse_term();

    while self.current_token != None && (self.current_token.clone().unwrap().ty == TokenType::Plus || self.current_token.clone().unwrap().ty == TokenType::Minus) {
      let op = BinOp::new(&self.current_token.clone().unwrap().ty);
      self.expect(Vec::from([TokenType::Plus, TokenType::Minus]));
      result = AST::Expression(Expr::Binary(BinaryExpr { left: Box::new(result), op, right: Box::new(self.parse_expr()) }));
    }
    return result
  }

  fn parse_term(&mut self) -> AST {
    let mut result = self.parse_factor();

    while self.current_token != None && (self.current_token.clone().unwrap().ty == TokenType::Star || self.current_token.clone().unwrap().ty == TokenType::Slash) {
      let op = BinOp::new(&self.current_token.clone().unwrap().ty);
      self.expect(Vec::from([TokenType::Star, TokenType::Slash]));
      result = AST::Expression(Expr::Binary(BinaryExpr { left: Box::new(result), op, right: Box::new(self.parse_expr()) }));
    }
    return result;
  }

  fn parse_factor(&mut self) -> AST {
    let current = self.current_token.clone().unwrap();
  
    if TokenType::Literal(Val::Number) == current.ty {
      let num_str = self.source[current.range].to_string();
      self.expect(Vec::from([TokenType::Literal(Val::Number)]));
      if num_str.contains('.') {
        AST::Expression(Expr::Decimal(num_str.parse::<f64>().unwrap()))
      } else {
        AST::Expression(Expr::Integer(num_str.parse::<i64>().unwrap()))
      }
    } else {
      panic!("")
    }
  }
}
