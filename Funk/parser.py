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
        exp = self.parse_expr()
        if not exp:
          self.next()
          continue
        self.program.append(exp)
        print(type(exp))
        print()
      self.next()

  def parse_assignment(self):
    left = self.tokens[self.pos-1]
    if left.type != TokenType.Variable:
      raise Exception("Syntax Error: Can only assign to variable.")

    while self.current_token != None and self.current_token.type != TokenType.Newline:
      self.next()
      return Assignment(left, self.parse_expr())

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

    elif self.current_token.type == TokenType.Operator and self.current_token.value in (TokenType.Add, TokenType.Remove):
      n = self.current_token
      self.next()
      return UnaryOperator(n, self.parse_factor())

    elif self.current_token.type in (TokenType.Variable, TokenType.Equal):
      self.next()
      return self.parse_assignment()
