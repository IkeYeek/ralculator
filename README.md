Was too bored to make music tonight so here is a simple calculator using rust to parse simple expressions because I want to learn rust lmao

# Disclaimer
It is late, I don't know rust very well and I have not written a single parser/compiler/interpreter/actually somehow a little theorical thing in like two years so the code in this repo will probably be terrfying...

# Syntax
1+1 => 2

2*4 => 8

(1+2) * 4 => 12

a = 3 => a=4

a * 2 => 8
b * 2 => b is not defined

```text
note that it has context. If I do for example in interactive mode:
a = 4 [enter] => 4
b = a * 2 [enter] => 8
b [enter] => 8
a = 2 [enter] => 2
b [enter] => 4

```

# Operators
OPS:
- Binary operators:
  - PLUS (+)
  - MINUS (-)
  - ASSIGN (=)
  - TIMES (*)
  - DIVIDE (/)
- Unary operators
  - UNARY_PLUS (+)
  - UNARY_MINUS (-)

# Separators
- (
- )

# Identifiers
simple sequences of letters matching this simple regex: `^[a-zA-Z_]+$`

# Litterals
numbers: integers, decimal or scientific notation, regex: `^\d+([.]\d+)?(e[+-]?\d+)?`

# Steps:
- Scanning / parsing
- Interpreting

# lexer
Types of token:
- Operator
- Identifier
- Litteral
- Separator

# parser
As my grammar is simple and I managed to avoid left recursion and ambiguous rules, I will implement some simple Recursive Descent Parser.
## grammar
### currently only assignments to litteral values is supported
```
<Line> ::= <Assignment> | <Expression>

<Assignment> ::= <Identifier> "=" <Expression> 

<Expression> ::= <Term> <ExpressionPrime>
<ExpressionPrime> ::= ("+" <Term> <ExpressionPrime> | "-" <Term> <ExpressionPrime> |  E)

<Term> ::= <Factor> <TermPrime>
<TermPrime> ::= ("*" <Factor> <TermPrime> | "/" <Factor> <TermPrime> |  E)

<Factor> ::= ("+" | "-") <Factor> 
			|	<Literal> 
			|	<Identifier> 
			|	"(" <Expression> ")" 

<Literal> ::= ([0-9])+
<Identifier> ::= ([a-z] | [A-Z] | "_")+
```