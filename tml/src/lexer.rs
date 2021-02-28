use std::ops::Range;

use crate::{Op, Sep};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Nb,
    Op(Op),
    Id,
    Sep(Sep),
    Err,
    Eof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<'a> {
    source: &'a str,
    kind: TokenKind,
    span: Range<usize>,
}

impl<'a> Token<'a> {
    fn new(source: &'a str, kind: TokenKind, span: Range<usize>) -> Self {
        Self { source, kind, span }
    }

    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    pub fn span(&self) -> &Range<usize> {
        &self.span
    }

    pub fn splice(&self) -> &'a str {
        &self.source[self.span.clone()]
    }

    pub fn err_there(&self, err: &str) -> String {
        format!(
            "{}\n{}\n{:>4$}{:^>5$}",
            err,
            self.source,
            "",
            "^",
            self.span.start,
            self.span.len()
        )
    }
}

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    input: &'a str,
    index: usize,
    peeked: Option<Token<'a>>,
}

impl<'a> Lexer<'a> {
    /// Lex in a pull based manner
    pub fn load(input: &'a str) -> Lexer {
        Lexer {
            input,
            index: 0,
            peeked: None,
        }
    }

    /// Lex in a collect manner
    pub fn collect(input: &'a str) -> Vec<Token<'a>> {
        let mut lexer = Self::load(input);
        let mut buf = Vec::new();
        loop {
            let token = lexer.next();
            buf.push(token);
            if buf.last().unwrap().kind() == TokenKind::Eof {
                return buf;
            }
        }
    }

    /// Lex the next token
    fn lex_next(&mut self) -> Token<'a> {
        let i = &mut self.index;
        let bytes = self.input.as_bytes();

        // Skip whitespace
        while *i != bytes.len() && bytes[*i].is_ascii_whitespace() {
            *i += 1; // Consume whitespace
        }

        // Lex token
        let (kind, range) = if *i != bytes.len() {
            let start = *i;
            let uni_range = start..start + 1;

            *i += 1; // Consume one byte
            match bytes[*i - 1] {
                b'+' => ((TokenKind::Op(Op::Add), uni_range)),
                b'-' => ((TokenKind::Op(Op::Sub), uni_range)),
                b'*' => ((TokenKind::Op(Op::Mul), uni_range)),
                b'/' => ((TokenKind::Op(Op::Div), uni_range)),
                b'(' => ((TokenKind::Sep(Sep::Open), uni_range)),
                b')' => ((TokenKind::Sep(Sep::Close), uni_range)),
                b'#' => ((TokenKind::Sep(Sep::Comment), uni_range)),
                b'0'..=b'9' => {
                    let mut has_dot = false;
                    while *i != bytes.len() {
                        match bytes[*i] {
                            b'0'..=b'9' => *i += 1, // Consume digit
                            b'.' if !has_dot => {
                                has_dot = true;
                                *i += 1; // Consume dot one time
                            }
                            _ => break,
                        }
                    }
                    (TokenKind::Nb, start..*i)
                }
                b'a'..=b'z' => {
                    while *i != bytes.len() && bytes[*i].is_ascii_alphanumeric() {
                        *i += 1; // Consume alphanumeric
                    }
                    (TokenKind::Id, start..*i)
                }
                _ => ((TokenKind::Err, uni_range)),
            }
        } else {
            // No more token
            (TokenKind::Eof, 0..0)
        };
        Token::new(self.input, kind, range)
    }

    /// Return the next token moving forward
    pub fn next(&mut self) -> Token<'a> {
        self.peeked.take().unwrap_or_else(|| self.lex_next())
    }

    /// Return the next token without moving
    pub fn peek(&mut self) -> &Token<'a> {
        if let None = self.peeked {
            self.peeked = Some(self.lex_next());
        }
        self.peeked.as_ref().unwrap()
    }
}
