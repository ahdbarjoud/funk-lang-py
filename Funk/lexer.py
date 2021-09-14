from .utils.classes import *

class Lexer:
  def __init__(self, code):
    self.code = code
    self.pos = 0
    self.line = 0
    self.current_char = self.code[self.pos]
    self.next_char = self.code[self.pos+1]
    self.code_length = len(code) - 1
    self.tokens = []

  def next(self):
    # We increment.
    self.pos += 1
    if self.pos > self.code_length: # We no go too far.
      self.current_char = '\0'
      self.next_char = '\0'
      return

    self.current_char = self.code[self.pos] # We set current char.

    if self.pos+1 > self.code_length:
      self.next_char = '\0'
    else:
      self.next_char = self.code[self.pos + 1] # We set next char.

  def parse(self):
    while self.current_char != "\0":
      if self.current_char.isspace():
        if self.current_char == "\n":
          self.line += 1
          self.tokens.append(Token(TokenType.Newline, self.current_char, self.pos, self.line))
        self.next()

      elif self.current_char == "(":
        self.tokens.append(Token(TokenType.LPar, "(", self.pos, self.line))
        self.next()

      elif self.current_char == ")":
        self.tokens.append(Token(TokenType.RPar, ")", self.pos, self.line))
        self.next()

      elif self.current_char == "{":
        self.tokens.append(Token(TokenType.LCurl, "{", self.pos, self.line))
        self.next()

      elif self.current_char == "}":
        self.tokens.append(Token(TokenType.RCurl, "}", self.pos, self.line))
        self.next()

      elif self.current_char == ";":
        self.tokens.append(Token(TokenType.Semi, ";", self.pos, self.line))
        self.next()

      elif self.current_char == ",":
        self.tokens.append(Token(TokenType.Comma, ",", self.pos, self.line))
        self.next()

      elif self.current_char in ('"', "'"):
        self.parse_string(self.current_char)

      elif self.current_char.isalpha() or self.current_char == "_":
        self.parse_identifier()

      elif self.current_char in OPS:
        self.parse_operators()

      elif self.current_char != '\0' and self.current_char.isdigit():
        self.parse_number()

      else:
        raise Exception(
          'Invalid character on {0}:{1}\n  {2}\n  {3}'.format(
          self.line, self.line, str(self.code).split('\n')[self.line - 1], f"{' ' * (self.line - 1)}^")
        )

  def parse_string(self, char):
    string = ''
    start_pos = self.pos
    end = False

    while self.current_char != "\0":
      self.next()

      if self.current_char == char:
        self.tokens.append(Token(TokenType.String, string, start_pos, self.line))
        self.next()
        break

      if self.current_char == "\\":
        if self.next_char == "n":
          string += "\n"
        else: 
          string += self.next_char
        continue

      else:
        string += self.current_char

    if self.current_char == '\0' and not end:
      raise Exception('A string was not properly enclosed at {0}:{1}\n  {2}\n  {3}'.format(
        self.line, start_pos, str(self.code).split('\n')[self.line - 1], f"{' ' * (start_pos - 1)}^")
      )

  def parse_identifier(self):
    start_pos = self.pos
    name = ""

    while self.current_char != "\0" and (self.current_char.isalpha() or self.current_char == "_"):
      name += self.current_char
      self.next()

    if name in KEYWORDS:
      self.tokens.append(Token(TokenType.Keyword, name, start_pos, self.line))
    else:
      self.tokens.append(Token(TokenType.Variable, name, start_pos, self.line))

  def parse_operators(self):
    start_pos = self.pos
    operator = ""

    while self.current_char != "\0" and self.current_char in OPS:
      operator += self.current_char
      self.next()

    if not operator in OPS:
      raise Exception('Invalid operator on {0}:{1}\n  {2}\n  {3}'.format(
        self.line, start_pos, str(self.code).split('\n')[self.line - 1], f"{' ' * (start_pos - 1)}^")
      )

    if operator == "+":
      operator = TokenType.Addition
    elif operator == "-":
      operator = TokenType.Subtraction
    elif operator == "*":
      operator = TokenType.Multiplication
    elif operator == "/":
      operator = TokenType.Division
    elif operator == "++":
      operator = TokenType.Division
    elif operator == "--":
      operator = TokenType.Division

    self.tokens.append(Token(TokenType.Operator, operator, start_pos, self.line))

  def parse_number(self):
    start_pos = self.pos
    num = ""

    while self.current_char != '\0' and not self.current_char.isspace() and self.current_char.isdigit():
      num += self.current_char
      self.next()

    self.tokens.append(Token(TokenType.Num, int(num), start_pos, self.line))