
use rug::{ Integer, Float };

/// Precision of floats read into Micron
pub const FLOAT_PRECISION: u32 = 53;

#[derive(Debug)]
pub enum Statement {

    Assignment(String, Box<Expr>),
    StringAssignment(String, Box<StringExpr>),
    BareExpression(Box<Expr>)
}

#[derive(Debug)]
pub enum StringExpr {
    String(String),
    Number(Integer),
    Real(Float),
    Variable(String),
    Op(Box<StringExpr>, StringOp, Box<StringExpr>)

    // Variable(String) // For Concatenation
    // Number(Integer)  // For Duplication
    // Concatenate(Box<StringExpr>, StringOp Box<StringExpr>)
    // Bracket(Box<StringExpr>, StringUnaryOp)
}

#[derive(Clone, Debug)]
pub enum StringOp {
    Concatenate,         // Concatenate
    Mul,                // Duplicate - Like python
}

/*

pub enum StringUnaryOp {

    Bracket(Integer)    // my_str[3]
}
*/

/*

pub enum DictExpr {


}

*/



#[derive(Debug)]
pub enum Expr {
    Number(Integer),
    Real(Float),
    Variable(String),
    Op(Box<Expr>, Opcode, Box<Expr>),
    UnaryOp(Box<Expr>, UnaryOpcode)
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

#[derive(Debug)]
pub enum UnaryOpcode {
    Negate,
    BwNot
}