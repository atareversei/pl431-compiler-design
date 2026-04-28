use crate::{expression::Expression, token::Token};

#[derive(Debug, PartialEq)]
pub enum Statement {
    Var {
        name: Token,
        initializer: Expression,
    },
    Expression(Expression),
    Print(Expression), // TODO: move to standard library
}
