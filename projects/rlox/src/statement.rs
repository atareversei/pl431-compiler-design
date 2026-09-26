use crate::{expression::Expression, token::Token};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionStatement {
    pub name: Token,
    pub parameters: Vec<Token>,
    pub body: Option<Box<Statement>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    If {
        cond: Expression,
        body: Box<Statement>,
        elze: Option<Box<Statement>>,
    },
    Var {
        name: Token,
        initializer: Option<Expression>,
    },
    For {
        increment: Option<Expression>,
        cond: Expression,
        body: Box<Statement>,
    },
    Function(FunctionStatement),
    Return(Option<Expression>),
    Block(Vec<Statement>),
    Expression(Expression),
    Print(Expression), // TODO: move to standard library
    Break,
    Continue,
}
