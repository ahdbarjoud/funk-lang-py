from .utils.classes import *

class Interpreter:
  def __init__(self, AST):
    self.AST = AST
    self.ast_length = len(self.AST) - 1
    self.pos = 0
    self.current_ast = self.AST[self.pos]
    self.next_ast = self.AST[self.pos + 1]

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
      ev = self.eval_ast(self.current_ast)
      if ev:
        print(ev)
      self.next()

  def eval_ast(self, ast):
    if isinstance(ast, BinaryOperator) and ast.operator in ("+", "-", "*", "/"):
      left = self.eval_ast(ast.left)
      right = self.eval_ast(ast.right)

      if ast.operator == "+":
        return left + right
      if ast.operator == "-":
        return left - right
      if ast.operator == "*":
        return left * right
      if ast.operator == "/":
        return left / right

    if isinstance(ast, Token) and ast.type in (TokenType.Num, TokenType.String):
      return ast.value
