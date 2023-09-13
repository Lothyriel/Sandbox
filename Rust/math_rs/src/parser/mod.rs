use std::collections::VecDeque;

use crate::lexer::Token;

mod parser;

pub use parser::{Expression, Operator, SyntacticError};

pub fn parse(tokens: VecDeque<Token>) -> Result<Expression, SyntacticError> {
    parser::Parser::new(tokens).get_expression()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_assert_simple_expression() -> Result<(), SyntacticError> {
        let tokens = [
            Token::Number(4.into()),
            Token::AddOp,
            Token::Number(5.into()),
        ];

        let expression = Expression::Binary {
            lhs: Box::new(Expression::Number(4.into())),
            op: Operator::Add,
            rhs: Box::new(Expression::Number(5.into())),
        };

        assert_eq!(parse(tokens.into())?, expression);
        Ok(())
    }

    #[test]
    fn should_assert_long_expression() -> Result<(), SyntacticError> {
        let tokens = [
            Token::Number(1.into()),
            Token::AddOp,
            Token::Number(2.into()),
            Token::AddOp,
            Token::Number(3.into()),
            Token::AddOp,
            Token::Number(4.into()),
            Token::AddOp,
            Token::Number(5.into()),
        ];

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

        assert_eq!(parse(tokens.into())?, expression);
        Ok(())
    }
}
