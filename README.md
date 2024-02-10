Was too borer to make music tonight so here is a simple calculator using rust to parse simple expressions because I want to learn rust lmao

# Disclaimer
It is late, I don't know rust very well and I have not written a single parser/compiler/interpreter/actually somehow a little theorical thing in like two years so the code in this repo will probably be terrfying...

# Syntax
1+1 => 2

2*4 => 8

(1+2) * 4 => 12

a = 3 => a=4

a * 2 => 8
b * 2 => b is not defined

# Symbols
OPS:
- BINOPS:
  - PLUS (+)
  - MINUS (-)
  - ASSIGN (=)
  - TIMES (*)
  - DIVIDE (/)
  - POW (^)
- UNOPS
  - UNARY_PLUS (+)
  - UNARY_MINUS (-)
- LEFT_PARENTHESIS
- RIGHT_PARENTHESIS

# Identifiers
simple sequences of letters matching this simple regex: `^[a-zA-Z_]+$`

# Litterals
simple numbers, integers for now

# Steps:
- Scanning / parsing
- Interpreting

# Scanning / parsing
Types of token:
- Symbol
- Identifier
- Litteral