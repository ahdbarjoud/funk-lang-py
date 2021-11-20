pub mod interpreter {
    use std::collections::hash_map::HashMap;
    pub use structs::structs::*;

    pub struct Interpreter {
        pub pos: usize,
        pub last_pos: usize,
        pub current_ast: Option<AST>,
        pub next_ast: Option<AST>,
        pub asts: Vec<AST>,
        pub variables: HashMap<String, AST>,
    }

    impl Interpreter {
        fn next(&mut self) -> Option<AST> {
            if self.pos > self.last_pos {
                // Means we reached the end.
                // Set everything to None.
                self.current_ast = None;
                self.next_ast = None;
                return None;
            }
            self.current_ast = Some(self.asts[self.pos].clone()); // Update current_token

            if self.pos + 1 > self.last_pos {
                // Means we are on the last token.
                // Set next_token to None.
                self.next_ast = None;
                self.pos += 1;
                return None;
            }
            self.next_ast = Some(self.asts[self.pos + 1].clone()); // Update next_token
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
            if let AST::BiOpAST { op, left, right } = ast {
                let left = self.handle_ast(*left);
                let right = self.handle_ast(*right);

                if op == String::from("+") {
                    return left + right;
                } else if op == String::from("-") {
                    return left - right;
                } else if op == String::from("*") {
                    return left * right;
                } else if op == String::from("/") {
                    return left / right;
                } else {
                    panic!("Unhandled AST.");
                }
            } else if let AST::Integer { value: _ } = ast {
                return ast;
            } else if let AST::Decminal { value: _ } = ast {
                return ast;
            } else if let AST::Str { value: _ } = ast {
                return ast;
            } else if let AST::Assgignment {
                ref name,
                ref value,
                ref var_type,
                ref scope,
                line,
            } = ast
            {
                let val = self.handle_ast(*value.clone());
                let var = AST::Variable {
                    name: name.clone(),
                    value: Box::new(val),
                    var_type: var_type.clone(),
                    scope: scope.to_string(),
                    line: line,
                };
                self.variables.insert(name.clone(), var.clone());
                var
            } else if let AST::CallItem {
                name,
                call_type,
                args: _,
                scope: _,
            } = ast
            {
                if call_type == "VariableCall" {
                    match self.variables.get(&name) {
                        Some(var) => match var {
                            AST::Variable {
                                name: _,
                                value,
                                var_type: _,
                                scope: _,
                                line: _,
                            } => {
                                return *value.clone();
                            }
                            _ => {
                                panic!("");
                            }
                        },
                        None => {
                            panic!("");
                        }
                    }
                } else {
                    panic!("");
                }
            } else {
                panic!("Unhandled AST.")
            }
        }
    }
}
