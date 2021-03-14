# Tiny Mathematical Language

This project is a small experimentation with a mathematical language.

**WORK IN PROGRESS**

## Organisation

- [tml](./tml) The language implementation written in rust
- [wasm](./wasm) A wasm binding for integration with the web
- [website](./website) The project website and code editor written in svelte

## TML implementation

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

## Code Editor

The DOM is originally made to handle document content, and it do not scale well
with thousand of dynamic elements and do not give a lot of control. To get
around those limitation we can handle everything using a WebGL (but that require
a lot of work) or we can try to handle as much as possible manually and then
relie on the DOM for hard things like smooth scroll, font rendering, etc. I
chose to leverage the DOM.

### Virtual list

A technique to reduce significantly the number of elements is to only render
those who are actually visible in lists. When the use scroll, we computer
visible elements and position them manually with absolute position.

### Virtual scroll

If only a small parts of elements are rendered, the DOM can not handle the
scroll in lits as elements are only taking a portion of they real heights. We
need to compute the total heights of all elements to inform the DOM how to
handle vertical scrolling.

Horizontal scroll is harder to simulate, if we leverage the virtual list, the
scrollbar would be positioned at the bottom, out-of-view until the user reach
the bottom. We use virtual scroller. We position a div above the list content in
a sticky manner, and artificially create an internal width in proportion to the
list width. Then we need to keep list elements offset in sync with the virtual
scroller scrolling amount using styling.

### Position the cursor

Even if code font are most of the type monospaced, if you want to support UTF-8
there will always be variable length characters. This is a problem because there
is no easy way to compute text rendering length in the DOM. The editor heavily
use the expensive Range API to position cursor and find cursor position from
user input. When it is not possible to use Range (e.g estimate the width of the
widest line) we fallback to using lossy estimation based on arbitrary size. We
are working on removing all those fallbacks.
