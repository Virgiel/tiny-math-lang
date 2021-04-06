use crate::{
    lexer::Lexer,
    parser::{parse, BinOp, Expression, Line, Literal, Print, UnOp},
};
use std::{collections::HashMap, fmt::Write};

/** Execution context */
pub struct Context {
    variables: HashMap<String, f64>,
}

impl Context {
    pub fn empty() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    fn assign(&mut self, id: String, nb: f64) {
        self.variables.insert(id, nb);
    }

    fn get(&self, id: &str) -> Option<&f64> {
        self.variables.get(id)
    }
}

/** Compute a line, returning a formatted result */
pub fn compute(ctx: &mut Context, input: &str) -> Result<String, String> {
    let lexer = Lexer::load(input);
    let result = match parse(lexer)? {
        Line::Expr(expr) => match expr {
            Expression::Assign(id, lit) => {
                let nb = compute_literal(ctx, &lit)?;
                ctx.assign(id.into(), nb);
                format!("{} = {}", id, nb)
            }
            Expression::Literal(lit) => {
                let nb = compute_literal(ctx, &lit)?;
                ctx.assign("$".to_string(), nb);
                nb.to_string()
            }
            Expression::Print(print) => compute_print(ctx, &print)?,
        },
        Line::Empty | Line::Comment(_) => "".into(),
    };
    Ok(result)
}

/** Compute a print expression, concatenate raw string with literal expression result */
fn compute_print(ctx: &mut Context, print: &[Print]) -> Result<String, String> {
    let mut buf = String::from("\"");
    for item in print {
        match item {
            Print::Literal(lit) => write!(buf, "{}", compute_literal(ctx, &lit)?).unwrap(),
            Print::Str(str) => buf.push_str(&str),
        }
    }
    buf.push('\"');
    Ok(buf)
}

/** Compute a literal expression, perform calculation */
fn compute_literal(ctx: &mut Context, lit: &Literal) -> Result<f64, String> {
    Ok(match lit {
        Literal::Nb(nb) => *nb,
        Literal::UnaryOp(op, lit) => {
            let nb = compute_literal(ctx, lit)?;
            match op {
                UnOp::Add => nb,
                UnOp::Sub => -nb,
            }
        }
        Literal::BinaryOp(op, lits) => {
            let (l, r) = (
                compute_literal(ctx, &lits.0)?,
                compute_literal(ctx, &lits.1)?,
            );
            match op {
                BinOp::Add => l + r,
                BinOp::Sub => l - r,
                BinOp::Mul => l * r,
                BinOp::Div => l / r,
                BinOp::Mod => l % r,
                BinOp::Pow => l.powf(r),
            }
        }
        Literal::Fun(name, lit) => {
            let nb = compute_literal(ctx, lit)?;
            match name.as_ref() {
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
                _ => return Err(format!("Unknown function '{}'", name)),
            }
        }
        Literal::Var(id) => match id.as_ref() {
            "PI" => std::f64::consts::PI,
            "E" => std::f64::consts::E,
            _ => match ctx.get(id) {
                Some(nb) => *nb,
                None => return Err(format!("Unknown variable '{}'", id)),
            },
        },
    })
}

#[cfg(test)]
mod test {
    use crate::interpreter::{compute, compute_literal, compute_print, Context};
    use crate::lexer::Lexer;
    use crate::parser::parse;
    use crate::parser::Expression;
    use crate::parser::Line;
    use proptest::prelude::*;

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
        let result = compute_literal(&mut Context::empty(), &lit);
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
        let result = compute_print(&mut Context::empty(), &print);
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!(result.unwrap(), expected)
    }

    fn assert_fail(str: &str) {
        let mut context = Context::empty();
        let result = compute(&mut context, str);
        assert!(result.is_err(), "{:?}", result)
    }

    fn compute_no_context(str: &str) -> Result<String, String> {
        let mut context = Context::empty();
        compute(&mut context, str)
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
        assert_compute("10^2", 100.);
        assert_compute("2*10^2", 200.);
        assert_compute("2*10^2*2", 400.);
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
        assert_eq!(compute_no_context("").unwrap(), "");
        assert_eq!(compute_no_context("# I love chocolate").unwrap(), "");
    }

    #[test]
    fn test_print() {
        assert_print("\"I Love Chocolate\"", "\"I Love Chocolate\"");
        assert_print("   \"I Love Chocolate\"  ", "\"I Love Chocolate\"");
        assert_print("\"I am \"18\" year old\"  ", "\"I am 18 year old\"");
        assert_print("\"I am \"18\" year old\"42", "\"I am 18 year old42\"");
        assert_print(
            "\"A\"\"B\"42\"C\"log2(345)+(5/9)*19-2\"Chocolate\"",
            "\"AB42C16.98600810722109Chocolate\"",
        );
    }

    prop_compose! {
        fn arb_nb()(nb in any::<u8>(), op in "[+-]?") -> String {
            format!("{}{}", op, nb)
        }
    }
    prop_compose! {
        fn arb_var()(var in "PI|E", op in "[+-]?") -> String {
            format!("{}{}", op, var)
        }
    }
    fn arb_lit() -> impl Strategy<Value = String> {
        let leaf = prop_oneof![
            4 => arb_nb(),
            1 => arb_var(),
        ];
        leaf.prop_recursive(10, 1000, 1, |inner| {
            prop_oneof![
                1 => inner.clone().prop_map(|lit| format!("({})", lit)),
                4 => (inner.clone(), inner.clone(), "[+/-/*//%]")
                    .prop_map(|(a, b, op)| format!("{}{}{}", a, op, b)),
                2 => (inner, "floor|ceil|round|trunc|fract|sqrt|exp|ln|log2|log10|cos|sin|tan|acos|asin|atan").prop_map(|(lit, fun)| format!("{}({})", fun, lit))
            ]
        })
    }
    proptest! {
        #[test]
        fn execute_anything(s: String) {
            compute_no_context(&s).ok();
        }

        #[test]
        fn execute_lit(nb in arb_lit()) {
        let parsed = parse(Lexer::load(&nb));
        assert!(parsed.is_ok(), "{:?}", parsed);
        let expr = match parsed.unwrap() {
            Line::Expr(it) => it,
            _ => unreachable!(),
        };
        let lit = match expr {
            Expression::Literal(lit) => lit,
            _ => unreachable!(),
        };
        let result = compute_literal(&mut Context::empty(), &lit);
        assert!(result.is_ok(), "{:?}", result);
        }
    }
}
