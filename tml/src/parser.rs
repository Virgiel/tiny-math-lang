use std::convert::{TryFrom, TryInto};

use crate::lexer::{Lexer, Op, Sep, Token, TokenKind};

/** The parser is responsible to line into usable type. The design is inspired by the following
excellent article: https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html  */

#[derive(Debug, Clone, PartialEq)]
pub enum Line<'a> {
    Expr(Expression<'a>),
    Comment(usize),
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnOp {
    Add, // +x
    Sub, // -x
}

impl TryFrom<Op> for UnOp {
    type Error = &'static str;

    fn try_from(op: Op) -> Result<Self, Self::Error> {
        let op = match op {
            Op::Add => UnOp::Add,
            Op::Sub => UnOp::Sub,
            _ => return Err("Expected an unary operator such as + or -"),
        };
        Ok(op)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BinOp {
    Add, // x+x
    Sub, // x-x
    Mul, // x*x
    Div, // x/x
    Mod, // x%x
    Pow, // x^x
}

impl TryFrom<Op> for BinOp {
    type Error = &'static str;

    fn try_from(op: Op) -> Result<Self, Self::Error> {
        let op = match op {
            Op::Add => BinOp::Add,
            Op::Sub => BinOp::Sub,
            Op::Mul => BinOp::Mul,
            Op::Div => BinOp::Div,
            Op::Mod => BinOp::Mod,
            Op::Pow => BinOp::Pow,
            _ => return Err("Expected an binary operator such as +, -, *, *, % or ^"),
        };
        Ok(op)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a> {
    Assign(&'a str, Literal<'a>),
    Literal(Literal<'a>),
    Print(Vec<Print<'a>>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal<'a> {
    Nb(f64),
    UnaryOp(UnOp, Box<Literal<'a>>),
    BinaryOp(BinOp, Box<(Literal<'a>, Literal<'a>)>),
    Fun(&'a str, Box<Literal<'a>>),
    Var(&'a str),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Print<'a> {
    Literal(Literal<'a>),
    Str(&'a str),
}

/** Parse a line from tokens */
pub fn parse<'a>(mut lexer: Lexer<'a>) -> Result<Line<'a>, String> {
    let peek = lexer.peek();
    let line = match peek.kind() {
        TokenKind::Sep(Sep::Comment) => Line::Comment(peek.span().start),
        TokenKind::Eof => Line::Empty,
        _ => {
            let expr = match peek.kind() {
                TokenKind::Str => Expression::Print(parse_print(&mut lexer)?),
                TokenKind::Id => {
                    let id = lexer.next().splice();
                    match lexer.next().kind() {
                        TokenKind::Op(Op::Eq) => {
                            Expression::Assign(id, parser_literal(&mut lexer, 0)?)
                        }
                        _ => {
                            lexer.reset();
                            Expression::Literal(parser_literal(&mut lexer, 0)?)
                        }
                    }
                }
                _ => Expression::Literal(parser_literal(&mut lexer, 0)?),
            };
            expect_kind(lexer.next(), TokenKind::Eof, "Incomplete expression")?;
            Line::Expr(expr)
        }
    };
    Ok(line)
}

/** Check token's kind */
fn expect_kind<'a, 'b>(
    token: Token<'a>,
    kind: TokenKind,
    msg: &'b str,
) -> Result<Token<'a>, String> {
    if token.kind() != kind {
        return Err(token.err_there(msg));
    } else {
        return Ok(token);
    }
}

/** Parse a print parts from tokens */
fn parse_print<'a>(lexer: &mut Lexer<'a>) -> Result<Vec<Print<'a>>, String> {
    let mut buf = Vec::new();
    loop {
        let token = lexer.peek();

        buf.push(match token.kind() {
            TokenKind::Str => {
                if !token.splice().ends_with('"') || token.splice().len() < 2 {
                    return Err(token
                        .after()
                        .err_there("Missing string end, '\"' is missing"));
                }
                Print::Str(lexer.next().splice().trim_matches('"'))
            }
            TokenKind::Eof => return Ok(buf),
            _ => Print::Literal(parser_literal(lexer, 0)?),
        });
    }
}

/** Parse a literal from tokens */
fn parser_literal<'a>(lexer: &mut Lexer<'a>, min_bp: u8) -> Result<Literal<'a>, String> {
    let token = lexer.next();
    let mut lhs = match token.kind() {
        TokenKind::Nb => match token.splice().parse::<f64>() {
            Ok(nb) => Literal::Nb(nb),
            Err(_) => return Err(token.err_there("Invalid Number")),
        },
        TokenKind::Sep(Sep::Open) => {
            let lhs = parser_literal(lexer, 0)?;
            expect_kind(
                lexer.next(),
                TokenKind::Sep(Sep::Close),
                "Missing block end ')'",
            )?;
            lhs
        }
        TokenKind::Id => {
            let id = token.splice().into();
            let peek = lexer.peek();
            if peek.kind() == TokenKind::Sep(Sep::Open) {
                lexer.next();

                let expr = parser_literal(lexer, 0)?;
                expect_kind(
                    lexer.next(),
                    TokenKind::Sep(Sep::Close),
                    "Missing function invocation end ')'",
                )?;
                Literal::Fun(id, Box::new(expr))
            } else {
                Literal::Var(id)
            }
        }
        TokenKind::Op(op) => match op.try_into() {
            Ok(op) => {
                let hs = parser_literal(lexer, prefix_binding_power(op))?;
                Literal::UnaryOp(op, Box::new(hs))
            }
            Err(err) => return Err(token.err_there(err)),
        },
        _ => return Err(token.err_there("Incomplete expression")),
    };

    loop {
        let op = match lexer.peek().kind() {
            TokenKind::Op(op) => match op.try_into() {
                Ok(op) => op,
                Err(_) => break,
            },
            _ => break,
        };

        let bp = infix_binding_power(op);
        if bp <= min_bp {
            break;
        }
        lexer.next();

        let rhs = parser_literal(lexer, bp)?;
        lhs = Literal::BinaryOp(op, Box::new((lhs, rhs)))
    }

    Ok(lhs)
}

fn prefix_binding_power(op: UnOp) -> u8 {
    match op {
        UnOp::Add | UnOp::Sub => 3,
    }
}

fn infix_binding_power(op: BinOp) -> u8 {
    match op {
        BinOp::Add | BinOp::Sub => 1,
        BinOp::Mul | BinOp::Div | BinOp::Mod => 2,
        BinOp::Pow => 3,
    }
}
