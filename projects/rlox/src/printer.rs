use crate::expression::Expression;

pub fn ast(expr: &Expression) -> String {
    match expr {
        Expression::Binary {
            left,
            operator,
            right,
        } => format!("({} {} {})", operator.lexeme, ast(left), ast(right)),
        Expression::Unary { operator, right } => {
            format!("({} {})", operator.lexeme, ast(right))
        }
        Expression::Literal(v) => format!("{}", v),
        Expression::Grouping(inner) => format!("(group {})", ast(inner)),
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
        let expression = Expression::Binary {
            left: Box::new(Expression::Literal(LiteralValue::Number(1.0))),
            operator: Token::new(TokenType::Plus, 0, 1, String::from("+")),
            right: Box::new(Expression::Literal(LiteralValue::Number(2.0))),
        };

        let printed = ast(&expression);
        assert_eq!(printed, "(+ 1 2)");
    }
}
