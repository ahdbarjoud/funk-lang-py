import unittest
from Funk.lexer import Lexer
from Funk.utils.tokens import *

class TestLexer(unittest.TestCase):
  def test_white_space(self):
    lexer = Lexer(" ")
    lexer.lex()
    self.assertEqual(lexer.tokens[0].type, TokenType.WHITESPACE)

unittest.main()