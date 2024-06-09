#[derive(Debug, Clone, Copy)]
pub enum Token {
    Number(f64),
    Add,
    Sub,
    Div,
    Mul,
    Mod,
    Pow,
}
