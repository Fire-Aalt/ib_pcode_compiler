# IB Pseudocode Compiler — User Documentation

This documentation explains how to write IB approved pseudocode programs. It describes the language features implemented in the grammar and shows short examples for each feature.

---

## Table of Contents

* [Conventions](#conventions)
* [Basic syntax](#basic-syntax)
    * [Comments](#comments)
    * [Whitespace & line endings](#whitespace--line-endings)
* [Program structure](#program-structure)
* [Variables and assignment](#variables-and-assignment)
    * [Basic assignment](#basic-assignment)
    * [Compound assignments](#compound-assignments)
    * [Increment / Decrement](#increment--decrement)
* [Expressions](#expressions)
    * [Arithmetic](#arithmetic)
    * [Logical](#logical)
* [Control flow](#control-flow)
    * [If / Else](#if--else)
    * [While loop](#while-loop)
    * [For loop](#for-loop)
    * [Until loop](#until-loop)
* [Methods (functions)](#methods-functions)
    * [Declaring a method](#declaring-a-method)
    * [Returning a value](#returning-a-value)
    * [Calling a method](#calling-a-method)
* [Classes and objects](#classes-and-objects)
    * [Class declaration](#class-declaration)
    * [Creating an object and calling methods](#creating-an-object-and-calling-methods)
    * [Static classes](#static-classes)
* [Input / Output](#input--output)
    * [Input](#input)
    * [Output](#output)
* [Assertions](#assertions)
* [Arrays](#arrays)
    * [Literals and creation](#literals-and-creation)
    * [Accessing elements](#accessing-elements)
* [Data Types](#data-types)
* [Included standard classes](#included-standard-classes)
    * [Collection](#collection)
    * [Queue](#queue)
    * [Stack](#stack)
    * [static Math](#static-math)
* [Example program](#example-program)

---
## Conventions
| Section             | Naming Convention | Example        |
|---------------------|-------------------|----------------|
| Variable names      | All capitals      | `CITY`         |
| Method names        | camelCase         | `getRecord`    |
| Class names         | PascalCase        | `BankRegistry` |
| Pseudocode keywords | Lowercase         | `loop`, `if`   |


## Basic syntax

### Comments

Use `//` to write a comment. Comments are mostly used to describe parts of the code.

```text
// This is a comment
RESET = true // Mark for reset as ... is not needed anymore
```

### Whitespace & line endings

Whitespace and newlines are ignored except where needed to separate statements. Each statement should end with a **newline**.

---

## Program structure

A program is a sequence of statements (one statement per line). Example:

```text
X = 10
Y = 5
output X + Y
```

---

## Variables and assignment

### Basic assignment

```text
A = 5
B = "Hello"
```

### Compound assignments

```text
A += 2  // the same as `A = A + 2`
B -= 3
C *= 4
D /= 2
```

### Increment / Decrement

```text
COUNTER++  // the same as `COUNTER = COUNTER + 1`
INDEX--
```

---

## Expressions

Supports arithmetic, comparison, logical operators and parentheses.

### Arithmetic

|     Symbol | Definition               | Examples       | Example usage |
|-----------:|--------------------------|----------------|---------------|
|        `+` | addition                 | `2 + 2 = 4`    | `A = B + 2`   |
|        `-` | substraction             | `6 - 7 = -1`   | `A = B - 7`   |
|        `*` | multiplication           | `15 * 4 = 60`  | `A = B * 5`   |
|        `/` | division                 | `60 / 12 = 5`  | `A = B / 9`   |
| `mod`, `%` | modulo/remainder         | `15 mod 7 = 1` | `A = B mod 9` |
|      `div` | integer part of quotient | `15 div 7 = 2` | `A = B div 9` |

Edge cases are solved as follows:

| Expression            | Result    |
|-----------------------|-----------|
| n / ±Infinity         | 0         |
| ±Infinity * ±Infinity | ±Infinity |
| ±(non-zero) / 0       | ±Infinity |
| Infinity + Infinity   | Infinity  |
| ±0 / ±0               | NaN       |
| Infinity - Infinity   | NaN       |
| ±Infinity / ±Infinity | NaN       |
| ±Infinity * 0         | NaN       |

### Logical

|       Symbol | Definition                  | Examples           | Example usage             |
|-------------:|-----------------------------|--------------------|---------------------------|
|    `=`, `==` | is equal to                 | `X = 4`, `X = K`   | `if X = 4`                |
|          `>` | is greater than             | `X > 4`            | `if X > 4 then`           |
|         `>=` | is greater than or equal to | `X >= 6`           | `loop while X >= 6`       |
|          `<` | is less than                | `VALUE[Y] < 7`     | `loop until VALUE[Y] < 7` |
|         `<=` | is less than or equal to    | `VALUE[] <= 12`    | `if VALUE[Y] <= 12 then`  |
|   `!=`, `<>` | not equal to                | `X != 4`, `X != K` | `if X != 10 then`         |
|  `AND`, `&&` | logical AND                 | `A AND B`          | `if X < 7 AND Y > 2 then` |
| `OR`, `\|\|` | logical OR                  | `A OR B`           | `if X < 7 OR Y > 2 then`  |
|   `NOT`, `!` | logical NOT                 | `NOT A`            | `if NOT X = 7 then`       |

```text
output !false && (1 + 6) div 7 == 1 && 5 > 4 mod 2
// Outputs `true`
```

---

## Control flow

All the control blocks introduce a local scope.
What is in the local scope of an `if`, `loop`, `method` and `Class` statements, exists only in those statements.
So a variable defined in the body of a `for` statement will only exist in the body of that statement and all of its inner scopes.
This behavior is the same across all the programming languages.<br>

### If / Else

```text
if X > 10 then        // one `if` clause with a condition
    output "Large"
else if X > 5 then    // any number of `else if` clauses with a condition
    output "Medium"
else                  // optional `else` clause
    output "Small"
end if
```

### While loop

```text
loop while X < 10
    X++
end loop
```

### For loop

```text
I = "Something not an integer"

loop I from 1 to 5  // inclusive
    output I        // 1... 2... 3... 4... 5
end loop

output I    // Something not an integer
```

### Until loop

```text
loop until DONE == true
    DONE = checkCondition()
end loop
```

---

## Methods (functions)

### Declaring a method

```text
method greet(NAME)
    output "Hello, " + NAME
end method
```

### Returning a value

```text
method square(X)
    return X * X
end method
```

### Calling a method

```text
greet("Alice")
RESULT = square(5)
```

---

## Classes and objects

### Class declaration

```text
Class Person(NAME, AGE)
    public this.NAME = NAME
    public this.AGE = AGE

    this.greet = function()
    {
        output "Hello, my name is " + this.NAME
    }
end Class
```

### Creating an object and calling methods

```text
P = new Person("Alice", 25)
P.greet()
```

### Static classes

Static classes hold global/static data and methods. Example usage:

```text
static Class HashUtil()
    public this.OFFSET = 8784213548
    
    this.hash = function(X)
    {
        return X + this.OFFSET
    }
end Class

output HashUtil.hash(5)
```

---

## Input / Output

### Input

```text
input NAME
```

Stores user input in the variable `NAME`.

### Output

```text
output "Result:", X + Y
```

Multiple values may be provided separated by commas. The output string will join the values by whitespace. If any of the values produce strings which start or end with spaces — these spaces are trimmed.

---

## Assertions

This feature is NOT part of IB guidelines and should only be used for debugging, testing and validation purposes:

```text
assert(X + Y, 10)
```

If the two expressions are not equal, the runtime will raise an assertion error.

---

## Arrays

### Literals and creation

```text
ARR = [1, 2, 3]
ARR2 = new Array()
```

### Accessing elements

```text
X = ARR[0]
ARR[1] = 5
```

---

## Data Types

| Type           | Example                    |
|----------------|----------------------------|
| Number         | `5`, `3.14`, `2e10`        |
| String         | `"Hello"`                  |
| Boolean        | `true`, `false`            |
| Undefined      | `undefined`                |
| Array          | `[1, 2, 3]`, `new Array()` |
| Class instance | `new MyClass()`            |

Numbers have limited precision up to 15 digits total. So if you have 15 digits before `.`, or 15 digits after, or 7 before and 8 after - it will result in the maximum precision without any rounding errors.
All the numbers are represented as float64 to mimic the behavior of the "EZ Pseudocode" (c) Dave Mulkey 2012.*

---

## Included standard classes

The compiler ships with several ready-to-use classes: **Collection**, **Queue**, **Stack**, and a static **Math** class. You can use these directly in your programs.

### Collection

A simple dynamic-list helper with iteration support.

**Methods**

* `addItem(item)` — append `item` to the collection.
* `remove(item)` — remove the first occurrence of `item`; returns `true` if removed, otherwise `false`.
* `contains(item)` — returns `true` if `item` exists in the collection.
* `resetNext()` — set an iteration pointer to start.
* `hasNext()` — returns `true` if more items remain for iteration.
* `getNext()` — returns next item and advances iterator (does not remove the element).
* `isEmpty()` — returns `true` if the collection is empty.

**Examples**

```text
C = new Collection()
C.addItem(10)
C.addItem(20)
output C.contains(20)   // true

loop while C.hasNext()
    VAL = C.getNext()
    output VAL
end loop

C.resetNext()           // reset is needed if the Collection will be iterated later
```

---

### Queue

First-in-first-out queue.

**Methods**

* `enqueue(item)` — push `item` to the back of the queue.
* `dequeue()` — remove and return the front item (no safety check, check `isEmpty` before dequeue for robustness).
* `isEmpty()` — returns `true` if queue is empty.

**Examples**

```text
Q = new Queue()
Q.enqueue(1)
Q.enqueue(2)

while NOT Q.isEmpty()
    V = Q.dequeue()
    output V
end while
```

---

### Stack

Last-in-first-out stack.

**Methods**

* `push(item)` — push `item` onto stack.
* `pop()` — pop and return top item (no safety check — use `isEmpty()` first).
* `isEmpty()` — returns `true` if stack contains no items.

**Examples**

```text
S = new Stack()
S.push(10)
S.push(20)
while NOT S.isEmpty()
    V = S.pop()
    output V
end while
```

---

### static Math

A math utility class containing constants and common numeric functions.
Use `Math.<name>` to access constants and functions.

**Constants**

* `E`, `LN10`, `LN2`, `LOG10E`, `LOG2E`, `PI`, `SQRT1_2`, `SQRT2`

**Selected Functions & behavior**

* `abs(x)` — absolute value.
* `sign(x)` — returns `-1`, `0`, or `1`.
* `trunc(x)` — truncate toward zero (implemented with integer division).
* `floor(x)`, `ceil(x)`, `round(x)` — rounding operations.
* `max(a,b)`, `min(a,b)` — pairwise max/min.

**Exponential / Logarithm**

* `exp(x)`, `expm1(x)` — exponential and exp(x)-1 approximations.
* `log(x)`, `log1p(x)`, `log10(x)`, `log2(x)` — natural/log-base functions (return `undefined` for invalid inputs).

**Power & roots**

* `pow(x, y)` — supports integer exponents for negative bases; for non-integer `y` and negative `x` returns `undefined`. Special-cases: `0^0` => `1`, `0^neg` => `undefined`.
* `sqrt(x)`, `cbrt(x)` — square and cube root (return `undefined` for invalid inputs when appropriate).

**Trigonometry**

* `sin(x)`, `cos(x)`, `tan(x)`, `asin(x)`, `acos(x)`, `atan(x)`, `atan2(y,x)` — standard trigonometry and inverse trigonometry (return `undefined` on domain errors).

**Hyperbolic**

* `sinh(x)`, `cosh(x)`, `tanh(x)` and their inverses `asinh`, `atanh`, `acosh`.

**Random**

* `random()` — returns a random number in the range [0; 1]

**Examples**

```text
output Math.PI                // 3.14...
output Math.abs(-3.5)         // 3.5
output Math.pow(2, 10)        // 1024
output Math.pow(9, 0.5)       // 3

// domain checks
output Math.log(-1)           // undefined

// trig
output Math.sin(Math.PI / 2)  // ~1

// random
output Math.random()          // Random from 0 to 1
```

---

## Native methods provided by the compiler

The compiler recognizes and rewrites a handful of special methods and field-lookups into native calls.

### Global native functions

* `div(a, b)` — recognized as an integer-division native call. Maps to `a div b`.
* `input()` — recognized as a native input call. Can have zero or one parameters. If one parameter is present, it is evaluated and used as a text shown to the user.

**Example**

```text
X = div(7, 2)                 // 3
NAME = input()                // : <user input>
MOOD = input("How are you?")  // How are you?: <user input>
```

### Native field / property access

* `.length` — can be used on an array or a string to get the current length of the data type.

**Example**

```text
ARR = [1,2,3]
output ARR.length    // native length call on `ARR`
```

### Native instance methods

* `.substring(start, end)` — can be used on a string. Returns a substring, where `start` parameter determines the start index of the string slice, `end` parameter determines the end index of the string slice.

**Example**

```text
S = "Hello"
output S.substring(1, 3)   // "el"
```

## Example program

```text
Class Counter(START)
    public this.VALUE = START

    this.increment = function()
    {
        this.VALUE++
    }

    this.print = function()
    {
        output "Value:", this.VALUE
    }
end Class

method main()
    c = new Counter(0)
    loop i from 1 to 5
        c.increment()
    end loop
    c.print()
end method

main()
```