import unittest
from Funk.lexer import Lexer
from Funk.utils.classes import *

class LexerTests(unittest.TestCase):
  def test_string_double(self):
    code = '"fuck"'

    lex = Lexer(code)
    lex.parse()

    self.assertEqual([i.type for i in lex.tokens], [TokenType.String])

  def test_string_single(self):
    code = "'fuck'"

    lex = Lexer(code)
    lex.parse()

    self.assertEqual([i.type for i in lex.tokens], [TokenType.String])

  def test_num(self):
    code = "5"

    lex = Lexer(code)
    lex.parse()

    self.assertEqual([i.type for i in lex.tokens], [TokenType.Num])

  def test_num_var(self):
    code = "a = 5"

    lex = Lexer(code)
    lex.parse()

    self.assertEqual([i.type for i in lex.tokens], [TokenType.Variable, TokenType.Operator, TokenType.Num])

  def test_string_var(self):
    code = "a = 'Fuck'"

    lex = Lexer(code)
    lex.parse()

    self.assertEqual([i.type for i in lex.tokens], [TokenType.Variable, TokenType.Operator, TokenType.String])

  def test_keyword_println(self):
    code = "println('test')"

    lex = Lexer(code)
    lex.parse()

    self.assertEqual([i.type for i in lex.tokens], [TokenType.Keyword, TokenType.LPar, TokenType.String, TokenType.RPar])

  def test_funk_def(self):
    code = """funk test(a, b) {
      1 + 2
    }"""

    lex = Lexer(code)
    lex.parse()

    self.assertEqual([i.type for i in lex.tokens], [TokenType.Keyword, TokenType.Variable, TokenType.LPar,
      TokenType.Variable, TokenType.Comma, TokenType.Variable, TokenType.RPar, TokenType.LCurl, TokenType.Newline,
      TokenType.Num, TokenType.Operator, TokenType.Num, TokenType.Newline, TokenType.RCurl])

  def test_funk_call(self):
    code = "test('test', 1, 3)"

    lex = Lexer(code)
    lex.parse()

    self.assertEqual([i.type for i in lex.tokens], [TokenType.Variable, TokenType.LPar, TokenType.String, TokenType.Comma,
      TokenType.Num, TokenType.Comma, TokenType.Num, TokenType.RPar])

  def test_conditional(self):
    code = """if (1 + 1 == 2 + 0) {
      println("Hi")
    }"""

    lex = Lexer(code)
    lex.parse()

    self.assertEqual([i.type for i in lex.tokens], [TokenType.Keyword, TokenType.LPar, TokenType.Num, TokenType.Operator,
      TokenType.Num, TokenType.Operator, TokenType.Num, TokenType.Operator, TokenType.Num, TokenType.RPar, TokenType.LCurl, 
      TokenType.Newline, TokenType.Keyword, TokenType.LPar, TokenType.String, TokenType.RPar, TokenType.Newline, TokenType.RCurl])

class ParserTests(unittest.TestCase):
  def test_conditional(self):
    code = """
    """

unittest.main()
