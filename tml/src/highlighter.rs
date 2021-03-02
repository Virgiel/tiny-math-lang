use crate::lexer::{Lexer, Sep, TokenKind};
use std::fmt::Write;

pub fn highlight_assign(id: &str, nb: f64) -> String {
    format!("<span class=\"variable\">{}</span> <span class=\"operator\">=<span> <span class=\"number\">{}</span>",  id, nb)
}

pub fn highlight_no_id(nb: f64) -> String {
    highlight_assign("λ", nb)
}

pub fn highlight_print(print: &str) -> String {
    format!("<span class=\"string\">{}</span>", print)
}

pub fn highlight_code(input: &str) -> String {
    let mut lexer = Lexer::load(input);
    let peek = lexer.peek();
    if peek.kind() == TokenKind::Eof {
        "".into()
    } else if peek.kind() == TokenKind::Sep(Sep::Comment) {
        format!("<span class=\"comment\">{}</span>", input)
    } else {
        let mut buf = String::new();
        let mut c = 0;
        loop {
            let token = lexer.next();
            let span = token.span();
            if c < span.start {
                buf.push_str(&input[c..span.start]);
            }
            c = span.end;
            match token.kind() {
                TokenKind::Nb => {
                    write!(buf, "<span class=\"number\">{}</span>", token.splice()).unwrap()
                }
                TokenKind::Op(_) => {
                    write!(buf, "<span class=\"operator\">{}</span>", token.splice()).unwrap()
                }
                TokenKind::Id => match lexer.peek().kind() {
                    TokenKind::Sep(Sep::Open) => {
                        write!(buf, "<span class=\"function\">{}</span>", token.splice()).unwrap()
                    }
                    _ => write!(buf, "<span class=\"variable\">{}</span>", token.splice()).unwrap(),
                },
                TokenKind::Str => {
                    write!(buf, "<span class=\"string\">{}</span>", token.splice()).unwrap()
                }
                TokenKind::Sep(_) => buf.push_str(token.splice()),
                TokenKind::Err => buf.push_str(token.splice()),
                TokenKind::Eof => return buf,
            }
        }
    }
}
