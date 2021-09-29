pub mod lexer{
  const OPEARTORS: [&'static str; 24] = [
    "+", "-", "*", "/", "%", "^",
    ">", "<", ">=", "<=", "==", "!=",
    "!", "&", ":", "?", "|", "::", "~", "#",
    "&&", "||", "++", "--"
  ];
  const KEYWORDS: [&'static str; 17] = [
    "funk", "while", "for", "when", "if",
    "elseif", "else", "Integer", "String",
    "Decimal", "klass", "not", "return",
    "Boolean", "Array", "Hash", "mut"
  ];

  #[derive(Debug)]
  enum TokenType {
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
    Keyword
  }

  #[derive(Debug)]
  pub struct Token {
    typ: TokenType,
    value: String
  }

  #[derive(Debug)]
  pub struct Lexer {
    pub code: String,
    pub current_char: Option<char>,
    pub next_char: Option<char>,
    pub pos: usize,
    pub last_pos: usize,
    pub tokens: Vec<Token>
  }

  impl Lexer {
    fn next(&mut self){
      self.pos += 1;

      if self.pos > self.last_pos {
        self.current_char = None;
        self.next_char = None;
        return;
      }

      self.current_char = self.code.chars().nth(self.pos);

      if self.pos + 1 > self.last_pos {
        self.next_char = None;
        return;
      } 

      self.next_char = self.code.chars().nth(self.pos + 1);
    }

    pub fn parse(&mut self) {
      while self.current_char != None {
        if self.current_char.unwrap().is_whitespace() {
          if self.current_char.unwrap() == '\n' {
            self.tokens.push(Token{typ: TokenType::Newline, value: String::from(self.current_char.unwrap())});
          }
          self.next()
        }
        else if self.current_char.unwrap() == '(' {
          self.tokens.push(Token{typ: TokenType::LPar, value: String::from(self.current_char.unwrap())});
          self.next()
        }
        else if self.current_char.unwrap() == ')' {
          self.tokens.push(Token{typ: TokenType::RPar, value: String::from(self.current_char.unwrap())});
          self.next()
        }
        else if self.current_char.unwrap() == '{' {
          self.tokens.push(Token{typ: TokenType::LCurl, value: String::from(self.current_char.unwrap())});
          self.next()
        }
        else if self.current_char.unwrap() == '}' {
          self.tokens.push(Token{typ: TokenType::RCurl, value: String::from(self.current_char.unwrap())});
          self.next()
        }
        else if self.current_char.unwrap() == '[' {
          self.tokens.push(Token{typ: TokenType::LBrac, value: String::from(self.current_char.unwrap())});
          self.next()
        }
        else if self.current_char.unwrap() == ']' {
          self.tokens.push(Token{typ: TokenType::RBrac, value: String::from(self.current_char.unwrap())});
          self.next()
        }
        else if self.current_char.unwrap() == ';' {
          self.tokens.push(Token{typ: TokenType::Semi, value: String::from(self.current_char.unwrap())});
          self.next()
        }
        else if self.current_char.unwrap() == ',' {
          self.tokens.push(Token{typ: TokenType::Comma, value: String::from(self.current_char.unwrap())});
          self.next()
        }
        else if self.current_char.unwrap() == '\'' || self.current_char.unwrap() == '"' {
          let string = self.parse_string(self.current_char.unwrap());
          self.tokens.push(Token{typ: TokenType::String, value: string});
        }
        else if self.current_char.unwrap().is_numeric() {
          let num = self.parse_num();
          self.tokens.push(Token{ typ: TokenType::Number, value: num})
        }
        else if OPEARTORS.contains(&&*self.current_char.unwrap().to_string()){
          let op = self.parse_ops();
          self.tokens.push(Token{ typ: TokenType::Operator, value: op})
        }
      }

      for i in &self.tokens {
        println!("{:?}", i)
      }
    }

    fn parse_string(&mut self, achar: char) -> String {
      let mut string = String::new();
      let mut end = false;
      self.next();

      while self.current_char != None {
        if self.current_char.unwrap() == achar {
          self.next();
          end = true;
          break;
        }

        if self.current_char.unwrap() == '\\' {
          self.next();
          if self.current_char.unwrap() == 'n' {
            string.push('\\');
            string.push('n');
          }

          else {
            string.push(self.next_char.unwrap())
          }
          self.next();
          continue;

        } else {
          string.push(self.current_char.unwrap());
        }
        self.next();
      }

      if self.current_char == None && ! end {
        panic!("A string was not properly closed.")
      }

      return string;
    }

    fn parse_num(&mut self) -> String {
      let mut num = String::new();
      while self.current_char != None && self.current_char.unwrap().is_numeric() {
        num += &String::from(self.current_char.unwrap());
        self.next()
      }
      return num;
    }

    fn parse_ops(&mut self) -> String {
      let mut op = String::new();

      while self.current_char != None && OPEARTORS.contains(&&*self.current_char.unwrap().to_string()) {
        op += &String::from(self.current_char.unwrap());
        self.next()
      }

      if ! OPEARTORS.contains(&&*op) {
        panic!("Could not recognize {} in file.", op);
      }
      return op
    }
  }
}