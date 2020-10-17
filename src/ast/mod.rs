
#[derive(Debug)]
pub enum Statement {

    Assignment(String, Box<Expr>),
    BareExpression(Box<Expr>)
}

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Variable(String),
    Op(Box<Expr>, Opcode, Box<Expr>),
}

#[derive(Debug)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}