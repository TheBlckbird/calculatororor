#[derive(Debug)]
pub enum Expr {
    BinOp {
        left: Box<Expr>,
        right: Box<Expr>,
        op: BinOp,
    },
    UnOp(Box<Expr>, UnOp),
    Number(f64),
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
}

#[derive(Debug)]
pub enum UnOp {
    Neg,
}
