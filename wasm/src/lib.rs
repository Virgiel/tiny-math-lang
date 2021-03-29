use std::fmt::Write;
use tml::{highlighter::HtmlHighlighter, interpreter::Context};
use wasm_bindgen::prelude::*;
use wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

/** Execute a single line */
#[wasm_bindgen]
pub fn execute(line: &str) -> String {
    let mut ctx = Context::empty();
    match tml::interpreter::compute(&mut ctx, line) {
        Ok(e) => highlight(&e),
        Err(e) => format!("<span class=\"error\">{}</span>", e),
    }
}

#[wasm_bindgen]
pub struct BatchResult {
    content: String,
    lines_height: Vec<u16>,
}

#[wasm_bindgen]
impl BatchResult {
    pub fn content(&self) -> String {
        self.content.clone()
    }

    pub fn lines_height(&self) -> Vec<u16> {
        self.lines_height.clone()
    }
}

/** Execute multiple line in a batch */
#[wasm_bindgen]
pub fn execute_batch(lines: &str) -> BatchResult {
    let mut ctx = Context::empty();
    lines.lines().fold(
        BatchResult {
            content: String::new(),
            lines_height: Vec::new(),
        },
        |mut acc, line| match tml::interpreter::compute(&mut ctx, line) {
            Ok(line) => {
                tml::highlighter::highlight(&mut acc.content, &line, HtmlHighlighter).unwrap();
                acc.content.push('\n');
                acc.lines_height.push(1);
                return acc;
            }
            Err(e) => {
                writeln!(&mut acc.content, "<span class=\"error\">{}</span>", e).unwrap();
                acc.lines_height
                    .push(e.chars().filter(|c| *c == '\n').count() as u16 + 1);
                acc
            }
        },
    )
}

/** Highlight single line */
#[wasm_bindgen]
pub fn highlight(line: &str) -> String {
    let mut buf = String::new();
    tml::highlighter::highlight(&mut buf, line, HtmlHighlighter).unwrap();
    return buf;
}

/** Highlight multiple lines in a batch */
#[wasm_bindgen]
pub fn highlight_batch(lines: &str) -> BatchResult {
    lines.lines().fold(
        BatchResult {
            content: String::new(),
            lines_height: Vec::new(),
        },
        |mut acc, line| {
            tml::highlighter::highlight(&mut acc.content, line, HtmlHighlighter).unwrap();
            acc.content.push('\n');
            acc.lines_height.push(1);
            return acc;
        },
    )
}
