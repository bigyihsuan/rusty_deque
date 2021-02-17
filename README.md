# rusty_deque

A simple, deque-based programming language to practice writing Rust.

# Deque

The main data structure is a [deque](https://en.wikipedia.org/wiki/Deque), which allows insertion and removal of elements at both ends.

Inspired by [this Esolangs page](https://esolangs.org/wiki/Deque) and [this Concatenative page](https://concatenative.org/wiki/view/Deque).

# Types

The language contains the following primitve types:

* Signed 32-bit Ints
* 32-bit Floats
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
```

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

## Deque Operations
* (`push`: Handled by literals.)
* `pop`/`$`: Discard one element.
* `dup`/`:`: Duplicate one element.
* `rot`/`@` : Rotate the deque one element towards a direction (either to the front `!rot` or the back `rot!`).
* `over`/`^` : Duplicate the element below the top/bottom element.

## Aritmetic Operations

All of these operate on ints and floats only. Non-ints and non-floats are discarded, with no errors.
It tries to do integer operations by default; if there is a float, it pushes a float.

* `+`: Pops 2, pushes the sum.
* `-`: Pops 2, pushes the difference.
* `*`: Pops 2, pushes the product.
* `/`: Pops 2, pushes the quotient. Discards if the second element is 0.
* `**`: Pops 2 `a` and `b`, pushes the exponent `a^b`.
* `//`: Pops 2 `a` and `b`, pushes the logarithm `log_a(b)`
* 