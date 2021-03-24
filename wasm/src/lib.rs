use js_sys::Array;
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

/** Execute multiple line in a batch */
#[wasm_bindgen]
pub fn execute_batch(lines: &str) -> Array {
    let mut ctx = Context::empty();
    lines
        .lines()
        .map(|it| match tml::interpreter::compute(&mut ctx, it) {
            Ok(e) => highlight(&e),
            Err(e) => format!("<span class=\"error\">{}</span>", e),
        })
        .map(|it| JsValue::from_str(&it))
        .collect()
}

/** Highlight single line */
#[wasm_bindgen]
pub fn highlight(line: &str) -> String {
    tml::highlighter::highlight(line, HtmlHighlighter)
}

/** Highlight multiple lines in a batch */
#[wasm_bindgen]
pub fn highlight_batch(lines: &str) -> Array {
    lines
        .lines()
        .map(|line| JsValue::from_str(&highlight(line)))
        .collect()
}
