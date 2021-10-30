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
    // Move on to the next token, return the next token.
    fn next(&mut self) -> Option<Token> {
      if self.pos > self.last_pos { // Means we reached the end.
        // Set everything to None.
        self.current_token = None;
        self.next_token = None;
        return None;
      }
      self.current_token = Some(self.tokens[self.pos].clone()); // Update current_token

      if self.pos + 1 > self.last_pos { // Means we are on the last token.
        // Set next_token to None.
        self.next_token = None;
        self.pos += 1;
        return None;
      }
      self.next_token = Some(self.tokens[self.pos+1].clone()); // Update next_token
      self.pos += 1; // Update our position

      return Some(self.tokens[self.pos].clone()); // Return the next token.
    }

    fn expect(&mut self, typ_tkn: Option<Vec<TokenType>>, typ_str: Option<Vec<String>>) {
      if self.current_token == None {
        self.next();
      } else {
        if typ_tkn != None {
          if ! typ_tkn.as_ref().unwrap().contains(&self.current_token.as_ref().unwrap().typ) {
            panic!("Expected one of: {:?} on line {}, instead found {:?}", typ_tkn.clone().unwrap(),
              self.current_token.as_ref().unwrap().line, self.current_token.as_ref().unwrap().typ);
          } else {}
        }
        if typ_str != None {
          if ! typ_str.unwrap().contains(&self.current_token.as_ref().unwrap().value) {
            panic!("Expected one of: {:?} on line {}, instead found {:?}", typ_tkn.clone().unwrap(),
              self.current_token.as_ref().unwrap().line, self.current_token.as_ref().unwrap().value);
          } else {}
        }
        self.next();
      }
    }

    fn skip_newlines(&mut self) {
      while self.current_token.as_ref().unwrap().typ == TokenType::Newline {
        self.next();
      }
    }

    pub fn parse(&mut self) -> Vec<AST> {
      let mut program: Vec<AST> = Vec::new();
      self.next(); // Get the initial tokens, otherwise we would have "None"

      while self.current_token != None { // Loop until no more token.
        // Skip newlines,  I should make this a funtion.
        if self.current_token.as_ref().unwrap().typ == TokenType::Newline {
          self.next();
          continue;
        }
        program.push(self.parse_top()); // Call parse top and append result to our program vec.
        self.next(); // Go to next token, repeat the process yeet.
      }
      return program; // Return our AST Vec, sir program.
    }

    fn parse_top(&mut self) -> AST {
      if self.current_token.as_ref().unwrap().typ != TokenType::Keyword {
        let expr = self.parse_expr(); // This is an expression, so lettus parse it.
        // self.expect(Some(vec!(TokenType::Semi, TokenType::Newline)), None);
        expr
      } else {
        if ["Integer".to_string(), "Decimal".to_string(), "String".to_string()].contains(&self.current_token.as_ref().unwrap().value) {
          self.parse_assignment()
        } else if self.current_token.as_ref().unwrap().value == "funk" {
          self.expect(None, Some(vec!("funk".to_string())));
          self.parse_function()
        } else if self.current_token.as_ref().unwrap().value == "if" {
          self.parse_conditional()
        }
        else {
          AST::Number(Integer { value: 0 })
        }
      }
    }

    fn parse_expr(&mut self) -> AST {
      let mut result = self.parse_term();

      if self.current_token != None && self.current_token.as_ref().unwrap().typ == TokenType::Operator &&
      ["+".to_string(), "-".to_string(), "==".to_string()].contains(&self.current_token.as_ref().unwrap().value) {
        let op: &String = &self.current_token.as_ref().unwrap().value.clone();
        self.expect(Some(vec!(TokenType::Operator)), None);
        result = AST::BiOpAst(op.to_string(), Box::new(result), Box::new(self.parse_expr()));
      }
      result
    }

    fn parse_term(&mut self) -> AST {
      let mut result = self.parse_factor();

      if self.current_token != None && self.current_token.as_ref().unwrap().typ == TokenType::Operator && 
      ["*".to_string(), "/".to_string()].contains(&self.current_token.as_ref().unwrap().value) {
        let op = &self.current_token.as_ref().unwrap().value.clone();
        self.expect(Some(vec!(TokenType::Operator)), None);
        result = AST::BiOpAst(op.to_string(), Box::new(result), Box::new(self.parse_term()));
      }
      result
    }

    fn parse_factor(&mut self) -> AST {
      if self.current_token.as_ref().unwrap().typ == TokenType::Number { // Handle Numbers
        if self.current_token.as_ref().unwrap().value.contains('.') {
          let val = self.current_token.as_ref().unwrap().value.parse::<f64>().unwrap(); // Decimal Numbers
          self.expect(Some(vec!(TokenType::Number)), None);
          AST::Decminal(Decimal { value: val })
        } else { // Integer Numbers
          let val = self.current_token.as_ref().unwrap().value.parse::<i64>().unwrap();
          self.expect(Some(vec!(TokenType::Number)), None);
          AST::Number( Integer { value: val })
        }
      }

      else if self.current_token.as_ref().unwrap().typ == TokenType::String { // Handle Strings
        let val = self.current_token.clone().unwrap().value;
        self.expect(Some(vec!(TokenType::String)), None);
        AST::Str(Str { value: val })
      }

      else if self.current_token.as_ref().unwrap().typ == TokenType::LPar { // Handle Pars
        self.expect(Some(vec!(TokenType::LPar)), None);
        let result = self.parse_expr();
        self.expect(Some(vec!(TokenType::RPar)), None);
        return result;
      } else if self.current_token.as_ref().unwrap().typ == TokenType::Identifier {
        let var = self.current_token.clone();
        self.expect(Some(vec!(TokenType::Identifier)), None);
        AST::Var(Variable{ name: var.unwrap().value })
      }

      else {
        panic!("Unknown token: {:?} on line {}.", self.current_token.as_ref().unwrap(), self.current_token.as_ref().unwrap().line);
      }
    }

    fn parse_assignment(&mut self) -> AST {
      let line = self.current_token.clone().unwrap().line;
      self.expect(Some(vec!(TokenType::Keyword)), None);

      let var_name = self.current_token.clone().unwrap().value;
      self.expect(Some(vec!(TokenType::Identifier)), None);
      self.expect(None, Some(vec!("=".to_string())));
    
      let value: AST = self.parse_expr();
      self.expect(Some(vec!(TokenType::Semi, TokenType::Newline)), None);

      AST::Assign(Assgignment { name: var_name, value: Box::new(value), scope: "Global".to_string(), line: line })
    }

    fn parse_function(&mut self) -> AST {
      let expected_return = self.current_token.clone().unwrap().value;
      self.expect(None, Some(vec!("Integer".to_string(), "Decimal".to_string(), "String".to_string())));

      let funk_name = self.current_token.clone().unwrap().value;
      self.expect(Some(vec!(TokenType::Identifier)), None);

      let params: Vec<Box<AST>> = self.parse_funk_params();
      let body: Vec<Box<AST>> = self.parse_body();

      AST::Funk(Funktion { name: funk_name, return_typ: expected_return, params: params, body: body })
    }

    fn parse_funk_params(&mut self) -> Vec<Box<AST>> {
      let mut params: Vec<Box<AST>> = vec!();
      self.expect(Some(vec!(TokenType::LPar)), None);

      while self.current_token != None {
        if self.current_token.as_ref().unwrap().typ == TokenType::RPar {
          self.expect(Some(vec!(TokenType::RPar)), None);
          break;
        }
        else if self.current_token.as_ref().unwrap().typ == TokenType::LCurl {
          break;
        }

        let param_typ = self.current_token.clone().unwrap().value;
        self.expect(None, Some(vec!("Integer".to_string(), "Decimal".to_string(), "String".to_string())));
        let param_name = self.current_token.clone().unwrap().value;
        self.expect(Some(vec!(TokenType::Identifier)), None);
        self.expect(Some(vec!(TokenType::Comma, TokenType::RPar)), None);
        params.push(Box::new(AST::FunkParam(FunkParameter { name: param_name, typ: param_typ })))
      }
      params
    }

    fn parse_body(&mut self) -> Vec<Box<AST>> {
      self.expect(Some(vec!(TokenType::LCurl)), None);
      let mut body: Vec<Box<AST>> = vec!();

      while self.current_token != None && self.current_token.as_ref().unwrap().typ != TokenType::RCurl {
        self.skip_newlines();

        if self.current_token != None && self.current_token.as_ref().unwrap().typ == TokenType::RCurl {
          break;
        }

        body.push(Box::new(self.parse_top()));
        self.next();
      }
      self.expect(Some(vec!(TokenType::RCurl)), None);
      body
    }

    fn parse_conditional(&mut self) -> AST {
      let typ = self.current_token.clone();
      self.expect(None, Some(vec!(typ.clone().unwrap().value)));

      if typ.as_ref().unwrap().value == "else" {
        return AST::Cond(Conditional{ typ: typ.clone().unwrap().value, body: self.parse_body(), expr: None, other: None });
      }

      let expr = Some(Box::new(self.parse_expr()));
      let body = self.parse_body();
      let mut other: Option<Box<AST>> = None;

      if self.current_token != None && self.current_token.as_ref().unwrap().typ == TokenType::Keyword &&
      ["elif".to_string(), "else".to_string()].contains(&self.current_token.clone().unwrap().value) {
        other = Some(Box::new(self.parse_conditional()));
      }

      AST::Cond(Conditional{ typ: typ.clone().unwrap().value, body: body, expr: expr, other: other })
    }
  }
}