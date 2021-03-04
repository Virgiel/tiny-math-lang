use interpreter::Context;

mod highlighter;
mod interpreter;
mod lexer;
mod parser;

/** Execute multiple line of code, return highlighted html */
pub fn exec_batch(input: &str) -> Vec<Result<String, String>> {
    let mut ctx = Context::empty();
    input
        .lines()
        .map(|line| interpreter::compute(&mut ctx, line))
        .collect()
}

/** Execute a single line of code, return highlighted html */
pub fn exec_line(input: &str) -> Result<String, String> {
    let mut ctx = Context::empty();
    interpreter::compute(&mut ctx, input)
}

/** Highlight a line of code */
pub fn highlight(input: &str) -> String {
    highlighter::highlight(input)
}
