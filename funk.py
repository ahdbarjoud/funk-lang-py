class Token:
  def __init__(self, typ, value, position, line):
    self.type, self.value, self.position, self.line = type, value, position, line

class Assignment:
  def __init__(self, typ, variable, value):
    self.type, self.variable, self.value = typ, variable, value

  def __str__(self):
    return f"{{ 'type': 'Assignment', 'type': '{self.type}', 'variable': '{self.variable}', 'value': '{self.value}' }}"

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

path = './test.fk'

code = open(path).read()

pos = 0

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

# Don't ask who is "we"
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
          self.tokens.append({
            "type": "newline",
            "value": "newline",
            "position": pos,
            "line": self.line
          })
        self.next()

      elif self.current_char == "(":
        self.tokens.append({
          "type": "lpar",
          "value": "(",
          "position": self.pos,
          "line": self.line
        })
        self.next()

      elif self.current_char == ")":
        self.tokens.append({
          "type": "rpar",
          "value": ")",
          "position": self.pos,
          "line": self.line
        })
        self.next()

      elif self.current_char == "{":
        self.tokens.append({
          "type": "lcurl",
          "value": "{",
          "position": self.pos,
          "line": self.line
        })
        self.next()

      elif self.current_char == "}":
        self.tokens.append({
          "type": "rcurl",
          "value": "}",
          "position": self.pos,
          "line": self.line
        })
        self.next()

      elif self.current_char == ";":
        self.tokens.append({
          "type": "semi",
          "value": ";",
          "position": self.pos,
          "line": self.line
        })
        self.next()

      elif self.current_char == ",":
        self.tokens.append({
          "type": "comma",
          "value": ",",
          "position": self.pos,
          "line": self.line
        })
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
        self.tokens.append({
          "type": "string",
          "value": string,
          "position": start_pos,
          "line": self.line
        })
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
      self.tokens.append({
        "type": "keyword",
        "value": name,
        "position": start_pos,
        "line": self.line
      })
    else: 
      self.tokens.append({
        "type": "variable",
        "value": name,
        "position": start_pos,
        "line": self.line
      })

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

    self.tokens.append({
      "type": "operator",
      "value": operator,
      "position": start_pos,
      "line": self.line
    })

  def parse_number(self):
    start_pos = self.pos
    num = ""

    while self.current_char != '\0' and not self.current_char.isspace() and self.current_char.isdigit():
      num += self.current_char
      self.next()

    self.tokens.append({
      "type": "number",
      "value": int(num),
      "position": start_pos,
      "line": self.line
    })

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
      if self.current_token["type"] != "keyword":

        self.program.append(self.parse_expr())

      self.next()

  def parse_expr(self):
    result = self.parse_term()

    while self.current_token != None and self.current_token["type"] == "operator" and self.current_token["value"] in ("+", "-"):
      op = self.current_token["value"]
      self.next()
      result = BinaryOperator(op, result, self.parse_expr())

    return result\

  def parse_term(self):
    result = self.parse_factor()
    while self.current_token != None and self.current_token["type"] == "operator" and self.current_token["value"] in ("*", "/"):
      op = self.current_token["value"]
      self.next()
      result = BinaryOperator(op, result, self.parse_factor())

    return result

  def parse_factor(self):
    fac = self.current_token
    if self.current_token["type"] == "lpar":
      self.next()
      result = self.parse_expr()
      if self.current_token["type"] != "rpar":
        raise Exception(f"Missing closing parenthesis on line {self.current_token['line']}")
      self.next()
      return result

    elif self.current_token["type"] == "number":
      n = self.current_token
      self.next()
      return n

    elif self.current_token["type"] == "operator" and self.current_token["value"] in ("++","--"):
      n = self.current_token
      self.next()
      return UnaryOperator(n["value"], self.parse_factor())

lex = Lexer(code)
lex.parse()
parser = Parser(lex.tokens)
parser.parse()

# for i in lex.tokens:
  # print(i)

for i in parser.program:
  print(i)
