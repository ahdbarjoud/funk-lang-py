use structs::structs::*;

pub struct Interpreter {
  pub asts: Vec<AST>,
  pub current_ast: Option<AST>,
  pub pos: usize,
  pub last_pos: usize,
  pub env: Env
}

impl Interpreter {
  fn next(&mut self) {
    if self.pos > self.last_pos {
      self.current_ast = None;
    } else {
      self.current_ast = Some(self.asts[self.pos].clone());
      self.pos += 1;
    }
  }

  pub fn interpret(&mut self) {
    self.next();
    while self.current_ast != None {
      self.handle_ast(self.current_ast.clone().unwrap());
      self.next();
    }
  }

  fn handle_ast(&mut self, ast: AST) -> Object {
    if let AST::Expression(Expr::Binary(binop)) = ast { 
      let left = self.handle_ast(*binop.left);
      let right = self.handle_ast(*binop.right);

      if binop.op == BinOp::Add {
        let a = left + right;
        println!("{:?}", a);
        a
      } else {
        panic!("")
      }
    }
    else if let AST::Expression(Expr::Integer(val)) = ast {
      Object::Integer(Integer{ value: val })
    } 
    
    else {
      panic!("")
    }
  }

}