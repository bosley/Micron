use rug::{ Integer, Float };

/// Precision of floats read into Micron
pub const FLOAT_PRECISION: u32 = 53;
pub const RADIX: i32 = 10;

#[derive(Debug, Clone)]
pub enum VariableType {
   Singular(String),
   Nested(String, Vec<DictAccessType>)
}

#[derive(Debug, Clone)]
pub enum Statement {

    ScopedStatementBlock(Vec<Box<Statement>>),
    Assignment(VariableType, Box<Expr>),
    BareExpression(Box<Expr>),
    Yield(Box<Expr>)
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(Integer),
    Real(Float),
    String(String),
    Variable(VariableType),

    Op(Box<Expr>, Opcode, Box<Expr>),
    UnaryOp(Box<Expr>, UnaryOpcode),


    BuiltInModifierCall(String, String),

    Access(Box<Expr>, Accessors, Box<MemberMethod>),

    // { .. }
    Dict(Vec<Box<DictEntry>>),

    IfExpression(Box<Vec<ConditionalBlock>>),

    FunctionExpression(Box<FunctionDefinition>),
}


#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum UnaryOpcode {
    Negate,
    BwNot
}

#[derive(Debug, Clone)]
pub enum DictAccessType {
    RawValue(String),
    Variable(String)
}

#[derive(Debug, Clone)]
pub enum Accessors {
    Dot,    // .
    Eyes    // ::
}

#[derive(Debug, Clone)]
pub struct MemberMethod {
    pub method: String,
    pub params: Vec<Box<Expr>>
}

#[derive(Debug, Clone)]
pub struct DictEntry {
    pub key: String,
    pub value: Box<Expr>
}

#[derive(Debug, Clone)]
pub struct ConditionalBlock {
    pub expression: Option<Box<Expr>>,
    pub body:       Vec<Box<Statement>>
}

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub params: Vec<String>,
    pub body:   Vec<Box<Statement>>
}