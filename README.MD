# Tiny Mathematical Language

This project is a small experimentation with a mathematical language.

WORK IN PROGRESS

## Organisation

- [tml](./tml) The language implementation written in rust
- [wasm](./wasm) A wasm binding for integration with the web
- [website](./website) The project website and code editor written in svelte

## TML implementation

### The lexer

### The parser

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

There is no easy way to compute text rendering length, It is why we have
sometimes to rely on the monospaces characteristic of code font to compute
length. This is how we estimate internal width of virtual scrollers and the
position of the cursor.

When the user click on editable code, we use a less performant but more precise
technique using the Range API. We iterate on every character until we found the
one under the pointer.
