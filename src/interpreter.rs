pub mod interpreter {
  pub use structs::structs::*;

  pub struct Interpreter {
    pub pos: usize,
    pub last_pos: usize,
    pub current_ast: Option<AST>,
    pub next_ast: Option<AST>,
    pub asts: Vec<AST>
  }

  
  impl Interpreter {
    fn next(&mut self) -> Option<AST> {
      if self.pos > self.last_pos { // Means we reached the end.
        // Set everything to None.
        self.current_ast = None;
        self.next_ast = None;
        return None;
      }
      self.current_ast = Some(self.asts[self.pos].clone()); // Update current_token

      if self.pos + 1 > self.last_pos { // Means we are on the last token.
        // Set next_token to None.
        self.next_ast = None;
        self.pos += 1;
        return None;
      }
      self.next_ast = Some(self.asts[self.pos+1].clone()); // Update next_token
      self.pos += 1; // Update our position

      return Some(self.asts[self.pos].clone()); // Return the next token.
    }

    pub fn interpret(&mut self) {
      self.next();

      while self.current_ast != None {
        let a = self.handle_ast(self.current_ast.clone().unwrap());
        println!("{:?}", a);
        self.next();
      }
    }

    fn handle_ast(&mut self, ast: AST) -> AST {
      if let AST::BiOpAST{op, left, right} = ast {
        let left = self.handle_ast(*left);
        let right = self.handle_ast(*right);

        if op == String::from("+") {
          return left + right;
        } else if op == String::from("-") {
          return left - right;
        }

      } else if let AST::Integer{value: _} = ast {
        return ast;
      } else if let AST::Decminal{value: _} = ast {
        return ast;
      } else if let AST::Str{value: _} = ast {
        return ast;
      }

      return AST::Integer{value: 0};
    }
  }

}