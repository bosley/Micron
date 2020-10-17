use rug::Integer;

#[derive(Debug)]
pub enum Statement {

    Assignment(String, Box<Expr>),
    BareExpression(Box<Expr>)
}

#[derive(Debug)]
pub enum Expr {
    Number(Integer),
    Variable(String),
    Op(Box<Expr>, Opcode, Box<Expr>),
}

#[derive(Debug)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
    Lte,
    Gte,
    Gt,
    Lt,
    Equal,
    Ne,
    Pow,
    Mod,
    Lsh,
    Rsh,
    BwXor,
    BwOr,
    BwAnd,
    Or,
    And
}