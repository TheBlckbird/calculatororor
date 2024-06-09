use crate::ast::{BinOp, Expr, UnOp};

fn run(expr: Expr) -> f64 {
    match expr {
        Expr::BinOp { left, right, op } => match op {
            BinOp::Add => run(*left) + run(*right),
            BinOp::Sub => run(*left) - run(*right),
            BinOp::Mul => run(*left) * run(*right),
            BinOp::Div => run(*left) / run(*right),
            BinOp::Mod => run(*left) % run(*right),
            BinOp::Pow => run(*left).powf(run(*right)),
        },
        Expr::UnOp(expr, op) => match op {
            UnOp::Neg => -run(*expr),
        },
        Expr::Number(num) => num,
    }
}
