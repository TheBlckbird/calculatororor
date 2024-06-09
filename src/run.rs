use crate::ast::{BinOp, Expr, UnOp};

pub fn run(ast: Expr) -> f64 {
    eval_expr(ast, 0.0)
}

fn eval_expr(expr: Expr, result: f64) -> f64 {
    match expr {
        Expr::BinOp { left, right, op } => match op {
            BinOp::Add => eval_expr(*left, result) + eval_expr(*right, result),
            BinOp::Sub => eval_expr(*left, result) - eval_expr(*right, result),
            BinOp::Mul => eval_expr(*left, result) * eval_expr(*right, result),
            BinOp::Div => eval_expr(*left, result) / eval_expr(*right, result),
            BinOp::Mod => eval_expr(*left, result) % eval_expr(*right, result),
            BinOp::Pow => eval_expr(*left, result).powf(eval_expr(*right, result)),
        },
        Expr::UnOp(expr, op) => match op {
            UnOp::Neg => -eval_expr(*expr, result),
        },
        Expr::Number(num) => num,
    }
}
