import unittest
from Funk.lexer import *

class TestLexer(unittest.TestCase):
  def test_white_space(self):
    lexer = Lexer(" ")
    lexer.lex()
    self.assertEqual(lexer.tokens[0].type, TokenType.WHITESPACE)

  def test_new_line(self):
    lexer = Lexer("""\n""")
    lexer.lex()
    self.assertEqual(lexer.tokens[0].type, TokenType.NEWLINE)

  def test_string(self):
    lexer = Lexer("\"Test\"")
    lexer.lex()
    self.assertEqual(lexer.tokens[0].type, TokenType.STRING)
  
  def test_number(self):
    lexer = Lexer("42")
    lexer.lex()
    self.assertEqual(lexer.tokens[0].type, TokenType.NUMBER)
  
  def test_decimal(self):
    lexer = Lexer("32.123")
    lexer.lex()
    self.assertEqual(lexer.tokens[0].type, TokenType.NUMBER)

  def test_operator(self):
    lexer = Lexer("+")
    lexer.lex()
    self.assertEqual(lexer.tokens[0].type, TokenType.OPERATOR)

  def test_keywords(self):
    lexer = Lexer("var")
    lexer.lex()
    self.assertEqual(lexer.tokens[0].type, TokenType.KEYWORD)

  def test_identifiers(self):
    lexer = Lexer("number_1_row_8")
    lexer.lex()
    self.assertEqual(lexer.tokens[0].type, TokenType.IDENTIFIER)


if __name__ == "__main__":
  unittest.main()