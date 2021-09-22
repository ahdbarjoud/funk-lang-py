from os import error
from .utils.classes import *

class Parser:
  def __init__(self, tokens):
    self.tokens = tokens
    self.pos = 0
    self.current_token = self.tokens[self.pos]
    self.next_token = None
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
    current = self.current_token
    if not item:
      return self.next()

    self.next()

    if isinstance(item, tuple):
      if not current.type in item and not current.value in item:
        if error:
          raise Exception(f"Expected {' or '.join(str(i) for i in item)} on line: {current.line+1}.")
      else:
        return current

    if current.type != item and current.value != item:
      if error:
        raise Exception(f"Expected {item} on line: {current.line+1}.")

    return current

  def parse(self):
    while self.current_token != None:
      exp = self.parse_top()
      if exp:
        self.program.append(exp)
      self.next()

  def parse_top(self):
    if self.current_token.type != TokenType.Keyword:
      exp = self.parse_expr()
      return exp
    else:
      if self.current_token.value == "funk":
        self.expect(TokenType.Keyword)
        return self.parse_function()

      elif self.current_token.value == "println":
        name = self.current_token.value
        self.expect('println')
        return self.parse_call(name)

      elif self.current_token.value == "if":
        cur = self.current_token
        return self.parse_conditional(cur)

  def skip_newlines(self):
    while self.current_token != None and self.current_token.type == TokenType.Newline:
      self.next()

  def parse_conditional(self, typ):
    self.expect(typ.value)

    if typ.value == 'else':
      return Condition(typ.value, None, self.parse_funk_body(), None)

    self.expect(TokenType.LPar)
    exp = self.parse_expr()
    self.expect(TokenType.RPar)
    body = self.parse_funk_body()
    self.skip_newlines()
    other = None

    if self.current_token and self.current_token.type == TokenType.Keyword and self.current_token.value in ("elif", "else"):
      other = self.parse_conditional(self.current_token)

    return Condition(typ.value, exp, body, other)

  def parse_expr(self):
    result = self.parse_term()

    while self.current_token != None and self.current_token.type == TokenType.Operator and self.current_token.value in ("+", "-", "==", "!=", ">", "<", ">=", "<="):
      op = self.current_token.value
      self.expect((TokenType.Operator))
      result = BinaryOperator(op, result, self.parse_expr())

    return result

  def parse_term(self):
    result = self.parse_factor()

    while self.current_token != None and self.current_token.type == TokenType.Operator and self.current_token.value in ("*", "/"):
      op = self.current_token.value
      self.expect((TokenType.Operator))
      result = BinaryOperator(op, result, self.parse_factor())

    return result

  def parse_factor(self):
    if self.current_token.type == TokenType.LPar:
      self.expect(TokenType.LPar)
      result = self.parse_expr()
      self.expect(TokenType.RPar)
      return result

    elif self.current_token.type == TokenType.Num:
      n = self.current_token
      self.expect(TokenType.Num)
      return n

    elif self.current_token.type == TokenType.String:
      s = self.current_token
      self.expect(TokenType.String)
      return s

    elif self.current_token.type == TokenType.Variable:
      var = self.current_token
      self.expect(TokenType.Variable)

      if self.current_token.value == '=':
        return self.parse_assignment()

      elif self.current_token.type == TokenType.LPar:
        return self.parse_call(var.value)

      return var

  def parse_function(self):
    funk_name = self.expect(TokenType.Variable)
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
    params = []

    while self.current_token != None:
      if self.current_token.type == TokenType.RPar:
        self.expect(TokenType.RPar)
        break
      elif self.current_token.type == TokenType.LCurl:
        break

      elif self.current_token.type == TokenType.Variable:
        cur = self.expect(TokenType.Variable)
        self.expect((TokenType.Comma, TokenType.RPar))
        params.append(FunctionParam(cur.value, cur.type, None))
  
    return params

  def parse_funk_args(self):
    self.expect(TokenType.LPar)
    args = []

    while self.current_token != None:
      if self.current_token.type == TokenType.RPar:
        self.expect(TokenType.RPar)
        break

      if self.current_token.type in (TokenType.Newline, TokenType.Comma):
        self.expect((TokenType.Newline, TokenType.Comma), error=False)
        continue

      args.append(self.parse_top())

    return args

  def parse_call(self, name):
    args = self.parse_funk_args()
    return CallExp(name, args)

  def parse_assignment(self):
    left = self.tokens[self.pos-1]
    if left.type != TokenType.Variable:
      raise Exception("Syntax Error: Can only assign to variable.")

    while self.current_token != None and self.current_token.type != TokenType.Newline:
      self.next()
      return Assignment(TokenType.Variable, left.value, self.parse_expr())
