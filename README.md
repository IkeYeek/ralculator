# 🦀 <sup>R</sup><sub>(ust)</sub><sup>alculator</sup> 🧮


| cargo test                                                                                                                                                    | cargo clippy -- -Wclippy::pedantic -Wclippy::style                                                                                                                  |
|---------------------------------------------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| [![cargo test](https://github.com/IkeYeek/ralculator/actions/workflows/test.yml/badge.svg)](https://github.com/IkeYeek/ralculator/actions/workflows/test.yml) | [![cargo clippy](https://github.com/IkeYeek/ralculator/actions/workflows/clippy.yml/badge.svg)](https://github.com/IkeYeek/ralculator/actions/workflows/clippy.yml) |
                                                                                                                                                      




Welcome to the <sup>R</sup><sub>(ust)</sub><sup>alculator</sup> repository! 🎉 This is a simple mathematical expression interpreter (and maybe later on compiler???👀) application written in Rust, created as a fun side project to explore the language and its features. The calculator can parse and evaluate basic arithmetic expressions, as well as store expression in variables and is designed to be easy to understand and extend.
## 🚧 TODO
- fix circle reference bug. atm, if I do:
  - a = 3
  - b = a
  - a = a

the program crashes.

## 📝 Table of Contents
- [Getting Started](#-getting-started)
  - [Prerequisites](#-Prerequisites)
  - [Features](#-features)
  - [Usage](#-usage)
- [Syntax](#-syntax)
  - [Operators](#-operators)
  - [Separators](#-separators)
  - [Identifiers](#-identifiers)
  - [Literals](#-literals)
- [Steps](#-steps)
  - [Lexer](#-lexer)
  - [Parser](#-parser)
  - [Interpreter](#-interpreter)
- [Grammar](#-grammar)
- [Contributing](#-contributing)
- [License](#-license)

## 🚀 Getting Started

To get started with the <sup>R</sup><sub>(ust)</sub><sup>alculator</sup>, clone the repository and follow the build instructions below.

### 📦 Prerequisites

Ensure you have Rust installed on your system. If not, you can download and install it from [rust-lang.org](https://www.rust-lang.org/tools/install).

### 🛠️ Building

```sh
git clone https://github.com/yourusername/rusty-calculator.git
cd rusty-calculator
cargo build --release
```

## 🌟 Features

- Simple arithmetic operations: addition, subtraction, multiplication, and division.
- Unary operations: positive and negative (yeah ikr unary positive operator is useless but that's still fun).
- Parentheses for grouping expressions.
- Variable assignment and usage. Variables are expression so if a variable relies on another one, and the one it relies on change, its expressed value will change as well if computed.
- Interactive mode for entering expressions one by one (REPL).

## 📖 Usage

The calculator can be used in interactive mode or by passing expressions as command-line arguments. For interactive mode, simply run the application without any arguments. To evaluate an expression directly, pass it as an argument:

```sh
cargo run --release -- -e "1 +  2 *  3"
```
for using it to eval this expression
```sh
cargo run --release -- -i
```
for using it in interactive mode

## 🔢 Syntax

Here are some examples of how expressions are evaluated:

- `1 +  1` => `2`
- `2 *  4` => `8`
- `(1 +  2) *  4` => `12`
- `a =  3` => `a =  3` (assigns the value `3` to variable `a`)
- `a *  2` => `6` (assumes `a` is already defined as `3`)
- `b *  2` => `Error: b is not defined` (since `b` is not defined)

## 🔄 Operators

- **Binary operators**: `+`, `-`, `=`, `*`, `/`
- **Unary operators**: `+`, `-`

## 🔤 Separators

- `(`
- `)`

## 🆔 Identifiers

Identifiers are sequences of letters, digits, and underscores, starting with a letter or underscore. They are used for variable assignment and reference.

## 🔢 Literals

Literals are numeric values, which can be integers or decimal numbers.

## 🚶‍♂️ Steps

The calculator operates in two main steps:

1. Scanning / parsing: The input expression is tokenized and parsed into an Abstract Syntax Tree (AST).
2. Interpreting: The AST is evaluated to produce the result.

## 📏 Lexer

The lexer breaks down the input string into tokens of different types:

- **Operator**: Represents an arithmetic operator.
- **Identifier**: Represents a variable name or an unquoted string.
- **Literal**: Represents a numeric value.
- **Separator**: Represents parentheses for grouping expressions.

## 📚 Parser

The parser constructs an Abstract Syntax Tree (AST) from the tokens produced by the lexer. It uses a Recursive Descent Parser to handle the precedence and associativity of the operators.

## 🧠 Interpreter

The interpreter walks through the AST and evaluates each expression node. It maintains a symbol table to keep track of variable assignments and their values.

## 📜 Grammar

The grammar of the calculator is defined as follows:

```
<Line> ::= <Assignment> | <Expression>

<Assignment> ::= <Identifier> "=" <Expression>  

<Expression> ::= <Term> <ExpressionPrime>
<ExpressionPrime> ::= ("+" <Term> <ExpressionPrime> | "-" <Term> <ExpressionPrime> |  E)

<Term> ::= <Factor> <TermPrime>
<TermPrime> ::= ("*" <Factor> <TermPrime> | "/" <Factor> <TermPrime> |  E)

<Factor> ::= ("+" | "-") <Factor>  
            | <Literal>  
            | <Identifier>  
            | "(" <Expression> ")"  

<Literal> ::= ([0-9])+
<Identifier> ::= ([a-z] | [A-Z] | "_")+
```

## 🤝 Contributing

Contributions are welcome! Feel free to submit pull requests or open issues for feature requests or bug reports.

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE.md) file for details.