use std::collections::VecDeque;

use rust_decimal::Decimal;

use crate::lexer::Token;

pub fn parse(tokens: VecDeque<Token>) -> Result<Expression, SyntacticError> {
    Parser::new(tokens).get_expression()
}

struct Parser {
    tokens: VecDeque<Token>,
}

impl Parser {
    pub fn new(tokens: VecDeque<Token>) -> Self {
        Self { tokens }
    }

    pub fn get_expression(&mut self) -> Result<Expression, SyntacticError> {
        let token = self.expect_symbol("Expression")?;

        match token {
            Token::Number(n) => self.resolve(Expression::Number(n)),
            Token::SubOp => self.get_negative_number(),
            Token::ScopeOpen => self.get_scope(),
            _ => Err(SyntacticError::UnexpectedSymbol {
                before: None,
                symbol: token,
            }),
        }
    }

    fn resolve(&mut self, lhs: Expression) -> Result<Expression, SyntacticError> {
        if self.tokens.is_empty() {
            Ok(lhs)
        } else {
            self.get_rhs(lhs)
        }
    }

    fn get_rhs(&mut self, lhs: Expression) -> Result<Expression, SyntacticError> {
        let token = self.expect_symbol("Operator")?;

        match token {
            Token::AddOp => self.get_binary_expression(lhs, Operator::Add),
            Token::SubOp => self.get_binary_expression(lhs, Operator::Sub),
            Token::MultOp => self.get_binary_expression(lhs, Operator::Mult),
            Token::DivOp => self.get_binary_expression(lhs, Operator::Div),
            s => Err(SyntacticError::UnexpectedSymbol {
                before: Some(s),
                symbol: token,
            }),
        }
    }

    fn get_binary_expression(
        &mut self,
        lhs: Expression,
        op: Operator,
    ) -> Result<Expression, SyntacticError> {
        let rhs = self.get_expression()?;

        let exp = Expression::Binary {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            op,
        };

        Ok(exp)
    }

    fn get_scope(&mut self) -> Result<Expression, SyntacticError> {
        let _token = self.expect_symbol("Expression")?;

        self.expect_symbol("Expression")?;

        todo!()
    }

    fn get_negative_number(&mut self) -> Result<Expression, SyntacticError> {
        let next = self.expect_symbol("Expression")?;

        match next {
            Token::Number(n) => Ok(Expression::Number(n)),
            Token::ScopeOpen => self.get_scope(),
            _ => todo!(),
        }
    }

    fn expect_symbol(&mut self, e: &str) -> Result<Token, SyntacticError> {
        match self.tokens.pop_front() {
            Some(t) => Ok(t),
            None => Err(SyntacticError::ExpectedSymbol {
                expected: e.to_owned(),
            }),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Number(Decimal),
    Binary {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
        op: Operator,
    },
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt;

        match self {
            Expression::Number(n) => fmt::Display::fmt(n, f),
            Expression::Binary { lhs, rhs, op } => write!(f, "{} {} {}", lhs, op, rhs),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Add,
    Sub,
    Div,
    Mult,
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            Operator::Add => '+',
            Operator::Sub => '-',
            Operator::Div => '/',
            Operator::Mult => '*',
        };

        write!(f, "{}", op)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SyntacticError {
    #[error("Unknown Symbol {symbol} after {before:?}")]
    UnexpectedSymbol {
        before: Option<Token>,
        symbol: Token,
    },
    #[error("Expected Symbol {expected}")]
    ExpectedSymbol { expected: String },
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
