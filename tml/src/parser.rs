use std::convert::{TryFrom, TryInto};

use crate::lexer::{Lexer, Op, Sep, Token, TokenKind};

#[derive(Debug, Clone, PartialEq)]
pub enum Line {
    Expr(Expression),
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
            _ => return Err("Expected an unary operator such as + or -"),
        };
        Ok(op)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Print(Vec<Print>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Nb(f64),
    UnaryOp(UnOp, Box<Literal>),
    BinaryOp(BinOp, Box<(Literal, Literal)>),
    Fun(String, Box<Literal>),
    Var(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Print {
    Literal(Literal),
    Str(String),
}

pub fn parse(mut lexer: Lexer) -> Result<Line, String> {
    let peek = lexer.peek();
    let line = match peek.kind() {
        TokenKind::Sep(Sep::Comment) => Line::Comment(peek.span().start),
        TokenKind::Eof => Line::Empty,
        _ => {
            let expr = match peek.kind() {
                TokenKind::Str => Expression::Print(parse_print(&mut lexer)?),
                _ => Expression::Literal(parser_literal(&mut lexer, 0)?),
            };
            expect_kind(lexer.next(), TokenKind::Eof, "Incomplete expression")?;
            Line::Expr(expr)
        }
    };
    Ok(line)
}

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

fn parse_print(lexer: &mut Lexer) -> Result<Vec<Print>, String> {
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
                Print::Str(lexer.next().splice().trim_matches('"').to_string())
            }
            TokenKind::Eof => return Ok(buf),
            _ => Print::Literal(parser_literal(lexer, 0)?),
        });
    }
}

fn parser_literal(lexer: &mut Lexer, min_bp: u8) -> Result<Literal, String> {
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
        if bp < min_bp {
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
    }
}
