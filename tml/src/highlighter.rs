use crate::lexer::{Lexer, Sep, TokenKind};
use std::fmt::{self, Write};

pub enum SpanKind {
    Nb,
    Op,
    Fun,
    Var,
    Str,
    Comment,
    Error,
}

pub trait Highlighter {
    fn span(&mut self, write: impl Write, kind: SpanKind, span: &str) -> fmt::Result;
}

pub struct HtmlHighlighter;

fn span_kind_to_name(kind: SpanKind) -> &'static str {
    match kind {
        SpanKind::Nb => "number",
        SpanKind::Op => "operator",
        SpanKind::Fun => "function",
        SpanKind::Var => "variable",
        SpanKind::Str => "string",
        SpanKind::Comment => "comment",
        SpanKind::Error => "error",
    }
}

impl Highlighter for HtmlHighlighter {
    fn span(&mut self, mut writer: impl Write, kind: SpanKind, span: &str) -> fmt::Result {
        write!(
            writer,
            "<span class=\"{}\">{}</span>",
            span_kind_to_name(kind),
            span
        )
    }
}
pub struct AnsiHighlighter;

fn span_kind_to_ansi(kind: SpanKind) -> u8 {
    match kind {
        SpanKind::Nb => 34,
        SpanKind::Op => 33,
        SpanKind::Fun => 32,
        SpanKind::Var => 35,
        SpanKind::Str => 36,
        SpanKind::Comment => 90,
        SpanKind::Error => 31,
    }
}

impl Highlighter for AnsiHighlighter {
    fn span(&mut self, mut writer: impl Write, kind: SpanKind, span: &str) -> fmt::Result {
        write!(writer, "\x1b[0;{}m{}\x1b[0m", span_kind_to_ansi(kind), span)
    }
}

/** Generate styled HTML */
pub fn highlight(code: &str, mut highlighter: impl Highlighter) -> String {
    let mut lexer = Lexer::load(code);
    let peek = lexer.peek();
    let mut buf = String::new();
    if peek.kind() == TokenKind::Eof {
        "".into()
    } else if peek.kind() == TokenKind::Sep(Sep::Comment) {
        highlighter.span(&mut buf, SpanKind::Comment, code).unwrap();
        buf
    } else {
        let mut c = 0;
        loop {
            let token = lexer.next();
            let span = token.span();
            if c < span.start {
                buf.push_str(&code[c..span.start]);
            }
            c = span.end;
            match token.kind() {
                TokenKind::Nb => highlighter
                    .span(&mut buf, SpanKind::Nb, token.splice())
                    .unwrap(),
                TokenKind::Op(_) => highlighter
                    .span(&mut buf, SpanKind::Op, token.splice())
                    .unwrap(),
                TokenKind::Id => {
                    if lexer.peek().kind() == TokenKind::Sep(Sep::Open) {
                        highlighter
                            .span(&mut buf, SpanKind::Fun, token.splice())
                            .unwrap()
                    } else {
                        highlighter
                            .span(&mut buf, SpanKind::Var, token.splice())
                            .unwrap()
                    }
                }
                TokenKind::Str => highlighter
                    .span(&mut buf, SpanKind::Str, token.splice())
                    .unwrap(),
                TokenKind::Sep(_) => buf.push_str(token.splice()),
                TokenKind::Err => buf.push_str(token.splice()),
                TokenKind::Eof => return buf,
            }
        }
    }
}
#[cfg(test)]
mod test {
    use crate::highlighter::{highlight, AnsiHighlighter, HtmlHighlighter};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn highlight_anything(s: String) {
            highlight(&s, HtmlHighlighter);
            highlight(&s, AnsiHighlighter);
        }
    }
}
