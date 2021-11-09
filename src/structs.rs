pub mod structs {
  use std::ops::*;

  pub const OPEARTORS: [&'static str; 28] = [
    "+", "-", "*", "/", "%", "^",
    ">", "<", ">=", "<=", "==", "!=",
    "!", "&", ":", "?", "|", "::", 
    "#", "&&", "||", "++", "--", "=", 
    "###", ".", "..", "..."
  ];
  pub const KEYWORDS: [&'static str; 18] = [
    "funk", "while", "for", "when", "if",
    "elseif", "else", "Integer", "String",
    "Decimal", "klass", "in", "return",
    "Boolean", "Array", "Hash", "mut", "Void"
  ];

  #[derive(Debug, Clone, PartialEq)]
  pub enum TokenType {
    LPar,
    RPar,
    Number,
    Newline,
    LCurl,
    RCurl,
    LBrac,
    RBrac,
    Comma,
    Semi,
    String,
    Operator,
    Keyword,
    Identifier
  }

  #[derive(Debug, Clone, PartialEq)]
  pub struct Token {
    pub typ: TokenType,
    pub value: String,
    pub line: usize,
    pub line_pos: usize
  }

  #[derive(PartialEq, Debug, Clone)]
  pub enum AST{
    BiOpAST{op: String, left: Box<AST>, right: Box<AST>},
    UnOp{op: String, value: Box<AST>},
    Integer{value: i64},
    Decminal{value: f64},
    Str{value: String},
    Assgignment{name: String, value: Box<AST>, scope: String, line: usize},
    FunktionParameter{name: String, typ: String},
    Funktion{name: String, return_typ: String, params: Vec<Box<AST>>, body: Vec<Box<AST>>},
    Conditional{typ: String, expr: Option<Box<AST>>, body: Vec<Box<AST>>, other: Option<Box<AST>>},
    CallItem{name: String, call_type: String, args: Option<Vec<Box<AST>>>, scope: String}
  }

  impl Add for AST {
    type Output = Self;

    fn add(self, other: Self) -> Self{
      if let AST::Integer{value} = self {
        let val1 = value;

        if let AST::Integer{value} = other {
          let val2 = value;
          return Self::Integer{value: val1 + val2}
        } else if let AST::Decminal{value} = other {
          let val2 = value;
          let val1 = val1 as f64;
          return Self::Decminal{value: val1 + val2}
        } else {
          panic!("Add AST panic.")
        }
      } else if let AST::Decminal{value} = self {
        let val1 = value;

        if let AST::Decminal{value} = other {
          let val2 = value;
          return Self::Decminal{value: val1 + val2}
        } else if let AST::Integer{value} = other {
          let val2 = value as f64;
          return Self::Decminal{value: val1 + val2}
        } else {
          panic!("Add AST panic.")
        }
      } else {
        panic!("Add AST panic.")
      }
    }
  }

  impl Sub for AST {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
      if let AST::Integer{value} = self {
        let val1 = value;

        if let AST::Integer{value} = other {
          let val2 = value;
          Self::Integer{value: val1 - val2}
        } else if let AST::Decminal{value} = other {
          let val2 = value;
          let val1 = val1 as f64;
          Self::Decminal{value: val1 - val2}
        } else {
          panic!("Sub AST panic.")
        }
      } else if let AST::Decminal{value} = self {
        let val1 = value;

        if let AST::Decminal{value} = other {
          let val2 = value;
          Self::Decminal{value: val1 - val2}
        } else if let AST::Integer{value} = other {
          let val2 = value as f64;
          Self::Decminal{value: val1 - val2}
        } else {
          panic!("Sub AST panic.")
        }
      } else {
        panic!("Sub AST panic.")
      }
    }

  }
}