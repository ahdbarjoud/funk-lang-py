import sys
from Funk.lexer import Lexer

if len(sys.argv) < 2:
  # TODO: Open REPL
  print("Open REPL")
else:
  # TODO: Read the file mentioned
  code_file_path = sys.argv[1]
  
  with open(code_file_path, 'r') as f:
    source_code = f.read()

  lexer = Lexer(source_code)
  lexer.lex()
  
  print(lexer.tokens)
