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

  def expect(self, item, error=True):
    if not item:
      return self.next()

    if isinstance(item, tuple):
      if not self.current_token.type in item and self.current_token.value in item:
        if error:
          raise Exception(f"Expected {''.join(i for i in item)} on line: {self.current_token.line+1}.")
      else:
        return self.current_token

    if self.current_token.type != item and self.current_token.value != item:
      if error:
        raise Exception(f"Expected {item} on line: {self.current_token.line+1}.")

    return self.current_token

  def parse(self):
    while self.current_token != None:
      a = self.parse_top()
      if a:
        self.program.append(a)

  def parse_top(self):
    if self.current_token.type != TokenType.Keyword:
      exp = self.parse_expr()
      self.next()
      return exp
    else:
      if self.current_token.value == "funk":
        self.next()
        return self.parse_function()

  def parse_function(self):
    funk_name = self.expect(TokenType.Variable)
    self.next()
    params = self.parse_funk_params()
    body = self.parse_funk_body()
    return Function(funk_name.value, params, body)

  def parse_funk_body(self):
    self.expect(TokenType.LCurl)
    self.next()
    body = []

    while self.current_token != None and self.current_token.type != TokenType.RCurl:
      body.append(self.parse_top())
      self.next()
    self.expect(TokenType.RCurl)
    return body

  def parse_funk_params(self):
    self.expect(TokenType.LPar)
    self.next()
    params = []

    while self.current_token != None:
      if self.current_token.type == TokenType.RPar:
        self.expect(TokenType.RPar)
        self.next()
        break
      elif self.current_token.type == TokenType.LCurl:
        break

      elif self.current_token.type == TokenType.Variable:
        cur = self.current_token
        self.next()
        self.expect((TokenType.Comma, TokenType.RPar))
        params.append(FunctionParam(cur.value, cur.type, None))
        self.next()
  
    return params

  def parse_assignment(self):
    left = self.tokens[self.pos-1]
    if left.type != TokenType.Variable:
      raise Exception("Syntax Error: Can only assign to variable.")

    while self.current_token != None and self.current_token.type != TokenType.Newline:
      self.next()
      return Assignment(TokenType.Variable, left.value, self.parse_expr())

  def parse_expr(self):
    result = self.parse_term()

    while self.current_token != None and self.current_token.type == TokenType.Operator and self.current_token.value in ("+", "-"):
      op = self.current_token.value
      self.next()
      result = BinaryOperator(op, result, self.parse_expr())

    return result

  def parse_term(self):
    result = self.parse_factor()
    while self.current_token != None and self.current_token.type == TokenType.Operator and self.current_token.value in ("*", "/"):
      op = self.current_token.value
      self.next()
      result = BinaryOperator(op, result, self.parse_factor())

    return result

  def parse_factor(self):
    fac = self.current_token
    if self.current_token.type == TokenType.LPar:
      self.next()
      result = self.parse_expr()

      self.expect(TokenType.RPar)

      self.next()
      return result

    elif self.current_token.type == TokenType.Num:
      n = self.current_token
      self.next()
      return n
    elif self.current_token.type == TokenType.String:
      n = self.current_token
      self.next()
      return n

    elif self.current_token.type == TokenType.Operator and self.current_token.value in ("++", "--"):
      n = self.current_token.value
      self.next()
      return UnaryOperator(n, self.parse_factor())

    elif self.current_token.type == TokenType.Variable:
      self.next()
      self.expect('=')
      return self.parse_assignment()
