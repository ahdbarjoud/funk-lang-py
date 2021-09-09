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

# Lexer
tokens = []

while pos <= len(code) - 1:
  # If character is any type of bracket. We make a bracket token.
  if code[pos].isspace():
    if code[pos] == "\n":
      tokens.append({
        "type": "newline",
        "value": "newline",
        "position": pos
      })
    pos += 1

  elif code[pos] == "(":
    tokens.append({
      "type": "lpar",
      "value": "(",
      "position": pos,
    })
    pos += 1

  elif code[pos] == ")":
    tokens.append({
      "type": "rpar",
      "value": ")",
      "position": pos,
    })
    pos += 1

  elif code[pos] == "{":
    tokens.append({
      "type": "lcurl",
      "value": "{",
      "position": pos,
    })
    pos += 1

  elif code[pos] == "}":
    tokens.append({
      "type": "rcurl",
      "value": "}",
      "position": pos,
    })
    pos += 1

  elif code[pos] == ";":
    tokens.append({
      "type": "semi",
      "value": ";",
      "position": pos,
    })
    pos += 1

  elif code[pos] == ",":
    tokens.append({
      "type": "comma",
      "value": ",",
      "position": pos,
    })
    pos += 1

  elif code[pos] in ("'", '"'):
    start_pos = pos
    last = None
    first = code[pos]
    pos += 1
    string = ""
    while pos <= len(code) - 1:
      if code[pos] == "\\":
        pos+=1
        if code[pos] == 'n':
          string+="\n"
        else:
          string+=code[pos]
        pos+=1
        continue

      if first == code[pos]:
        last = code[pos]
        pos+=1
        break

      string += code[pos]
      pos+=1

    if not last:
      raise Exception("Invalid syntax. Missing a quote.")

    tokens.append({
      "type": "string",
      "value": string,
      "position": start_pos
    })

  elif code[pos].isalpha() or code[pos] in ('_'):
    start_pos = pos
    var = ""
    while code[pos] is not None and (code[pos].isalnum() or code[pos] == "_"):
      var += code[pos]
      pos += 1

    if var in KEYWORDS:
      tokens.append({
        "type": "keyword",
        "value": var,
        "position": start_pos
      })
      continue
    tokens.append({
        "type": "variable",
        "value": var,
        "position": start_pos
      })

  elif code[pos] in OPS:
    start_pos = pos
    operator = ""
    while code[pos] is not None and code[pos] in OPS:
      operator += code[pos]
      pos += 1

    if operator in OPS:
      tokens.append({
        "type": "operator",
        "value": operator,
        "position": start_pos,
      })

  elif code[pos].isdigit():
    start_pos = pos
    num = ""
    while code[pos] is not None and not code[pos].isspace() and (code[pos].isdigit()):
      num += code[pos]
      pos += 1
    tokens.append({
      "type": "num",
      "value": int(num),
      "position": start_pos,
    })
    pos += 1

# Parser
pos = 0
program = []

while pos <= len(tokens) - 1:
  if tokens[pos]["type"] == "keyword":
    if tokens[pos]["value"] == "println":
      to_print = []
      lpar = None
      rpar = None
      semi = None
      pos += 1
      while pos <= len(tokens) - 1:
        if tokens[pos]["type"] == "lpar":
          lpar = tokens[pos]
          pos += 1
          continue
        if tokens[pos]["type"] == "rpar":
          rpar = tokens[pos]
          pos += 1
          continue
        if tokens[pos]["type"] == "semi":
          semi = tokens[pos]

        if tokens[pos]["type"] in ("newline", "semi", "lcurl", "rcurl"):
          if tokens[pos]["type"] == "semi" and (not rpar or tokens[pos-1]["position"] != rpar["position"]):
            raise Exception("Invalid Syntax")
          break

        to_print.append(tokens[pos]["value"])
        pos += 1

      if not lpar:
        raise Exception("Invalid syntax.")
      if not rpar:
        raise Exception("Invalid syntax.")
      if not semi:
        raise Exception("Invalid syntax.")

      for i in to_print:
        print(i)

  if tokens[pos]["type"] == "operator":
    if tokens[pos]["value"] == "=":
      left = tokens[pos-1]
      pos+= 1
      right = tokens[pos]

      if left["type"] != "variable" or not right["type"] in ("string", "num", "variable"):
        raise Exception("Invalid syntax.")

      program.append({
        "type": "assignment",
        "operator": "=",
        "left": left,
        "right": right
      })

  pos+=1

print(program)

# TODO reposition complicated ASTs below the lexer.
