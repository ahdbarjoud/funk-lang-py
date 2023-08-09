from utils.tokens import *

class Lexer:
  def __init__(self, source_code):
    self.source_code = source_code
    self.current_char: str = None
    self.pos = 0
    self.line = 1
    self.line_pos = 1
    self.tokens = []

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
          self.tokens.append(Token(TokenType.WHITESPACE, LocationInfo(self.pos, self.pos+1, self.line, self.line_pos)))
          self.next_char()
        case c if c == "\n":
          self.tokens.append(Token(TokenType.NEWLINE, LocationInfo(self.pos, self.pos+1, self.line, self.line_pos)))
          self.line_pos = 0
          self.line += 1
          self.next_char()
        case c if c.isnumeric():
          self.tokens.append(self.handle_numbers())
        case c if c == "\"":
          self.tokens.append(self.handle_strings())
        case _:
          # TODO: Make an Exception called UnknownCharacter and raise it
          self.next_char()

  def handle_numbers(self) -> Token:
    start_pos = self.pos
    while self.current_char and self.current_char.isnumeric() or self.current_char == ".":
      self.next_char()
    end_pos = self.pos + 1
    return Token(TokenType.NUMBER, LocationInfo(start_pos, end_pos, self.line, self.line_pos))

  def handle_strings(self) -> Token:
    start_pos = self.pos
    self.next_char()
    while self.current_char and self.current_char != "\"":
      self.next_char()
    if self.current_char != "\"":
      raise Exception("String was never closed.")
    end_pos = self.pos + 1
    self.next_char()
    return Token(TokenType.STRING, LocationInfo(start_pos, end_pos, self.line, self.line_pos))