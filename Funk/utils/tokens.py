from enum import Enum

OPERATORS = ["+", "-", "*", "/", "="]

KEYWORDS = ["if", "elif", "else", "funk", "while", "var"]


# TODO: Store a token's index range to fetch when needed from source code, instead of the actual value. This was broken, should revisit.
# class LocationInfo:
#   def __init__(self, start_pos, end_pos, line, line_pos) -> None:
#     self.start_pos = start_pos
#     self.end_pos = end_pos
#     self.line = line
#     self.line_pos = line_pos

#   def __repr__(self) -> str:
#     return f"<Start: {self.start_pos}, End: {self.end_pos}, Line: {self.line}, Line Position: {self.line_pos}>"

class TokenType(Enum):
  NUMBER = 0
  STRING = 1
  WHITESPACE = 2
  NEWLINE = 3
  OPERATOR = 4
  KEYWORD = 5
  IDENTIFIER = 6
  RPar = 7
  LPar = 8

class Token:
  def __init__(self, type, value: str) -> None:
    self.type = type
    self.value = value
  
  def __repr__(self) -> str:
    return f"<Type: {self.type}>, Value: {self.value}>"