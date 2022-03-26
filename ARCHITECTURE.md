# Architecture

This is the `ARCHITECTURE.md` file for describing the general structure of the program.

The grammar of this language is as follows in this EBNF:

```sh
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
Char = "'" character "'"
String = '"' characters '"'
List = '[' (Literal ',')* ']'
```