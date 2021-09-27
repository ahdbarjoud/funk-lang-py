import enum
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
    self.vars = {'global': {

    }}
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

    elif isinstance(ast, classes.BinaryOperator) and ast.operator in ("+=", "-="):
      if not isinstance(ast.left, classes.Variable):
        raise Exception("Can only use increment operators on variablers.")
      if not isinstance(ast.right, classes.Integer):
        raise Exception("Can only use Nums with increment operators.")
      left = ast.left
      right = self.eval_ast(ast.right)

      if ast.operator == "+=":
        self.vars[left.scope][left.name] += right
      else:
        self.vars[left.scope][left.name] -= right

    elif isinstance(ast, classes.Integer):
      return ast.value

    elif isinstance(ast, classes.String):
      return ast.value

    elif isinstance(ast, classes.Variable):
      if not ast.name in self.vars[ast.scope]:
        raise Exception(f"Variable {ast.name} was never created.")
      return self.vars[ast.scope][ast.name]

    elif isinstance(ast, classes.Assignment):
      if ast.type == classes.TokenType.Variable:
        if not ast.variable.scope in self.vars:
          self.vars[ast.variable.scope] = {}
        self.vars[ast.variable.scope][ast.variable.name] = self.eval_ast(ast.value)

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

      for index, i in enumerate(ast.args):
        self.eval_ast(classes.Assignment(classes.TokenType.Variable, classes.Variable(self.funks[ast.name].params[index].name, scope=f'funk:{ast.name}'), i))
        # self.eval_ast(classes.Assignment(classes.TokenType.Variable, classes.Variable(i.name, f'funk:{ast.name}'), i.value))
      for i in self.funks[ast.name].body:
        self.eval_ast(i)

    elif isinstance(ast, classes.Condition):
      if self.eval_ast(ast.exp) or ast.exp is None:
        for i in ast.body:
          self.eval_ast(i)
      elif ast.other:
        self.eval_ast(ast.other)

    elif isinstance(ast, classes.WhileLoop):
      while self.eval_ast(ast.exp) or ast.exp is None:
        for i in ast.body:
          self.eval_ast(i)
      else:
        if ast.other:
          self.eval_ast(ast.other)
