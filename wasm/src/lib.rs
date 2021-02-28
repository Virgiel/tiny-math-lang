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

#[wasm_bindgen]
pub fn exec_batch(arg: &str) -> Array {
    tml::exec_batch(arg)
        .iter()
        .map(|it| match it {
            Ok(e) => e.clone(),
            Err(e) => format!("<span class=\"error\">{}</span>", e),
        })
        .map(|it| JsValue::from_str(&it))
        .collect()
}

#[wasm_bindgen]
pub fn highlight_batch(arg: &str) -> Array {
    arg.lines()
        .map(|line| JsValue::from_str(&tml::highlight(line)))
        .collect()
}
