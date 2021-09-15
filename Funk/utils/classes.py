
from enum import Enum

KEYWORDS = [
  'println', 'funk', 'if', 'else', 
  'elif', 'forloop', 'while', 'each',
  'true', 'false', 'import', 'null'
]

OPS = [
  '+', '-', '/', '*', '=', 
  '==', '!=', '>', '<', '>=',
  '<=', ':=', '&&', '||', '&',
  '|', '++', '--', '.', '%', '^'
]

class TokenType(Enum):
  Num = 0
  Dec = 1
  LPar = 2
  RPar = 3
  LCurl = 4
  RCurl = 5
  String = 6
  Newline = 7
  Semi = 8
  Comma = 9
  Variable = 10
  Keyword = 11
  Operator = 12

class Token:
  def __init__(self, typ: TokenType, value, position, line):
    self.type, self.value, self.position, self.line = typ, value, position, line
  def __str__(self):
    return f"{{ 'type': 'Token<{self.type}>', 'contents': {{ 'value': '{self.value}', 'line': {self.line}, 'pos': {self.position} }} }}"
  def smallStr(self):
    return f"{{{self.type} {self.value}}}"
  __repr__ = __str__

class Assignment:
  def __init__(self, typ, variable, value):
    self.type, self.variable, self.value = typ, variable, value

  def __str__(self):
    return f"{self.variable} = {self.value}"

  __repr__ = __str__

class BinaryOperator:
  def __init__(self, operator, left, right):
    self.operator, self.left, self.right = operator, left, right

  def __str__(self):
    return f"{{ 'type': 'BinaryOperator', 'operator': '{self.operator}', 'left': '{self.left}', 'right': '{self.right}' }}"

  __repr__ = __str__

class UnaryOperator:
  def __init__(self, operator, value):
    self.operator, self.value = operator, value

  def __str__(self):
    return f"{{ 'type': 'UnaryOperator', 'operator': '{self.operator}', 'value': '{self.value}' }}"

  __repr__ = __str__

class Function:
  def __init__(self, name, params, body):
    self.name, self.params, self.body = name, params, body

  def __str__(self):
    return f"{{ 'type': 'Function', 'params': '{self.params}', 'body': '{self.body}' }}"

  __repr__ = __str__

class FunctionParam:
  def __init__(self, name, typ, default):
    self.name, self.type, self.default = name, typ, default

  def __str__(self):
    return f"{{ 'type': '{self.type}', 'name': '{self.name}', 'default': '{self.default}' }}"

  __repr__ = __str__
