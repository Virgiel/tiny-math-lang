use lexer::Lexer;
use parser::{parse, Expression, Line};

mod highlighter;
mod interpreter;
mod lexer;
mod parser;

pub fn exec_batch(input: &str) -> Vec<Result<String, String>> {
    input.lines().map(|line| exec_line(line)).collect()
}

pub fn exec_line(input: &str) -> Result<String, String> {
    let lexer = Lexer::load(input);
    Ok(match parse(lexer)? {
        Line::Expr(expr) => match expr {
            Expression::Literal(lit) => {
                highlighter::highlight_no_alias(interpreter::compute_literal(&lit)?)
            }
            Expression::Print(print) => {
                highlighter::highlight_print(&interpreter::compute_print(&print)?)
            }
        },
        Line::Empty | Line::Comment(_) => "".to_string(),
    })
}

pub fn highlight(input: &str) -> String {
    highlighter::highlight_code(input)
}

#[cfg(test)]
mod test {
    use crate::interpreter::{compute_literal, compute_print};
    use crate::parse;
    use crate::parser::Expression;
    use crate::Lexer;
    use crate::{exec_line, highlighter::highlight_code, parser::Line};

    fn assert_compute(str: &str, nb: f64) {
        let parsed = parse(Lexer::load(str));
        assert!(parsed.is_ok(), "{:?}", parsed);
        let expr = match parsed.unwrap() {
            Line::Expr(it) => it,
            _ => unreachable!(),
        };
        let lit = match expr {
            Expression::Literal(lit) => lit,
            _ => unreachable!(),
        };
        let result = compute_literal(&lit);
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!(result.unwrap(), nb)
    }

    fn assert_print(str: &str, expected: &str) {
        let parsed = parse(Lexer::load(str));
        assert!(parsed.is_ok(), "{:?}", parsed);
        let expr = match parsed.unwrap() {
            Line::Expr(it) => it,
            _ => unreachable!(),
        };
        let print = match expr {
            Expression::Print(print) => print,
            _ => unreachable!(),
        };
        let result = compute_print(&print);
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!(result.unwrap(), expected)
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
        assert_compute("log10(100)", 2.);
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
    fn test_var() {
        assert_compute("cos(PI)", -1.);
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
        assert_fail("\"");
        assert_fail("12 \"nop\"");
        assert_fail("\"nop");
        assert_fail("\"12\" 34 \"nop");
    }

    #[test]
    fn test_lines() {
        assert_eq!(exec_line("").unwrap(), "");
        assert_eq!(exec_line("# I love chocolate").unwrap(), "");
    }

    #[test]
    fn test_print() {
        assert_print("\"I Love Chocolate\"", "I Love Chocolate");
        assert_print("   \"I Love Chocolate\"  ", "I Love Chocolate");
        assert_print("\"I am \"18\" year old\"  ", "I am 18 year old");
        assert_print("\"I am \"18\" year old\"42", "I am 18 year old42");
        assert_print(
            "\"A\"\"B\"42\"C\"log2(345)+(5/9)*19-2\"Chocolate\"",
            "AB42C16.98600810722109Chocolate",
        );
    }

    #[test]
    fn test_support_unicode() {
        highlight_code("1+1°");
        highlight_code("あさきゆめみしゑひもせす");
    }
}
