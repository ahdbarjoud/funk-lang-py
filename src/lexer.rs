use std::slice::Iter;
use std::iter::Peekable;
use structs::structs::*;

pub struct Lexer {
  pub code: String,
  pub pos: usize,
  pub last_pos: usize,
  pub line_pos: usize,
  pub line: usize,
  pub current_char: Option<char>,
  pub next_char: Option<char>
}

impl Lexer {
  pub fn lex(&mut self) -> Vec<Token> {
    let chars = self.code.chars().collect::<Vec<char>>();

    self.last_pos = chars.len() - 1;
    let mut iter = chars.iter().peekable();

    self.parse_tokens(&mut iter)
  }

  fn parse_tokens(&mut self, iter: &mut Peekable<Iter<char>>) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    
    while self.pos <= self.last_pos {
      self.pos += 1;
      self.line_pos += 1;
      let current = iter.next();

      tokens.push(match current {
        None => {break},
        Some(cur) => {
          match cur {
            '(' => Token{ ty: TokenType::LPar, line: self.line, line_pos: self.line_pos },
            ')' => Token{ ty: TokenType::RPar, line: self.line, line_pos: self.line_pos },
            '{' => Token{ ty: TokenType::LCurl, line: self.line, line_pos: self.line_pos },
            '}' => Token{ ty: TokenType::RCurl, line: self.line, line_pos: self.line_pos },
            '[' => Token{ ty: TokenType::LBrac, line: self.line, line_pos: self.line_pos },
            ']' => Token{ ty: TokenType::RBrac, line: self.line, line_pos: self.line_pos },
            ',' => Token{ ty: TokenType::Comma, line: self.line, line_pos: self.line_pos },
            ';' => Token{ ty: TokenType::Semi, line: self.line, line_pos: self.line_pos },
            c if c.is_whitespace() => {
              if *c == '\n' {
                self.line += 1;
                self.line_pos = 0;
                Token{ ty: TokenType::Newline, line: self.line, line_pos: self.line_pos }
              } else {
                Token{ ty: TokenType::Whitespace, line: self.line, line_pos: self.line_pos }
              }
            },
            c if c.is_numeric() => self.lex_numbers(iter, *c),
            op if OPEARTORS.contains(&&*op.to_string()) => self.lex_operators(iter, *op),

            _ => Token{ ty: TokenType::Unknown, line: self.line, line_pos: self.line_pos },
          }
        }
      })
    }
    tokens
  }

  fn lex_operators(&mut self, iter: &mut Peekable<Iter<char>>, first: char) -> Token {
    let mut op = String::from(first);

    while iter.peek() != None && OPEARTORS.contains(&&*iter.peek().unwrap().to_string()) {
      op.push(*iter.next().unwrap());
      self.line_pos += 1;
      self.pos += 1;
    }

    if ! OPEARTORS.contains(&&*op.to_string()) {
      panic!("")
    } else {
      Token{ ty: TokenType::new(op), line: self.line, line_pos: self.line_pos }
    }
  }

  fn lex_numbers(&mut self, iter: &mut Peekable<Iter<char>>, first: char) -> Token {
    let mut num = String::from(first);

    while iter.peek() != None && (iter.peek().unwrap().is_numeric() || iter.peek().unwrap() == &&'.') {
      num.push(*iter.next().unwrap());
      self.line_pos += 1;
      self.pos += 1
    }

    if num.matches('.').count() > 1 {
      panic!("")
    }

    if num.contains('.') {
      Token{ ty: TokenType::Decimal(num.parse::<f64>().unwrap()), line: self.line, line_pos: self.line_pos }
    }  else {
      Token{ ty: TokenType::Integer(num.parse::<i64>().unwrap()), line: self.line, line_pos: self.line_pos }
    }
  }

}