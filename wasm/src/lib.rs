use js_sys::Array;
use wasm_bindgen::prelude::*;
use wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

/// Execute a single line
#[wasm_bindgen]
pub fn execute(line: &str) -> String {
    match tml::exec_line(line) {
        Ok(e) => e.clone(),
        Err(e) => format!("<span class=\"error\">{}</span>", e),
    }
}

/// Execute multiple line in a batch
#[wasm_bindgen]
pub fn execute_batch(lines: &str) -> Array {
    tml::exec_batch(lines)
        .iter()
        .map(|it| match it {
            Ok(e) => e.clone(),
            Err(e) => format!("<span class=\"error\">{}</span>", e),
        })
        .map(|it| JsValue::from_str(&it))
        .collect()
}

/// Highlight single line
#[wasm_bindgen]
pub fn highlight(line: &str) -> String {
    tml::highlight(line)
}

/// Highlight multiple lines in a batch
#[wasm_bindgen]
pub fn highlight_batch(lines: &str) -> Array {
    lines
        .lines()
        .map(|line| JsValue::from_str(&highlight(line)))
        .collect()
}
