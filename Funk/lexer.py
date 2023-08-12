from .utils.tokens import *

class Lexer:
  def __init__(self, source_code):
    self.source_code: str = source_code
    self.current_char: str = None
    self.pos: int = -1
    self.line: int = 1
    self.line_pos: int = 0
    self.tokens: list[Token] = []

  def next_char(self):
    if not self.source_code:
      self.current_char = None
      return
    self.current_char, self.source_code = self.source_code[0], self.source_code[1:]
    self.pos += 1
    self.line_pos += 1

  def lex(self):
    self.next_char()
    while self.current_char:
      match self.current_char:
        case c if c.isspace():
          if c == "\n":
            self.tokens.append(Token(TokenType.NEWLINE, "\n"))
            self.line_pos = 0
            self.line += 1
            self.next_char()
            continue
          self.tokens.append(Token(TokenType.WHITESPACE, " "))
          self.next_char()
        case c if c.isnumeric():
          self.tokens.append(self.handle_numbers())
        case c if c == "\"":
          self.tokens.append(self.handle_strings())
        case c if c in OPERATORS:
          self.tokens.append(self.handle_operators())
        case c if c.startswith("_") or c.isalpha():
          self.tokens.append(self.handle_keywords_and_variables())
        case c if c == "(":
          self.tokens.append(Token(TokenType.LPar, self.current_char))
          self.next_char()
        case c if c == ")":
          self.tokens.append(Token(TokenType.RPar, self.current_char))
          self.next_char()
        case _:
          # TODO: Make an Exception called UnknownCharacter and raise it
          self.next_char()

  def handle_numbers(self) -> Token:
    value = ""
    while self.current_char and self.current_char.isnumeric() or self.current_char == ".":
      value += self.current_char
      self.next_char()

    return Token(TokenType.NUMBER, value)

  def handle_strings(self) -> Token:
    value = ""
    self.next_char()
    while self.current_char and self.current_char != "\"":
      value += self.current_char
      self.next_char()
    if self.current_char != "\"":
      raise Exception("String was never closed.")
    self.next_char()
    return Token(TokenType.STRING, value)

  def handle_operators(self) -> Token:
    value = ""
    while self.current_char and self.current_char in OPERATORS:
      value += self.current_char
      self.next_char()
    return Token(TokenType.OPERATOR, value)
  
  def handle_keywords_and_variables(self) -> Token:
    value = self.current_char
    self.next_char()
    while self.current_char and (self.current_char.isalnum() or self.current_char == "_"):
      value += self.current_char
      self.next_char()
    if value in KEYWORDS:
      return Token(TokenType.KEYWORD, value)
    return Token(TokenType.IDENTIFIER, value)