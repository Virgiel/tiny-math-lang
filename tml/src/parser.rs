use crate::{
    lexer::{Lexer, TokenKind},
    Op, Sep,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Line {
    Expr(Expr),
    Comment(usize),
    Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Nb(f64),
    UnaryOp(Op, Box<Expr>),
    BinaryOp(Op, Box<(Expr, Expr)>),
    Fun(String, Box<Expr>),
}

pub fn parse(mut lexer: Lexer) -> Result<Line, String> {
    let peek = lexer.peek();
    if peek.kind() == TokenKind::Eof {
        Ok(Line::Empty)
    } else if peek.kind() == TokenKind::Sep(Sep::Comment) {
        Ok(Line::Comment(peek.span().start))
    } else {
        let expr = parser_expr(&mut lexer, 0)?;
        let next = lexer.next();
        if next.kind() != TokenKind::Eof {
            return Err(next.err_there("Invalid expression"));
        }
        Ok(Line::Expr(expr))
    }
}

fn parser_expr(lexer: &mut Lexer, min_bp: u8) -> Result<Expr, String> {
    let token = lexer.next();
    let mut lhs = match token.kind() {
        TokenKind::Nb => match token.splice().parse::<f64>() {
            Ok(nb) => Expr::Nb(nb),
            Err(_) => return Err(token.err_there("Invalid Number")),
        },
        TokenKind::Sep(Sep::Open) => {
            let lhs = parser_expr(lexer, 0)?;
            let next = lexer.next();
            if next.kind() != TokenKind::Sep(Sep::Close) {
                return Err(next.err_there("An opened block miss its end, a ')' is missing"));
            }
            lhs
        }
        TokenKind::Id => {
            let id = token.splice().into();
            let next = lexer.next();
            if next.kind() != TokenKind::Sep(Sep::Open) {
                return Err(
                    next.err_there("A function invocation miss its arguments, a '(' is missing")
                );
            }
            let expr = parser_expr(lexer, 0)?;
            let next = lexer.next();
            if next.kind() != TokenKind::Sep(Sep::Close) {
                return Err(next.err_there("An function invocation miss its end, a ')' is missing"));
            }
            Expr::Fun(id, Box::new(expr))
        }
        TokenKind::Op(op) => {
            let hs = parser_expr(lexer, prefix_binding_power(op))?;
            Expr::UnaryOp(op, Box::new(hs))
        }
        _ => return Err(token.err_there("Incomplete expression")),
    };

    loop {
        let op = match lexer.peek().kind() {
            TokenKind::Op(op) => op,
            _ => break,
        };

        let bp = infix_binding_power(op);
        if bp < min_bp {
            break;
        }
        lexer.next();

        let rhs = parser_expr(lexer, bp)?;
        lhs = Expr::BinaryOp(op, Box::new((lhs, rhs)))
    }

    Ok(lhs)
}

fn prefix_binding_power(op: Op) -> u8 {
    match op {
        Op::Add | Op::Sub => 3,
        _ => 0,
    }
}

fn infix_binding_power(op: Op) -> u8 {
    match op {
        Op::Add | Op::Sub => 1,
        Op::Mul | Op::Div | Op::Mod => 2,
    }
}
