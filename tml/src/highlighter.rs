use crate::lexer::{Lexer, Sep, TokenKind};
use std::fmt::Write;

/** Generate styled HTML */
pub fn highlight(code: &str) -> String {
    let mut lexer = Lexer::load(code);
    let peek = lexer.peek();
    if peek.kind() == TokenKind::Eof {
        "".into()
    } else if peek.kind() == TokenKind::Sep(Sep::Comment) {
        format!("<span class=\"comment\">{}</span>", code)
    } else {
        let mut buf = String::new();
        let mut c = 0;
        loop {
            let token = lexer.next();
            let span = token.span();
            if c < span.start {
                buf.push_str(&code[c..span.start]);
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
#[cfg(test)]
mod test {
    use crate::highlight;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn highlight_anything(s: String) {
            highlight(&s);
        }
    }
}
