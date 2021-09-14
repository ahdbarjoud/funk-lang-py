from Funk.lexer import Lexer
from Funk.parser import Parser

path = './test.fk'

code = open(path).read()

pos = 0

lex = Lexer(code)
lex.parse()
parser = Parser(lex.tokens)
parser.parse()

# for i in lex.tokens:
  # print(i)

for i in parser.program:
  print(i)
