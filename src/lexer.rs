pub mod lexer{
  use structs::structs::Token;
  use structs::structs::TokenType;
  use structs::structs::OPEARTORS;
  use structs::structs::KEYWORDS;
  use std::slice::Iter;
  use std::iter::Peekable;
  

  #[derive(Debug)]
  pub struct Lexer {
    pub code: String,
    pub pos: usize,
    pub last_pos: usize,
    pub line_pos: usize,
    pub line: usize
  }

  impl Lexer {
    pub fn lex(&mut self, code: String) -> Vec<Token> {
      self.code = code;
      let code = self.code.chars().collect::<Vec<char>>();
      self.last_pos = code.iter().count();
      let mut iter = code.iter().peekable();

      return self.parse_tokens(&mut iter);
    }

    fn parse_tokens(&mut self, iter: &mut Peekable<Iter<char>>) -> Vec<Token> {
      let mut tokens: Vec<Token> = Vec::new();

      while self.pos < self.last_pos {
        self.pos += 1;
        self.line_pos += 1;
        let current: Option<&char> = iter.next();

        if current == None {
          break;
        }

        if current.unwrap().is_whitespace() {
          if current.unwrap() == &'\n' {
            self.line += 1;
            self.line_pos = 0;
            tokens.push(Token{typ: TokenType::Newline, value: String::from(*current.unwrap()), line: self.line, line_pos: self.line_pos});
          }
        } else if current.unwrap() == &'(' {
          tokens.push(Token{typ: TokenType::LPar, value: String::from(*current.unwrap()), line: self.line, line_pos: self.line_pos});
        } else if current.unwrap() == &')' {
          tokens.push(Token{typ: TokenType::RPar, value: String::from(*current.unwrap()), line: self.line, line_pos: self.line_pos});
        } else if current.unwrap() == &'{' {
          tokens.push(Token{typ: TokenType::LCurl, value: String::from(*current.unwrap()), line: self.line, line_pos: self.line_pos});
        } else if current.unwrap() == &'}' {
          tokens.push(Token{typ: TokenType::RCurl, value: String::from(*current.unwrap()), line: self.line, line_pos: self.line_pos});
        } else if current.unwrap() == &'[' {
          tokens.push(Token{typ: TokenType::LBrac, value: String::from(*current.unwrap()), line: self.line, line_pos: self.line_pos});
        } else if current.unwrap() == &']' {
          tokens.push(Token{typ: TokenType::RBrac, value: String::from(*current.unwrap()), line: self.line, line_pos: self.line_pos});
        } else if current.unwrap() == &';' {
          tokens.push(Token{typ: TokenType::Semi, value: String::from(*current.unwrap()), line: self.line, line_pos: self.line_pos});
        } else if current.unwrap() == &',' {
          tokens.push(Token{typ: TokenType::Comma, value: String::from(*current.unwrap()), line: self.line, line_pos: self.line_pos});
        } else if [&'\'', &'"'].contains(&current.unwrap()) {
          tokens.push(self.parse_string(iter, current.unwrap()));
        } else if [&'\'', &'"'].contains(&current.unwrap()) {
          tokens.push(self.parse_string(iter, current.unwrap()));
        } else if current.unwrap().is_numeric() {
          tokens.push(self.prase_number(iter, current.unwrap()));
        } else if current.unwrap().is_alphabetic() || current.unwrap() == &'_' {
          tokens.push(self.parse_identifier(iter, current.unwrap()));
        } else if OPEARTORS.contains(&&*current.unwrap().to_string()) {
          tokens.push(self.parse_operator(iter, current.unwrap()));
        }
      }
      return tokens;
    }

    fn parse_string(&mut self, iter: &mut Peekable<Iter<char>>, first: &char) -> Token {
      let mut string = String::new();
      let mut end = false;
      let start = self.line;
      let mut cur = iter.next();
      self.pos += 1;
      self.line_pos += 1;

      while cur != None {
        if cur.unwrap() == first {
          end = true;
          break;
        }

        if cur.unwrap() == &'\\' {
          cur = iter.next();
          self.pos += 1;
          self.line_pos += 1;

          if cur.unwrap() == &'n' {
            string.push('\\');
            string.push('n');
            self.line += 1;
            self.line_pos = 0;
          } else {
            string.push(*cur.unwrap());
            cur = iter.next();
            self.pos += 1;
            self.line_pos += 1;
          }
        } else {
          string.push(*cur.unwrap());
          cur = iter.next();
          self.pos += 1;
          self.line_pos += 1;
        }
      }

      if cur == None && ! end {
        panic!("String not closed properly on line {}.", start)
      }

      return Token{ typ: TokenType::String, value: string, line: self.line, line_pos: self.line_pos };
    }

    fn prase_number(&mut self, iter: &mut Peekable<Iter<char>>, first: &char) -> Token {
      let mut num = String::from(*first);
      let start = self.line_pos;

      while iter.peek() != None && (iter.peek().unwrap().is_numeric() || iter.peek().unwrap() == &&'.') {
        num.push(*iter.next().unwrap());
        self.pos += 1;
        self.line_pos += 1;
      }

      if num.matches('.').count() > 1 {
        panic!("Bad number encountered on line {}.", self.line);
      }
      return Token{typ: TokenType::Number, value: num, line: self.line, line_pos: start}
    }
  
    fn parse_identifier(&mut self, iter: &mut Peekable<Iter<char>>, first: &char) -> Token {
      let mut iden = String::from(*first);
      let start = self.line_pos;

      while iter.peek() != None && (iter.peek().unwrap().is_alphanumeric() || iter.peek().unwrap() == &&'_') {
        iden.push(*iter.next().unwrap());
        self.pos += 1;
        self.line_pos += 1;
      }

      if KEYWORDS.contains(&&*iden) {
        return Token{typ: TokenType::Keyword, value:iden, line: self.line, line_pos: start};
      }
      return Token{typ: TokenType::Identifier, value:iden, line: self.line, line_pos: start};
    }

    fn parse_operator(&mut self, iter: &mut Peekable<Iter<char>>, first: &char) -> Token {
      let mut op = String::from(*first);
      let start = self.line_pos;

      while OPEARTORS.contains(&&*iter.peek().unwrap().to_string()) {
        op.push(*iter.next().unwrap());
        self.pos += 1;
        self.line_pos += 1;
      }

      if ! OPEARTORS.contains(&&*op) {
        panic!("Unknown Operator on line {}.", self.line);
      }
      return Token{typ: TokenType::Operator, value: op, line: self.line, line_pos: start}
    }
  }
}