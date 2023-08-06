from enum import Enum

class LocationInfo:
  def __init__(self, start_pos, end_pos, line, line_pos) -> None:
    self.start_pos = start_pos
    self.end_pos = end_pos
    self.line = line
    self.line_pos = line_pos

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
  def __init__(self, type, location: LocationInfo) -> None:
    self.type = type
    self.location = location