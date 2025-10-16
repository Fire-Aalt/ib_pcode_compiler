# IB Pseudocode Compiler — User Documentation

This documentation explains how to **write pseudocode** programs for the Rust-based pseudocode compiler. It describes the language features implemented in the grammar and shows short examples for each feature.

---

## Table of Contents

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
    * [Comparison](#comparison)
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

## Basic syntax

### Comments

Use `//` to write a comment.

```text
// This is a comment
```

### Whitespace & line endings

Whitespace and newlines are ignored except where needed to separate statements. Each statement should end with a **newline**.

---

## Program structure

A program is a sequence of statements (one statement per line). Example:

```text
x = 10
y = 5
output x + y
```

---

## Variables and assignment

### Basic assignment

```text
a = 5
b = "Hello"
```

### Compound assignments

```text
a += 2  // the same as `a = a + 2`
b -= 3
c *= 4
d /= 2
```

### Increment / Decrement

```text
counter++  // the same as `counter = counter + 1`
index--
```

---

## Expressions

Supports arithmetic, comparison, logical operators and parentheses.

### Arithmetic

```text
x = 3 + 2 * 5
y = 10 / 2
z = 2 ^ 3       // power (2³ = 8)
a = 7 div 2     // integer division (3)
b = 7 mod 2     // remainder (1)
```

### Comparison

```text
x == y
x != y
x > y
x <= y
```

### Logical

```text
a && b      // AND operation can be written as `&&` or `AND`
a || b      // OR operation can be written as `||` or `OR`
!a          // NOT operation can be written as `!` or `NOT`
```

---

## Control flow

### If / Else

```text
if x > 10 then        // one `if` clause with a condition
    output "Large"
else if x > 5 then    // any number of `else if` clauses with a condition
    output "Medium"
else                  // optional `else` clause
    output "Small"
end if
```

### While loop

```text
loop while x < 10
    x++
end loop
```

### For loop

```text
loop i from 1 to 5
    output i
end loop
```

### Until loop

```text
loop until done == true
    done = check_condition()
end loop
```

---

## Methods (functions)

### Declaring a method

```text
method greet(name)
    output "Hello, " + name
end method
```

### Returning a value

```text
method square(x)
    return x * x
end method
```

### Calling a method

```text
greet("Alice")
result = square(5)
```

---

## Classes and objects

### Class declaration

```text
Class Person(name, age)
    public this.name = name
    public this.age = age

    this.greet = function()
    {
        output "Hello, my name is " + this.name
    }
end Class
```

### Creating an object and calling methods

```text
p = new Person("Alice", 25)
p.greet()
```

### Static classes

Static classes hold global/static data and methods. Example usage:

```text
static Class HashUtil()
    public this.OFFSET = 8784213548
    
    this.hash = function(x)
    {
        return x + this.OFFSET
    }
end Class

output HashUtil.hash(5)
```

---

## Input / Output

### Input

```text
input name
```

Stores user input in the variable `name`.

### Output

```text
output "Result:", x + y
```

Multiple values may be provided separated by commas. The output string will join the values by whitespace. If any of the values produce strings which start or end with spaces — these spaces are trimmed.

---

## Assertions

Used for testing and validation:

```text
assert(x + y, 10)
```

If the two expressions are not equal, the runtime will raise an assertion error.

---

## Arrays

### Literals and creation

```text
arr = [1, 2, 3]
arr2 = new Array()
```

### Accessing elements

```text
x = arr[0]
arr[1] = 5
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

---

## Included standard classes

The compiler ships with several ready-to-use classes: **Collection**, **Queue**, **Stack**, and a static **Math** class. You can use these directly in your programs.

### Collection

A simple dynamic-list helper with iteration support.

**Fields**

* `array` — underlying array storage
* `index` — number of items currently stored
* `next` — iterator pointer for `getNext` / `hasNext`

**Methods**

* `addItem(item)` — append `item` to the collection.
* `remove(item)` — remove the first occurrence of `item`; returns `true` if removed, otherwise `false`.
* `contains(item)` — returns `true` if `item` exists in the collection.
* `resetNext()` — set iteration pointer to start.
* `hasNext()` — returns `true` if more items remain for iteration.
* `getNext()` — returns next item and advances iterator.
* `isEmpty()` — returns `true` if collection is empty.

**Examples**

```text
c = new Collection()
c.addItem(10)
c.addItem(20)
output c.contains(20)   // true

loop while c.hasNext()
    val = c.getNext()
    output val
end loop

c.resetNext()           // reset is needed if the Collection will be iterated later
```

---

### Queue

First-in-first-out queue.

**Fields**

* `array` — queue storage
* `index` — position where next item will be enqueued
* `head` — position of next item to dequeue

**Methods**

* `enqueue(item)` — push `item` to the back of the queue.
* `dequeue()` — remove and return the front item (no safety check, check `isEmpty` before dequeueing for robustness).
* `isEmpty()` — returns `true` if queue is empty.

**Examples**

```text
q = new Queue()
q.enqueue(1)
q.enqueue(2)

while NOT q.isEmpty()
    v = q.dequeue()
    output v
end while
```

---

### Stack

Last-in-first-out stack.

**Fields**

* `array` — storage
* `index` — next push position (also size)

**Methods**

* `push(item)` — push `item` onto stack.
* `pop()` — pop and return top item (no safety check — use `isEmpty()` first).
* `isEmpty()` — returns `true` if stack contains no items.

**Examples**

```text
s = new Stack()
s.push(10)
s.push(20)
while NOT s.isEmpty()
    v = s.pop()
    output v
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
output Math.PI
output Math.abs(-3.5)
output Math.pow(2, 10)        // 1024
output Math.pow(9, 0.5)       // 3

// domain checks
output Math.log(-1)           // undefined

// trig
output Math.sin(Math.PI / 2)  // ~1

// random
output Math.random()
```

---

## Native methods provided by the compiler

The compiler recognizes and rewrites a handful of special methods and field-lookups into native calls.

### Global native functions

* `div(a, b)` — recognized as an integer-division native call. Maps to `a div b`.
* `input()` — recognized as a native input call. Can have zero or one parameters. If one parameter is present, it is evaluated and used as a text shown to the user.

**Example**

```text
x = div(7, 2)                 // 3
name = input()                // : <user input>
mood = input("How are you?")  // How are you?: <user input>
```

### Native field / property access

* `.length` — can be used on an array or a string to get the current length of the data type.

**Example**

```text
arr = [1,2,3]
output arr.length    // native length call on `arr`
```

### Native instance methods

* `.substring(start, end)` — can be used on a string. Returns a substring, where `start` parameter determines the start index of the string slice, `end` parameter determines the end index of the string slice.

**Example**

```text
s = "Hello"
output s.substring(1, 3)   // "el"
```

## Example program

```text
Class Counter(start)
    public this.value = start

    this.increment = function()
    {
        this.value++
    }

    this.print = function()
    {
        output "Value:", this.value
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