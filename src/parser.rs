pub mod parser {
  pub use structs::structs::Token;
  pub use structs::structs::TokenType;
  pub use structs::structs::OPEARTORS;
  pub use structs::structs::KEYWORDS;

  #[derive(Debug)]
  pub struct Parser {
    pub tokens: Vec<Token>
  }

}