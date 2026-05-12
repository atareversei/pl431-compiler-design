use crate::{expression::Expression, token::Token};

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Var {
        name: Token,
        initializer: Option<Expression>,
    },
    Block(Vec<Statement>),
    Expression(Expression),
    Print(Expression), // TODO: move to standard library
}
