from .utils.classes import *

class Parser:
  def __init__(self, tokens):
    self.tokens = tokens
    self.pos = 0
    self.current_token = self.tokens[self.pos]
    self.next_token = self.tokens[self.pos+1]
    self.tokens_length = len(self.tokens) - 1
    self.program = []

  def next(self):
    # We increment.
    self.pos += 1
    if self.pos > self.tokens_length: # We no go too far.
      self.current_token = None
      self.next_token = None
      return

    self.current_token = self.tokens[self.pos] # We set current token.

    if self.pos+1 > self.tokens_length:
      self.next_token = None
    else:
      self.next_token = self.tokens[self.pos + 1] # We set next token.

  def parse(self):
    while self.current_token != None:
      if self.current_token.type != TokenType.Keyword:

        self.program.append(self.parse_expr())

      self.next()

  def parse_expr(self):
    result = self.parse_term()

    while self.current_token != None and self.current_token.type == TokenType.Operator and self.current_token.value in (TokenType.Addition, TokenType.Subtraction):
      op = self.current_token.value
      self.next()
      result = BinaryOperator(op, result, self.parse_expr())

    return result

  def parse_term(self):
    result = self.parse_factor()
    while self.current_token != None and self.current_token.type == TokenType.Operator and self.current_token.value in (TokenType.Multiplication, TokenType.Division):
      op = self.current_token.value
      self.next()
      result = BinaryOperator(op, result, self.parse_factor())

    return result

  def parse_factor(self):
    fac = self.current_token
    if self.current_token.type == TokenType.LPar:
      self.next()
      result = self.parse_expr()
      if self.current_token is None or self.current_token.type != TokenType.RPar:
        raise Exception(f"Missing closing parenthesis.")
      self.next()
      return result

    elif self.current_token.type == TokenType.Num:
      n = self.current_token
      self.next()
      return n

    elif self.current_token.type == TokenType.Operator and self.current_token.value in ("++","--"):
      n = self.current_token
      self.next()
      return UnaryOperator(n, self.parse_factor())
