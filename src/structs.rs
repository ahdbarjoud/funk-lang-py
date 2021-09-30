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

  #[derive(Debug)]
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

  #[derive(Debug)]
  pub struct Token {
    pub typ: TokenType,
    pub value: String
  }
}