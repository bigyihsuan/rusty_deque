# rusty_deque

A simple, deque-based programming language to practice writing Rust.

### Goals

* Learn and practice Rust programming.
* Create a functional programming language using deques.
* Create a programming language that allows using code as a deque value.

# Deque

The main data structure is a [deque](https://en.wikipedia.org/wiki/Deque), which allows the insertion and removal of elements at both ends.

Inspired by [this Esolangs page](https://esolangs.org/wiki/Deque) and [this Concatenative page](https://concatenative.org/wiki/view/Deque).

# Examples

More examples in the [`examples`](./examples) directory.

### Hello World
```bash
"Hello World!"~ ol~
```

### Cat
```bash
{il~ ol~}~ loop~
```

### Factorial
```bash
# given an int n (n -- n!)
1~ +~ 1~ {*~}~ 1~ rot! 2~ range~
```

# Types

The language contains the following primitve types:

* Signed 64-bit Ints
* 64-bit Floats
* Booleans
* Characters

Floats contain a `.`, and floats `0 < f < 1` must start with `0.`. Booleans are `true` and `false`. Characters are surrounded by single quotes `'c'`.

The language has a single compund type: **a List**. Lists can be nested, and its elements can have different types. List elements are separated by commas.

## Truthiness
The five types have the following truthiness rules:

* Ints, floats, chars are truthy if non-zero, and falsy otherwise.
* Lists are truthy if they are non-empty, and falsy otherwise.
* Blocks are always truthy.

# Literals

Literals are pushed onto the deque as-is. The appending sigil denotes where it gets pushed to (see section **Appending Sigil** below).

```bash
123   # postive int
-321  # negative int
9.87  # float
'c'   # single character
true  # boolean, an instruction that pushes a `true`
false # boolean, an instruction that pushes a `false`
[1.2, 'a', [true, 3], -4] # nested list
[]    # empty list
```

## Character Lists

Character lists are a special case for lists. As a literal, these are identical:
```bash
['h','e','l','l','o']
"hello"
```
The latter, double-quote syntax is sugar for a `[Char]`.

# Blocks

Blocks are code treated as a literal. Code surrounded in curly brackets `{}` are pushed as a literal. You can nest blocks in blocks. Blocks can contain no code (a nop).

# Comments

Comments start with a hash mark `#` and end at the end of the line.

```bash
# this is a comment
1! 2~ +! # comment after some code
```

# Sigils `~` and `!`

All instructions and literals are either postpended by a sigil, which differs based on which side of the deque the operation works on.
A bang `!` means that the operation works on the left of the deque.
A tilde `~` means that the operation works on the right of the deque.

## Sigil Semantics

You can get both stack-like and queue-like semantics through careful use of sigils:

| Semantic | Add | Remove | Notes                                                               |
| -------- | --- | ------ | ------------------------------------------------------------------- |
| Stack    | `x` | `x`    | As long as both sigils are the same, you have stack-like semantics. |
| Queue    | `~` | `!`    | For a queue that enqueues on the right and deques on the left.      |
| Queue    | `!` | `~`    | For a queue that enqueues on the left and deques on the right.      |

# Instructions

Instructions have the general philosophy of "discard if failed". If an instruction is called and its supplied parameters are of the incorrect domain, it will discard these values.

## Deque Operations
* (`push`: Handled by literals.)
* `clear`: Clear the deque.
* `pop`: Discard one element.
* `dup`: Duplicate one element.
* `swap`: Swap the front/back two elements.
* `rot` : Rotate the deque one element towards a direction (either to the front/left `rot~` or the back/right `rot!`).
* `over` : Duplicate the element below the top/bottom element.
* `len`: Push the length of the deque.

## Castings

These operations attempt to cast primitives to primitives. If the conversion fails, it discards the value.

* `toInt`: Pops 1 and pushes an int. Floats are truncated at the decimal point. Characters have their Rust `c as u64` value. Booleans are 1 or 0.
* `toFloat`: Pops 1 and pushes a float. Characters have their Rust `c as u64` value.
* `toChar`: Pops 1 and pushes a char. Uses Rust `std::char::to_char(v)`.
* `toBool`: Pops 1 and pushes a bool, based on the truthiness rules noted above.

## `Int`/`Float` Operations

All of these operate on ints and floats only. Non-ints and non-floats are discarded, with no errors.
It tries to do integer operations by default; if any arguments are a float, it pushes the result of a float operation.

* `+`: Sum.
* `-`: Difference.
* `*`: Product.
* `/`: Integer Division. Discards the arguments if the second element is 0.
* `//`: Float Division. Discards the arguments if the second element is 0.
* `%`: Remainder. Discards the arguments if the second element is 0.
* `exp`: `(a b -- a^b)` Pops 2 `a` and `b`, pushes the exponent `a^b`. Always returns a float.
* `log`: `(a b -- log_a[b])` Pops 2 `a` and `b`, pushes the logarithm `log_a(b)`. Always returns a float.
* `--`: Negation.
* `&`, `|`, `^`, `n`: Bitwise AND, OR, XOR, NOT. Ignores types and operates directly on the bits. Always returns an integer.

## Comparisons and Boolean Operations

All of these operators push a boolean. All binary operators pop `a` and `b` and push `a OP b`,


* `true`, `false`: The boolean constant. Pushes a boolean `true` or `false` onto the deque.
* `>`,`>=`,`<`,`<=`,`=`, `ne`: Numerical comparison. You can compare int and float to int and float. You can also compare char to char, but not char to any other type.
* `&&`, `||`, `nn`: Logical AND, OR, NOT. Non-zero integers and floats, non-empty lists, and blocks are truthy. All other values are falsey.

## List Operations

`[Char]` is the language's representation of a string.

List operations still operate on `[Char]` because `[Char]` is still a list.

* `l+`: Concatentates two values to result the following:
  * `(a, b -- [a, b])`
  * `(a, [... b] -- [... b, a])`
  * `([... a], b -- [... a, b])`
  * `([... a], [... b] -- [... a, ... b])`
* `lj`: List join. Pops a list, and some value `v`. Pushes a list where every element is separated by `v`: `0~ [1,2,3]~ lj~ => [1,0,2,0,3]`
* `l/`: List slice. Pops a list, and 2 ints `a` and `b`. Pushes a slice of the list from index `a`, inclusive, to `b`, exclusive.
* `li`: List index. Pops a list and an int, pushes the element at that index. Discards if the index is outside of list bounds.
* `ll`: List length. Pops a list and pushes the number of elements in the list.
* `lb`: List build. Pops an int, being the number of elements on the deque to take, and pushes a list of those elements.
* `ld`: List destructuring. Pops a list and pushes all elements in that list.

## List Functions
These instructions (usually) pop a list and a block and push a list.

* `map`: Pops a list and a block. Applies the block to every element in the list.
* `filter`: Pops a list and a conditional block. Appplies the block to every element in the list and pushes a list containing only the elements that succeeded the conditional block.
* `reduce`: Pops a list, an accumulator, and a block. For each element in the list, apply it to the element and the accumulator. Each iteration pushes the accumulator onto the deque after executing the block.

## Control Flow

All of these instructs pop blocks that are executed. In the following, a "condition block" is a block that leaves a boolean on the stack when provided a stack that satisfies its instructions. A "body block" is some block of code.

* `exec`: Pops and executes a block.
* `loop`: Infinite loop.
* `range`: Pops a lower bound `a`, upper bound `b`, an increment value `c`, and a body block. Equvalent to Python-like `for i in range(a,b,c): body`. Pushes the current index `i` onto the stack before executing the body block.
* `while`: Pops 2 blocks: a condition block, and a body block. The body block executes while the condition block is true. More specifically, it runs the condition block first, checks for a truthy value on the top/front of the deque (depending on which side `while` was called on), and if so, executes the body block. This then repeats until it sees a falsy value after executing the condition block.
* `ite`: If-Then-Else. Pops 3 blocks: a condtion block, a true block, and a false block. The blocks execute based on the condition block's output. If true, the true block executes. If false, the false block executes.

[comment]: <> (`in`: Pushes the current loop index.)
[comment]: <> (`break`: Exit the current loop.)

## Input and Output

* `il`: Consumes and pushes a line as a string from STDIN. Strips off the newline.
* `ia`: Consumes everything from STDIN and pushes it as a string to the stack.
* `ol`: Pops and prints an element, with a newline. `[Char]` is printed as a string.
* `ow`: Pops and prints an element. `[Char]` is printed as a string.
