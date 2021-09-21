from .utils import classes

evals = {
  "+": lambda a, b: 0 if a + b is None else a + b,
  "-": lambda a, b: 0 if a - b is None else a - b,
  "*": lambda a, b: 0 if a * b is None else a * b,
  "/": lambda a, b: 0 if a / b is None else a / b,
  "==": lambda a, b: a == b,
  "!=": lambda a, b: a != b,
  ">": lambda a, b: a > b,
  "<": lambda a, b: a < b,
  ">=": lambda a, b: a >= b,
  "<=": lambda a, b: a <= b
}

class Interpreter:
  def __init__(self, AST):
    self.AST = AST
    self.ast_length = len(self.AST) - 1
    self.pos = 0
    self.current_ast = self.AST[self.pos]
    self.next_ast = None
    self.vars = {}
    self.funks = {}

  def next(self):
    # We increment.
    self.pos += 1
    if self.pos > self.ast_length: # We no go too far.
      self.current_ast = None
      self.next_ast = None
      return

    self.current_ast = self.AST[self.pos] # We set current token.
    if self.pos+1 > self.ast_length:
      self.next_ast = None
    else:
      self.next_ast = self.AST[self.pos + 1] # We set next token.

  def eval(self):
    while self.current_ast != None:
      self.eval_ast(self.current_ast)
      self.next()

  def eval_ast(self, ast):
    if isinstance(ast, classes.BinaryOperator) and ast.operator in ("+", "-", "*", "/", "==", "!=", ">", "<", ">=", "<="):
      left = self.eval_ast(ast.left)
      right = self.eval_ast(ast.right)

      return evals[ast.operator](left, right)

    elif isinstance(ast, classes.Token) and ast.type in (classes.TokenType.Num, classes.TokenType.String):
      return ast.value

    elif isinstance(ast, classes.Token) and ast.type == classes.TokenType.Variable:
      if not ast.value in self.vars:
        raise Exception(f"Variable {ast.value} was never created.")
      return self.vars[ast.value]

    elif isinstance(ast, classes.Assignment):
      if ast.type == classes.TokenType.Variable:
        self.vars[ast.variable] = self.eval_ast(ast.value)

    elif isinstance(ast, classes.Function):
      self.funks[ast.name] = ast

    elif isinstance(ast, classes.CallExp):
      if ast.name in classes.KEYWORDS:
        if ast.name == "println":
          for i in ast.args:
            print(self.eval_ast(i))
          return

      elif not ast.name in self.funks.keys():
        raise Exception(f"Function {ast.name} is not defined anywhere.")

      elif len(ast.args) != len(self.funks[ast.name].params):
        raise Exception(f"Params required by function \"{ast.name}\" and arguments provided do not match.")

      for i in self.funks[ast.name].body:
        self.eval_ast(i)

    elif isinstance(ast, classes.Condition):
      if self.eval_ast(ast.exp) or ast.exp is None:
        for i in ast.body:
          self.eval_ast(i)
      elif ast.other:
        self.eval_ast(ast.other)
