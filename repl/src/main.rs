use std::borrow::Cow;

use rustyline::{highlight::Highlighter, Config, Editor};
use rustyline_derive::{Completer, Helper, Hinter, Validator};
use tml::{
    highlighter::AnsiHighlighter,
    interpreter::{compute, Context},
};

#[derive(Helper, Completer, Hinter, Validator)]
struct TmlHelper;

impl Highlighter for TmlHelper {
    fn highlight<'l>(&self, line: &'l str, _: usize) -> std::borrow::Cow<'l, str> {
        Cow::Owned(tml::highlighter::highlight(line, AnsiHighlighter))
    }

    fn highlight_char(&self, _: &str, _: usize) -> bool {
        true
    }
}

fn main() {
    let config = Config::builder().auto_add_history(true).build();
    let mut editor = Editor::with_config(config);
    editor.set_helper(Some(TmlHelper));
    let mut ctx = Context::empty();
    loop {
        match editor.readline("> ") {
            Ok(line) => match compute(&mut ctx, &line) {
                Ok(result) => {
                    if !result.is_empty() {
                        println!("{}", tml::highlighter::highlight(&result, AnsiHighlighter))
                    }
                }
                Err(err) => println!("\x1b[0;31m{}\x1b[0m", err),
            },
            Err(_) => {
                break;
            }
        }
    }
}
