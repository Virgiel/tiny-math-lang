use std::ops::Range;

/// This is a pull lexer responsible for finding tokens in a code line.
/// It is designed to not allocate any memory.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Mod, // %
    Eq,  // =
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sep {
    Open,    // (
    Close,   // )
    Comment, // #
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Nb,       // f64 num
    Op(Op),   // Any operator
    Id,       // Sequence of supported char
    Str,      // Sequence of any char between "
    Sep(Sep), // Any separator
    Err,      // Unsupported char
    Eof,      // End of file
}

/// A code token
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

    // Currently used for end string missing " error, this is a design smell and should be removed
    pub fn after(&self) -> Token {
        Token::new(self.source, self.kind, self.span.end..self.span.end + 1)
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
    source: &'a str,
    offset: usize,
    peeked: Option<Token<'a>>,
}

impl<'a> Lexer<'a> {
    /// Init the lexer at the beginning of a source
    pub fn load(source: &'a str) -> Lexer {
        Lexer {
            source,
            offset: 0,
            peeked: None,
        }
    }

    /// Lex the next token
    fn lex_next(&mut self) -> Token<'a> {
        let chars = self.source.get(self.offset..).unwrap_or("").char_indices();

        // Skip whitespace
        let mut chars = chars.skip_while(|(_, c)| c.is_whitespace());

        // Lex token
        let (kind, range) = if let Some((i, c)) = chars.next() {
            let start = self.offset + i;
            let uni_range = start..start + 1;
            match c {
                '+' => ((TokenKind::Op(Op::Add), uni_range)),
                '-' => ((TokenKind::Op(Op::Sub), uni_range)),
                '*' => ((TokenKind::Op(Op::Mul), uni_range)),
                '/' => ((TokenKind::Op(Op::Div), uni_range)),
                '%' => ((TokenKind::Op(Op::Mod), uni_range)),
                '=' => ((TokenKind::Op(Op::Eq), uni_range)),
                '(' => ((TokenKind::Sep(Sep::Open), uni_range)),
                ')' => ((TokenKind::Sep(Sep::Close), uni_range)),
                '#' => ((TokenKind::Sep(Sep::Comment), uni_range)),
                '"' => {
                    // Search next "
                    let end = chars
                        .find(|(_, c)| *c == '"')
                        .map(|(i, _)| i + self.offset + 1)
                        .unwrap_or(self.source.len());
                    (TokenKind::Str, start..end)
                }
                c if c.is_ascii_digit() => {
                    let mut chars = chars.skip_while(|(_, c)| c.is_ascii_digit());

                    let end = match chars.next() {
                        Some((i, c)) => match c {
                            '.' => {
                                // The number is in two part, apply same logic for the second part
                                chars
                                    .find(|(_, c)| !c.is_ascii_digit())
                                    .map(|(i, _)| i + self.offset)
                                    .unwrap_or(self.source.len())
                            }
                            _ => i + self.offset, // End of number
                        },
                        // We have reach the end of the line
                        None => self.source.len(),
                    };
                    (TokenKind::Nb, start..end)
                }
                c if c.is_alphabetic() => {
                    // Search en of id
                    let end = chars
                        .find(|(_, c)| !c.is_alphanumeric())
                        .map(|(i, _)| i + self.offset)
                        .unwrap_or(self.source.len());
                    (TokenKind::Id, start..end)
                }
                _ => ((TokenKind::Err, start..self.source.len())),
            }
        } else {
            // No more token
            let len = self.source.len();
            (TokenKind::Eof, len..len)
        };
        self.offset = range.end; // Move forward
        return Token::new(self.source, kind, range);
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

    pub fn reset(&mut self) {
        self.offset = 0;
        self.peeked = None;
    }
}
