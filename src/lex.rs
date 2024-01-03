use crate::ir::*;

#[test]
fn test() {
    dbg!(lexing("1 + 2 * 3 / 4 % 5"));
}

pub fn lexing(mut src: &str) -> Box<dyn Expr> {
    // 0. remove spaces at both ends
    src = src.trim();

    // 1. try whole recognize as int
    if let Ok(val) = src.parse::<usize>() {
        return Box::new(Int { val });
    }

    // 2. try find b-op from right, by low-priorities
    //    op splitting src as longer at lhs has higter priority
    for ops in [&["+", "-"][..], &["*", "/"][..], &["%"][..]] {
        let result = ops
            .iter()
            .filter_map(|op| src.rsplit_once(op).map(|lr| (*op, lr)))
            .max_by_key(|(_, (lhs, _))| lhs.len())
            .map(|(op, (lhs, rhs))| b_op(op)(lexing(lhs), lexing(rhs)));

        if let Some(expr) = result {
            return expr;
        }
    }

    // x. failed parseing
    panic!("compile error: can't parse expression `{src}`");
}

#[allow(clippy::type_complexity)]
fn b_op(op: &str) -> Box<dyn Fn(Box<dyn Expr>, Box<dyn Expr>) -> Box<dyn Expr>> {
    match op {
        "+" => Box::new(|lhs, rhs| Box::new(Add { lhs, rhs })),
        "-" => Box::new(|lhs, rhs| Box::new(Sub { lhs, rhs })),

        "*" => Box::new(|lhs, rhs| Box::new(Mul { lhs, rhs })),
        "/" => Box::new(|lhs, rhs| Box::new(Div { lhs, rhs })),

        "%" => Box::new(|lhs, rhs| Box::new(Mdl { lhs, rhs })),

        _ => unreachable!("internal compiler error"),
    }
}
