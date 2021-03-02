use crate::parser::{BinOp, Literal, Print, UnOp};
use std::fmt::Write;

pub fn compute_print(print: &[Print]) -> Result<String, String> {
    let mut buf = String::new();
    for item in print {
        match item {
            Print::Literal(lit) => write!(buf, "{}", compute_literal(&lit)?).unwrap(),
            Print::Str(str) => buf.push_str(&str),
        }
    }
    Ok(buf)
}

pub fn compute_literal(lit: &Literal) -> Result<f64, String> {
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
                _ => return Err(format!("Unknown function '{}'", name)),
            }
        }
        Literal::Var(name) => match name.as_str() {
            "PI" => std::f64::consts::PI,
            "E" => std::f64::consts::E,
            _ => return Err(format!("Unknown variable '{}'", name)),
        },
    })
}
