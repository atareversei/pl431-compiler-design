use std::fmt;

use crate::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Ternary {
        condition: Box<Expression>,
        true_branch: Box<Expression>,
        false_branch: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Logical {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Call {
        callee: Box<Expression>,
        paren: Token,
        arguments: Vec<Expression>,
    },
    Assignment {
        name: Token,
        value: Box<Expression>,
    },
    Unary {
        operator: Token,
        right: Box<Expression>,
    },
    Comma {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Variable(Token),
    Grouping(Box<Expression>),
    Literal(LiteralValue),
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralValue {
    String(String),
    Number(f64),
    True,
    False,
    Null,
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralValue::String(s) => write!(f, "{}", s),
            LiteralValue::Number(n) => write!(f, "{}", n),
            LiteralValue::True => write!(f, "true"),
            LiteralValue::False => write!(f, "false"),
            LiteralValue::Null => write!(f, "null"),
        }
    }
}
