# Tiny Mathematical Language

This project is a small experimentation with a mathematical language.

**WORK IN PROGRESS**

## Organisation

- [tml](./tml) The language implementation written in rust
- [wasm](./wasm) A wasm binding for integration with the web
- [website](./website) The project website and code editor written in svelte

## Language implementation

![Implementation pipeline](./doc/pipeline.drawio.svg)

### Lexer

The role of the lexer is to find tokens in raw code. A token is a slice of UTF-8
characters associated with a kind (operator, number, etc.). The lexer is pull
designed making it allocation free.

### Highlighter

The role of the highlighter is to generate styled HTML from tokens.

### Parser

The role of the parser is to interpret tokens as a line. There is three types of
lines :

- _Expression_ when the line is interpretable
- _Comment_ when the line should be ignored
- _Empty_ when the line is empty

There is two type of expression based on what is returned :

- _Literal_ when a number is generated
- _Assign_ when a number is stored
- _Print_ when a string is generated

### Interpreter

The role of the interpreter is to perform operations encoded in expression.
