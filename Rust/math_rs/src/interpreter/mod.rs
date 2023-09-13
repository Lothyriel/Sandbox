use rust_decimal::Decimal;

use crate::parser::Expression;

mod interpretation;

pub use interpretation::SemanticError;

pub fn evaluate(expression: Expression) -> Result<Decimal, SemanticError> {
    expression.evaluate()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Operator;

    #[test]
    fn should_assert_simple_expression() -> Result<(), SemanticError> {
        let expression = Expression::Binary {
            lhs: Box::new(Expression::Number(4.into())),
            op: Operator::Add,
            rhs: Box::new(Expression::Number(5.into())),
        };

        assert_eq!(expression.evaluate()?, 9.into());
        Ok(())
    }

    #[test]
    fn should_assert_long_expression() -> Result<(), SemanticError> {
        let expression = Expression::Binary {
            lhs: Box::new(Expression::Number(1.into())),
            op: Operator::Add,
            rhs: Box::new(Expression::Binary {
                lhs: Box::new(Expression::Number(2.into())),
                op: Operator::Add,
                rhs: Box::new(Expression::Binary {
                    lhs: Box::new(Expression::Number(3.into())),
                    op: Operator::Add,
                    rhs: Box::new(Expression::Binary {
                        lhs: Box::new(Expression::Number(4.into())),
                        op: Operator::Add,
                        rhs: Box::new(Expression::Number(5.into())),
                    }),
                }),
            }),
        };

        assert_eq!(expression.evaluate()?, 15.into());
        Ok(())
    }
}
