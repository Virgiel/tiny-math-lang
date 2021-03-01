use lexer::{Lexer, Sep, TokenKind};
use parser::{BinOp, Line, Literal, Print, UnOp, parse, Expression};
use std::fmt::Write;

mod lexer;
mod parser;

pub fn exec_batch(input: &str) -> Vec<Result<String, String>> {
    input.lines().map(|line| exec_line(line)).collect()
}

pub fn exec_line(input: &str) -> Result<String, String> {
    let lexer = Lexer::load(input);
    Ok(match parse(lexer)? {
        Line::Expr(expr) => 
            match expr {
                Expression::Literal(lit)=> 
            format!("<span class=\"variable\">λ</span> <span class=\"operator\">=<span> <span class=\"number\">{}</span>",  compute_literal(&lit)?),
            Expression::Print(print) =>  
            format!("<span class=\"string\">{}</span>",  compute_print(&print)?),

            }
        Line::Empty | Line::Comment(_) => "".to_string(),
    })
}

pub fn highlight(input: &str) -> String {
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
                TokenKind::Id => {
                    write!(buf, "<span class=\"function\">{}</span>", token.splice()).unwrap()
                }
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

fn compute_print(print: &[Print]) -> Result<String, String> {
   let mut buf = String::new();
    for item in print {
       match item {
           Print::Literal(lit) => write!(buf, "{}", compute_literal(&lit)?).unwrap(),
           Print::Str(str) => buf.push_str(&str)
       } 
    }
   Ok(buf)

}

fn compute_literal(lit: &Literal) -> Result<f64, String> {
    Ok(match lit {
        Literal::Nb(nb) => *nb,
        Literal::UnaryOp(op, lit) => {
            let nb = compute_literal(lit)?;
            match op {
                UnOp::Add => nb,
                UnOp::Sub => -nb,
            }
        }
        Literal::BinaryOp(op, lits) => {
            let (l, r) = (compute_literal(&lits.0)?, compute_literal(&lits.1)?);
            match op {
                BinOp::Add => l + r,
                BinOp::Sub => l - r,
                BinOp::Mul => l * r,
                BinOp::Div => l / r,
                BinOp::Mod => l % r,
            }
        }
        Literal::Fun(name, lit) => {
            let nb = compute_literal(lit)?;
            match name.as_str() {
                "floor" => nb.floor(),
                "ceil" => nb.ceil(),
                "round" => nb.round(),
                "trunc" => nb.trunc(),
                "fract" => nb.fract(),
                "sqrt" => nb.sqrt(),
                "exp" => nb.exp(),
                "ln" => nb.ln(),
                "log2" => nb.log2(),
                "log10" => nb.log10(),
                "cos" => nb.cos(),
                "sin" => nb.sin(),
                "tan" => nb.tan(),
                "acos" => nb.acos(),
                "asin" => nb.asin(),
                "atan" => nb.atan(),
                _ => return Err(format!("Unknown function {}", name)),
            }
        }
    })
}

#[cfg(test)]
mod test {
    use crate::compute_literal;
    use crate::parse;
    use crate::Lexer;
    use crate::{exec_line, highlight, parser::Line};
    use crate::parser::Expression;

    fn assert_compute(str: &str, nb: f64) {
        let parsed = parse(Lexer::load(str));
        assert!(parsed.is_ok(), "{:?}", parsed);
        let expr = match parsed.unwrap() {
            Line::Expr(it) => it,
            _ => unreachable!(),
        };
        let lit = match expr {
           Expression::Literal(lit) => lit,
           _ => unreachable!() 
        };
        let result = compute_literal(&lit);
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!(result.unwrap(), nb)
    }

    fn assert_fail(str: &str) {
        assert!(exec_line(str).is_err(), "{:?}", exec_line(str))
    }

    #[test]
    fn test_parse_nb() {
        assert_compute("1", 1.);
        assert_compute("12345", 12345.);
        assert_compute("12345.", 12345.);
        assert_compute("123.45", 123.45);
    }

    #[test]
    fn test_parse_fun() {
        assert_compute("cos(0.25)", 0.25f64.cos());
        assert_compute("sin(0.25)", 0.25f64.sin());
    }

    #[test]
    fn test_op() {
        assert_compute("2+2", 4.);
        assert_compute("2-3", -1.);
        assert_compute("+2", 2.);
        assert_compute("-2", -2.);
        assert_compute("2*3", 6.);
        assert_compute("10/2", 5.);
        assert_compute("100%7", 2.);
    }

    #[test]
    fn test_order() {
        assert_compute("2*3+3*4", 18.);
        assert_compute("2*(3*3)*4", 72.);
        assert_compute("2+3*3+4", 15.);
        assert_compute("(2+3)*(3+4)", 35.);
    }

    #[test]
    fn test_error() {
        assert_fail("test");
        assert_fail("test(");
        assert_fail("test()");
        assert_fail("3.4.5");
        assert_fail(".5");
        assert_fail("2*3*3)*4");
        assert_fail("2*(3*3*4");
        assert_fail("*4");
        assert_fail("/1");
        assert_fail("/1#");
    }

    #[test]
    fn test_lines() {
        assert_eq!(exec_line("").unwrap(), "");
        assert_eq!(exec_line("# I love chocolate").unwrap(), "");
    }

    #[test]
    fn test_support_unicode() {
        highlight("1+1°");
        highlight("あさきゆめみしゑひもせす");
    }
}
