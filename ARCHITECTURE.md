# Architecture

This is the `ARCHITECTURE.md` file for describing the general structure of the program.

The grammar of this language is as follows in this EBNF:

```
Code = Exec*
Exec = ExecLeft | ExecRight
ExecLeft = Op '!'
ExecRight = Op '~'
Op = Literal | Instruction
Instruction = some instruction
Literal = Int | Float | Bool | Char | String | List | Block
Block = '{' Code '}'
Int = sone int
Float = some float
Bool = true | false
Char = "'" c "'"
String = '"' characters '"'
List = '[' (Literal ',')* ']'
```

# Lexer
Contains 2 modules:

* `tok`: Contains definitions for tokens.
* `lex`: Converts `rusty_deque` source code into a list of tokens.

# Parser
Contains 2 modules:

* `ast`: Contains definitions for the AST of `rusty_deque`, based on a Visitor pattern.
* `par`: Parses a list of tokens into an AST.

# Evaluator
Contains 2 modules:

* `visit`: Defines the `Visitor` trait to traverse the AST.
* `tree_print`: a test visitor for debugging.