use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Mod, // %
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
    Str,      // Sequence of any char between """
    Sep(Sep), // Any separator
    Err,      // Unsupported char
    Eof,      // End of file
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

    /// Lex the next token
    fn lex_next(&mut self) -> Token<'a> {
        let chars = self.input.get(self.index..).unwrap_or("").char_indices();

        // Skip whitespace
        let mut chars = chars.skip_while(|(_, c)| c.is_whitespace());

        // Lex token
        let (kind, range) = if let Some((i, c)) = chars.next() {
            let start = self.index + i;
            let uni_range = start..start + 1;
            match c {
                '+' => ((TokenKind::Op(Op::Add), uni_range)),
                '-' => ((TokenKind::Op(Op::Sub), uni_range)),
                '*' => ((TokenKind::Op(Op::Mul), uni_range)),
                '/' => ((TokenKind::Op(Op::Div), uni_range)),
                '%' => ((TokenKind::Op(Op::Mod), uni_range)),
                '(' => ((TokenKind::Sep(Sep::Open), uni_range)),
                ')' => ((TokenKind::Sep(Sep::Close), uni_range)),
                '#' => ((TokenKind::Sep(Sep::Comment), uni_range)),
                '"' => {
                    let end = chars
                        .find(|(_, c)| *c == '"')
                        .map(|(i, _)| i + self.index + 1)
                        .unwrap_or(self.input.len());
                    (TokenKind::Str, start..end)
                }
                c if is_nb(c) => {
                    let mut chars = chars.skip_while(|(_, char)| is_nb(*char));

                    let end = match chars.next() {
                        Some((i, c)) => match c {
                            '.' => {
                                // The number is in two part, apply same logic for the second part
                                chars
                                    .find(|(_, c)| !is_nb(*c))
                                    .map(|(i, _)| i + self.index)
                                    .unwrap_or(self.input.len())
                            }
                            _ => i + self.index, // End of number
                        },
                        // We have reach the end of the line
                        None => self.input.len(),
                    };
                    (TokenKind::Nb, start..end)
                }
                c if is_id_init(c) => {
                    let end = chars
                        .find(|(_, c)| !is_id_content(*c))
                        .map(|(i, _)| i + self.index)
                        .unwrap_or(self.input.len());
                    (TokenKind::Id, start..end)
                }
                _ => ((TokenKind::Err, start..self.input.len())),
            }
        } else {
            // No more token
            let len = self.input.len();
            (TokenKind::Eof, len..len)
        };
        self.index = range.end;
        return Token::new(self.input, kind, range);
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

pub fn is_nb(c: char) -> bool {
    matches!(c, '0'..='9')
}

pub fn is_id_init(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_')
}

pub fn is_id_content(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_')
}
