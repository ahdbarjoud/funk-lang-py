from Funk.lexer import Lexer
from Funk.parser import Parser
from Funk.interpreter import Interpreter
path = './test.fk'

code = open(path).read()

pos = 0

lex = Lexer(code)
lex.parse()
# for i in lex.tokens:
#   print(i)
#   print()
parser = Parser(lex.tokens)
parser.parse()
inter = Interpreter(parser.program)
inter.eval()

# for i in lex.tokens:
  # print(i)
