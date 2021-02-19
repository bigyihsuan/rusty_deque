# Architecture

This is the `ARCHITECTURE.md` file for describing the general structure of the program.

The grammar of this language is as follows in this EBNF:

```
Code = Instr*
Inter = '!' Op | Op '!'
Op = Literal | Instruction | Block
Block = '{' Code '}'
Literal = Int | Float | Bool | Char | String | List
Int = sone int
Float = some float
Bool = "true" | "false"
Char = "'" c "'"
String = '"' characters '"'
List = '[' (Literal ',')* ']'
```

# Lexer

The lexer converts an input code string and converts it into a list of tokens.

Contains `TokenType`, `LiteralType`, `Token`, and `tokenize()` for use.

`tokenize()` takes in a string and outputs a list of tokens found in that string.

# Parser

The parser takes the list of tokens from the lexer and parses it into a abstract syntax tree.

The parser, in addition, validates the AST for evaluation.

# Evaluater

The evaluator takes the AST from the parser and runs it.