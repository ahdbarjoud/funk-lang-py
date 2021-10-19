pub mod structs {
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
    Variable
  }

  #[derive(Debug, Clone, PartialEq)]
  pub struct Token {
    pub typ: TokenType,
    pub value: String,
    pub line: usize,
    pub line_pos: usize
  }

  #[derive(PartialEq, Debug)]
  pub enum OPS {
    BiOp,
    Addition,
    Multiplication
  }

  #[derive(PartialEq, Debug)]
  pub enum AST {
    BiOpAst(String, Box<AST>, Box<AST>),
    UnOp(OPS, String, Box<AST>),
    Number(i64),
    Decminal(f64),
    Nothing
  }
}