use std::borrow::Cow;

use rustyline::{highlight::Highlighter, Config, Editor};
use rustyline_derive::{Completer, Helper, Hinter, Validator};
use tml::{
    highlighter::{self, AnsiHighlighter},
    interpreter::{compute, Context},
};

#[derive(Helper, Completer, Hinter, Validator)]
struct TmlHelper;

impl Highlighter for TmlHelper {
    fn highlight<'l>(&self, line: &'l str, _: usize) -> std::borrow::Cow<'l, str> {
        let mut buf = String::new();
        highlighter::highlight(&mut buf, line, AnsiHighlighter).unwrap();
        Cow::Owned(buf)
    }

    fn highlight_char(&self, _: &str, _: usize) -> bool {
        true
    }
}

fn main() {
    let args: String = std::env::args()
        .skip(1)
        .fold(String::new(), |buf, elem| buf + " " + &elem);
    if args.is_empty() {
        // Start repl
        let config = Config::builder().auto_add_history(true).build();
        let mut editor = Editor::with_config(config);
        editor.set_helper(Some(TmlHelper));
        let mut ctx = Context::empty();
        loop {
            match editor.readline("> ") {
                Ok(line) => match compute(&mut ctx, &line) {
                    Ok(result) => {
                        if !result.is_empty() {
                            let mut buf = String::new();
                            highlighter::highlight(&mut buf, &result, AnsiHighlighter).unwrap();
                            println!("{}", buf);
                        }
                    }
                    Err(err) => println!("\x1b[0;31m{}\x1b[0m", err),
                },
                Err(_) => {
                    break;
                }
            }
        }
    } else {
        // Execute single line
        let mut ctx = Context::empty();
        match compute(&mut ctx, &args) {
            Ok(result) => {
                if !result.is_empty() {
                    let mut buf = String::new();
                    highlighter::highlight(&mut buf, &result, AnsiHighlighter).unwrap();
                    println!("{}", buf);
                }
            }
            Err(err) => println!("\x1b[0;31m{}\x1b[0m", err),
        };
    }
}
