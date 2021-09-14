
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
  LPar = 3
  RPar = 4
  LCurl = 5
  RCurl = 6
  String = 7
  Newline = 8
  Semi = 9
  Comma = 10
  Variable = 11
  Keyword = 12
  Operator = 13
  Addition = 14
  Subtraction = 15
  Multiplication = 16
  Division = 17
  Add = 18
  Remove = 19

class Token:
  def __init__(self, typ: TokenType, value, position, line):
    self.type, self.value, self.position, self.line = typ, value, position, line
  def __str__(self):
    return f"{{ 'type': 'Token<{self.type}>', 'contents': {{ 'value': '{self.value}', 'line': {self.line}, 'pos': {self.position} }} }}"
  def smallStr(self):
    return f"{{{self.type} {self.value}}}"
  __repr__ = __str__

class Assignment:
  def __init__(self, typ: TokenType, variable, value):
    self.type, self.variable, self.value = typ, variable, value

  def __str__(self):
    return f"{{ 'type': 'Assignment', 'type': '{self.type}', 'variable': '{self.variable}', 'value': '{self.value}' }}"

  __repr__ = __str__

class BinaryOperator:
  def __init__(self, operator: TokenType, left, right):
    self.operator, self.left, self.right = operator, left, right

  def __str__(self):
    return f"{{ 'type': 'BinaryOperator', 'operator': '{self.operator}', 'left': '{self.left}', 'right': '{self.right}' }}"

  __repr__ = __str__

class UnaryOperator:
  def __init__(self, operator: TokenType, value):
    self.operator, self.value = operator, value

  def __str__(self):
    return f"{{ 'type': 'UnaryOperator', 'operator': '{self.operator}', 'value': '{self.value}' }}"

  __repr__ = __str__