# Architecture

This is the `ARCHITECTURE.md` file for describing the general structure of the program.

# Lexer

The lexer converts an input code string and converts it into a list of tokens.

Contains `TokenType`, `LiteralType`, `Token`, and `tokenize()` for use.

`tokenize()` takes in a string and outputs a list of tokens found in that string.

# Parser

The parser takes the list of tokens from the lexer and parses it into a abstract syntax tree.

# Evaluater

The evaluator takes the AST from the parser and runs it.