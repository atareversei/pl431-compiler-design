use crate::expression::Expression;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Expression(Expression),
    Print(Expression), // TODO: move to standard library
}
