- [funk language basics](#funk-language-basics)
- [Introduction](#introduction)
  - [Expressions](#expressions)
  - [Functions and function calls](#functions-and-function-calls)
- [Symbols](#symbols)
  - [Operators](#operators)
    - [`=` Assignment](#-assignment)
    - [`==` Equality test](#-equality-test)
    - [`!=` Negative equality test](#-negative-equality-test)
    - [`>`/`>=` Greater than / or equal to test](#-greater-than--or-equal-to-test)
    - [`<`/`<=` Less than / or equal to test](#-less-than--or-equal-to-test)
    - [`+` Addition operator](#-addition-operator)
    - [`-` Subtraction operator](#--subtraction-operator)
    - [`*` Multiplication operator](#-multiplication-operator)
    - [`/` Division operator](#-division-operator)
    - [`and`/`or`/`not` operators](#andornot-operators)
  - [Parentheses `()`](#parentheses-)
  - [Curly braces `{}`](#curly-braces-)
  - [Brackets `[]`](#brackets-)
- [Top-level keywords](#top-level-keywords)
  - [`mut`](#mut)
  - [`def`](#def)
- [Keywords](#keywords)
  - [`break`](#break)
  - [`if` / `else`](#if--else)
  - [`loop`](#loop)
  - [`not`](#not)
  - [`return`](#return)
    <!-- - [`match`/`case`](#selectcase) -->
  - [`while`](#while)
  - [`with`](#with)
  - [`when`](#when)
- [Types:](#types)
  - [`Bool`](#bool-u1)
  - [`Byte (u8)`](#byte-u8)
  - [`Integer`](#i83264)
  - [`Decimal`](#u83264)
  - [`array`](#array)
  - [`str`](#str)
  - [`Vec`](#vec)

# funk language basics

This is a document of funk syntax and usage.

> ⚠ This document is incomplete and is being revised continuously.

# Introduction

## Expressions

Expressions are the basic unit of computation in funk. Each expression returns a value of a specific type. Every statement is itself an expression:

```
5
```

or

```
8 * (1 + 5) / (20 + 8 - (10 - 5))
```

## Functions and function calls

Use the `funk` keyword to define a function:

```
funk Integer Test(Integer a, String b) {
  return 10
}
```

Function definitions need to have:

- a name that does not shadow any existing name or keyword
- zero or more explicitly-typed arguments
- a return type
- and a function body.

In the above example:

- the name is `Test`
- the arguments are `a` and `b`, with an explicit type of `Integer` and `String`
- the return type is `Integer`
- and the body is simply the expression `return 10`.

Functions can also take optional arguments with defaults:

```
funk Integer Test(Integer a = 10) {
  return a
}
```

Note that optional arguments must always follow mandatory arguments.

# Symbols

## Operators

The following operator symbols are predefined:

### `=` Assignment

`Integer a = 5`

### `==` Equality test

```if (x == 1) {
  print(x)
}
```

### `!=` Negative equality test

```if (x != 1) {
  print(x)
}
```

### `>`/`>=` Greater than / or equal to test

```if (x >= 1) {
  print(x)
}
```

### `<`/`<=` Less than / or equal to test

```if (x <= 1) {
  print(x)
}
```

### `+` Addition operator

`1 + 1`

### `-` Subtraction operator

`2 - 1`

### `*` Multiplication operator

`5 * 5`

### `/` Division operator

`4 / 2`

### `and`/`or`/`xor`/`not` operators

Logical `and`, `or`, and `not`.

## Parentheses `()`

Parentheses are used to set aside clauses in expressions, and to identify the arguments for a function.

## Curly braces `{}`

Curly braces are used to set aside expression blocks.

## Hash symbol `#`

The hash symbol is a comment to the end of a line. (This is one of the few cases where linebreaks are honored as syntax.)

```
funk String main(){
    # This is a comment.
    return do_something() # Comment after statement.
}
```

There is no block comment syntax.

# Top-level keywords

"Top-level" keywords can only appear as the first level of keywords encountered by the compiler in a module.

## `funk`

Define a function signature and its body.

```
funk Integer add(Integer a, Integer b){
    return a+b
}

```

# Keywords

These keywords are valid within the body of a function.

## `break`

Exit a `loop` manually.

```
for (Integer x; x <= 10; x++) {
  if (x == 5) {
    break
  }
  print(x)
}
```

## `if` / `else`

If a given expression yields `True`, then yield the value of one expression; if `False`, yield the value of another.

```
Integer y = 0
Integer t = if (y == 1) 2 else 3

```

## `loop`

Defines a loop operation. The default is a loop that is infinite and needs to be exited manually with a `break`.

```
x = 0
for (x; x <= 10; x++) {
  print(x)
}
```

## `not`

A built-in unary for negating values.

```
Boolean x = True
Boolean y = not x # False
```

## `return`

Exits from a function early and returns a value.

```
funk String f1(Integer x) {
    if (x == 1) {
      return "Yes"
    } else {
      return "No"
    }
}
```

<!-- ## `match`/`case`

Evaluates an expression based on whether or not a value is equal to one of a given set of cases

The value returned from this expression is the selected value, _not any value returned by the expressions themselves._

There is no "fall-through" between cases

The `default` keyword indicates which expression to use if no other match can be found.

```
match a {
  case 1 => {
    a += 1
  },
  case 2 => {
    a *= 2
  },
  case 3..10 => { // Between 3 and up to 9
    a += 10
  },
  case ... => {}
}
``` -->

## `while`

Defines a loop condition that continues as long as a given condition is true.

```
Integer x=0
while (x < 100) {
    x++
}
```

## `with`

Provides a context, or closure, for variable assignments.

```
Integer y = 1 # y is valid from here on down

with Integer x = 32 {
    y + x
    # but use of x is only valid in this block
}
```

As with variables generally, a variable name in a `with` block cannot "shadow" one outside.

```
Integer y=1
with Integer y = 2 { # this is invalid
    ...
}
```

## `when`

```If a given expression yields `True`, then use the value of one expression; if `False`, use the value of another.

Differs from `if/then/else` in that the `else` clause is optional, and that the value yielded is that of the _deciding expression_, not the `then/else` expressions. This way, the values of the `then/else` expressions can be of entirely different types if needed.

```
when x=1 do_something() # this function returns an u64
else if x=2 do_something_else() # this function returns an i32
else do_yet_another_thing() # this function returns an i8
```

In all cases the above expression would return the value of whatever `x` was, not the value of any of the called functions.```

# Types:

## `bool (u1)`

An unsigned true or false value.

Constant representation of 1: `1:bool`

## `byte (u8)`

An unsigned byte.

Constant representation of 1: `1:byte`

## `i8/32/64`

Signed integers of 8, 32, or 64 bit widths.

Constant representation of 1: `1:i8, 1:i32, 1:i64`

The default variable type is a 32-bit signed integer (`1:i32`).

## `u8/32/64`

Unsigned integers of 8, 32, or 64 bit widths.

Constant representation of 1: `1:u8, 1:u32, 1:u64`

## `f32/64`

Floats of 32 or 64 bit widths: `3.2:f32`/ `3.2:f64`.

Constant representation of 1: `1.` or `1.0`.

## `array`

An array of scalar (integer or float) types.

For a one-dimensional array of bytes:

`var x:array byte[100]`

For a multidimensional array of bytes:

`var x:array byte[32,32]`

> ⚠ There is as yet no way to define array members on creation. They have to be assigned individually.

> ⚠ There is as yet no way to nest different scalars in different array dimensions.

> ⚠ There is as yet no way to perform array slicing or concatenation.

## `str`

A string of characters, defined either at compile time or runtime.

```
hello = "Hello World!"
hello_also = 'Hello world!'
hello_again = 'Hello
world!'
```

Linebreaks inside strings are permitted. Single or double quotes can be used as long as they match.

String escaping functions are not yet robustly defined, but you can use `\n` in a string for a newline, and you can escape quotes as needed with a backslash as well:

```
hello = "Hello \"world\"! \n"
```

> ⚠ There is as yet no way to perform string slicing or concantenation.
