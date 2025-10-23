# IB Pseudocode Compiler

This tool is specifically designed to follow IB guidelines on how to write Pseudocode, implementing every feature with the same syntax and behavior, making it ideal for CS lessons and exam preparation.

The websites with the official IB documentation for pseudocode got taken down since September this year. It seems that next year exams will be based on a new curriculum, which looks like will not have this pseudocode at all. But for us, current year future graduates, the pseudocode is still a thing. Due to this, I tried to find and extract all the official guidelines and combine them into one single Markdown document, available on my website along with the compiler itself.

The unique feature of this compiler is that it is not some kind of wrapper/cheap `.replace` interpreter for Python or JavaScript, but a sophisticated program written in Rust, and engineered like a real compiler. The compiler follows these steps:

1. Pest parser. Parses your text and converts it into rules based on the grammar file;
2. AST Builder. These rules are then processed to create an Abstract Syntax Tree (AST) — a sequence of commands to be executed;
3. AST Validator. The initial AST is then validated for all the undefined behavior which can be spotted with a static analysis;
4. AST Evaluator. After validation, the AST is executed node by node.

During each step of the process, errors may occur. These errors are all caught, handled and shown to the user with an appropriate description for what went wrong and where.

As a proof of the validity of the compiler, I have written 40+ tests, which consist of:
* All the 32 sample programs from http://ibcomp.fis.edu/pseudocode/pcode.html;
* IB examples;
* Handwritten tests.

Check it out: https://fire-aalt.github.io/ib_pcode_compiler/

You are welcome to provide any feedback or drop a star if you like the project.

The syntax was inspired by "EZ Pseudocode" (c) Dave Mulkey 2012.
<br>The compiler and architecture are original (c) Maksim Krasutski 2025.