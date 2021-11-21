use std::slice::Iter;
use std::iter::Peekable;
use std::ops::Range;
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
            '(' => Token{ ty: TokenType::LPar, line: self.line, range: Range{ start: self.pos, end: self.pos} },
            ')' => Token{ ty: TokenType::RPar, line: self.line, range: Range{ start: self.pos, end: self.pos} },
            '{' => Token{ ty: TokenType::LCurl, line: self.line, range: Range{ start: self.pos, end: self.pos} },
            '}' => Token{ ty: TokenType::RCurl, line: self.line, range: Range{ start: self.pos, end: self.pos} },
            '[' => Token{ ty: TokenType::LBrac, line: self.line, range: Range{ start: self.pos, end: self.pos} },
            ']' => Token{ ty: TokenType::RBrac, line: self.line, range: Range{ start: self.pos, end: self.pos} },
            ',' => Token{ ty: TokenType::Comma, line: self.line, range: Range{ start: self.pos, end: self.pos} },
            ';' => Token{ ty: TokenType::Semi, line: self.line, range: Range{ start: self.pos, end: self.pos} },
            c if c.is_whitespace() => {
              if *c == '\n' {
                self.line += 1;
                self.line_pos = 0;
                Token{ ty: TokenType::Newline, line: self.line, range: Range{ start: self.pos, end: self.pos} }
              } else {
                Token{ ty: TokenType::Whitespace, line: self.line, range: Range{ start: self.pos, end: self.pos} }
              }
            },
            c if c.is_numeric() => self.lex_numbers(iter, *c),
            op if OPEARTORS.contains(&&*op.to_string()) => self.lex_operators(iter, *op),
            string if string == &'\"' => self.lex_string(iter),

            _ => Token{ ty: TokenType::Unknown, line: self.line, range: Range{ start: self.pos, end: self.pos} },
          }
        }
      })
    }
    tokens
  }

  fn lex_string(&mut self, iter: &mut Peekable<Iter<char>>) -> Token {
    let mut string = String::new();
    let start_pos = self.pos;
    let start_line = self.line;
    let mut end = false;

    while iter.peek() != None {
      if iter.peek().unwrap() == &&'\"' { 
        iter.next();
        end = true; 
        break;
      }

      if iter.peek().unwrap() == &&'\\' {
        iter.next();
        self.pos += 1;
        self.line_pos += 1;

        if iter.peek().unwrap() == &&'n' {
            string.push('\\');
            string.push('n');
            iter.next();
            self.pos += 1;
            self.line += 1;
            self.line_pos = 0;
        } else {
            string.push(*iter.next().unwrap());
            self.pos += 1;
            self.line_pos += 1;
        }
      } else {
          string.push(*iter.next().unwrap());
          self.pos += 1;
          self.line_pos += 1;
      }
    }

    if iter.peek() == None && end == false {
      panic!("")
    }

    Token{ ty: TokenType::Literal(Val::String), line: start_line, range: Range{ start: start_pos, end: self.pos} }
  }

  fn lex_operators(&mut self, iter: &mut Peekable<Iter<char>>, first: char) -> Token {
    let mut op = String::from(first);
    let start_pos = self.pos;

    while iter.peek() != None && OPEARTORS.contains(&&*iter.peek().unwrap().to_string()) {
      op.push(*iter.next().unwrap());
      self.line_pos += 1;
      self.pos += 1;
    }

    if ! OPEARTORS.contains(&&*op.to_string()) {
      panic!("")
    } else {
      Token{ ty: TokenType::new(op), line: self.line, range: Range{ start: start_pos, end: self.pos} }
    }
  }

  fn lex_numbers(&mut self, iter: &mut Peekable<Iter<char>>, first: char) -> Token {
    let mut num = String::from(first);
    let start_pos = self.pos;

    while iter.peek() != None && (iter.peek().unwrap().is_numeric() || iter.peek().unwrap() == &&'.') {
      num.push(*iter.next().unwrap());
      self.line_pos += 1;
      self.pos += 1
    }

    if num.matches('.').count() > 1 {
      panic!("")
    }

    Token{ ty: TokenType::Literal(Val::Number), line: self.line, range: Range{ start: start_pos, end: self.pos} }
  }
}