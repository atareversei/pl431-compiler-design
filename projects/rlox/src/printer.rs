use std::fmt::format;

use crate::expression::{Expression, ExpressionKind};

pub fn ast(expr: &Expression) -> String {
    match &expr.kind {
        ExpressionKind::Ternary {
            condition,
            true_branch,
            false_branch,
        } => format!(
            "({} ? {} : {})",
            ast(condition),
            ast(true_branch),
            ast(false_branch)
        ),
        ExpressionKind::Binary {
            left,
            operator,
            right,
        }
        | ExpressionKind::Logical {
            left,
            operator,
            right,
        } => format!("({} {} {})", operator.lexeme, ast(left), ast(right)),
        ExpressionKind::Assignment { name: _, value } => format!("{:?}", value),
        ExpressionKind::Set {
            object,
            name,
            value,
        } => format!("property set {} = {}", name, ast(value)),
        ExpressionKind::Comma { left, right } => format!("({}, {})", ast(left), ast(right)),
        ExpressionKind::Unary { operator, right } => {
            format!("({} {})", operator.lexeme, ast(right))
        }
        ExpressionKind::Call {
            callee,
            paren,
            arguments,
        } => format!("<fn>"),
        ExpressionKind::Get { object, name } => format!("property access {}", name.lexeme),
        ExpressionKind::Literal(v) => format!("{}", v),
        ExpressionKind::Grouping(inner) => format!("(group {})", ast(inner)),
        ExpressionKind::Variable(var) => format!("{}", var.lexeme),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        expression::LiteralValue,
        token::{Token, TokenType},
    };

    use super::*;

    #[test]
    fn binary() {
        let expression = Expression {
            id: 0,
            kind: ExpressionKind::Binary {
                left: Box::new(Expression {
                    id: 0,
                    kind: ExpressionKind::Literal(LiteralValue::Number(1.0)),
                }),
                operator: Token::new(TokenType::Plus, 0, 1, String::from("+")),
                right: Box::new(Expression {
                    id: 0,
                    kind: ExpressionKind::Literal(LiteralValue::Number(2.0)),
                }),
            },
        };

        let printed = ast(&expression);
        assert_eq!(printed, "(+ 1 2)");
    }
}
