# rusty_deque

A simple, deque-based programming language to practice writing Rust.

# Deque

The main data structure is a [deque](https://en.wikipedia.org/wiki/Deque), which allows insertion and removal of elements at both ends.

Inspired by [this Esolangs page](https://esolangs.org/wiki/Deque) and [this Concatenative page](https://concatenative.org/wiki/view/Deque).

# Examples

### Hello World
```bash

```

# Types

The language contains the following primitve types:

* Signed 64-bit Ints
* 64-bit Floats
* Booleans
* Characters

Floats contain a `.`, and floats `0 < f < 1` must start with `0.`. Booleans are `true` and `false`. Characters are surrounded by single quotes `'c'`.

The language has a single compund type: **a List**. Lists can be nested, and its elements can have different types. List elements are separated by commas.

# Literals

Literals are pushed onto the deque as-is. The appending operator denotes where it gets pushed to (see section **Appending Operator** below).

```bash
123   # postive int
-321  # negative int
9.87  # float
'c'   # single character
true  # boolean
false # boolean
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

Blocks are code treated as a literal. Code surrounded in curly brackets `{}` are pushed as a literal. You can nest blocks in blocks.

# Comments

Comments start with a hash mark `#` and end at the end of the line.

```bash
# this is a comment
1! !2 !+ # comment after some code
```


# Appending Operator `!`

Instructions and literals are either prepended or postpended by an exclamation mark `!`.
The location of this exclamation mark determines where on the deque it looks to: the front (`!add`), like a stack, or the back (`add!`), like a queue.

# Instructions

Instructions have the general philosophy of "discard if failed". If an instruction is called and its supplied parameters are of the incorrect domain, it will discard these values.

## Deque Operations
* (`push`: Handled by literals.)
* `pop`/`$`: Discard one element.
* `dup`/`:`: Duplicate one element.
* `rot`/`@` : Rotate the deque one element towards a direction (either to the front `!rot` or the back `rot!`).
* `over`/`^` : Duplicate the element below the top/bottom element.

## Castings

These operations attempt to cast primitives to primitives. If the conversion fails, it discards the value.

* `toInt`: Pops 1 and pushes an int. Floats are truncated at the decimal point. Characters have their Rust `c as u32` value.
* `toFloat`: Pops 1 and pushes a float. Characters have their Rust `c as u32` value.
* `toChar`: Pops 1 and pushes a char. Uses Rust `std::char::to_char(v)`.
* `toBool`: Pops 1 and pushes a bool. For ints and floats, this is `v != 0`. This is always `true` for chars. This is `true` for lists only when non-empty.

## `Int`/`Float` Operations

All of these operate on ints and floats only. Non-ints and non-floats are discarded, with no errors.
It tries to do integer operations by default; if there is a float, it pushes a float.

* `+`: Sum.
* `-`: Difference.
* `*`: Product.
* `/`: Quotient. Discards if the second element is 0.
* `**`: Pops 2 `a` and `b`, pushes the exponent `a^b`.
* `//`: Pops 2 `a` and `b`, pushes the logarithm `log_a(b)`
* `--`: Negation.
* `&`, `|`, `n`: Bitwise AND, OR, NOT. Pushes an int.

## Comparisons and Boolean Operations

All of these operators push a boolean.

* `>`,`>=`,`<`,`<=`,`=`: Numerical comparison. You can compare ints, floats, and chars to each other.
* `&&`, `||`, `nn`: Logical AND, OR, NOT.

## List Operations

`[Char]` is the language's representation of a string.

List operations still operate on `[Char]` because `[Char]` is still a list.

* `l+`: Concatentates to result the following:
  * `a, b => [a, b]`
  * `a, [... b] => [... b, a] `
  * `[... a], b => [... a, b] `
  * `[... a], [... b] => [... a, ... b]`
* `l/`: List slice. Pops a list, and 2 ints `a` and `b`. Pushes a slice of the list from index `a`, inclusive, to `b`, exclusive.
* `li`: List index. Pops a list and an int, pushes the element at that index. Discards if the index is outside of list bounds.
* `ll`: List length. Pops a list and pushes the number of elements in the list.

# Control Flow

* 