use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Statement {
    LetStatement {
        variable: Identifier,
        initializer: Expression,
    },
    CallStatement {
        callee: Expression,
        function: Identifier,
        arguments: Vec<Expression>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Expression {
    Literal(Literal),
    Variable(Identifier),
    Tuple(Vec<Expression>),
    Array(Vec<Expression>),
    Option(Option<Box<Expression>>),
    Result(Result<Box<Expression>, Box<Expression>>),
    Vector(Vec<Expression>),
    Call {
        callee: Box<Expression>,
        function: Identifier,
        arguments: Vec<Expression>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Identifier {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Literal {
    Boolean(bool),
    Integer(i128, String),
    String(String),
    Address(String),
}
