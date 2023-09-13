use std::collections::VecDeque;

use rust_decimal::Decimal;

use crate::lexer::Token;

pub fn parse(mut tokens: VecDeque<Token>) -> Result<Expression, SyntacticError> {
    get_expression(&mut tokens)
}

fn get_expression(tokens: &mut VecDeque<Token>) -> Result<Expression, SyntacticError> {
    let token = expect_symbol(tokens, "Expression")?;

    match token {
        Token::Number(n) => resolve(Expression::Number(n), tokens),
        Token::SubOp => get_negative_number(tokens),
        Token::ScopeOpen => get_scope(tokens),
        _ => Err(SyntacticError::UnexpectedSymbol {
            before: None,
            symbol: token,
        }),
    }
}

fn resolve(lhs: Expression, tokens: &mut VecDeque<Token>) -> Result<Expression, SyntacticError> {
    if tokens.is_empty() {
        Ok(lhs)
    } else {
        get_rhs(lhs, tokens)
    }
}

fn get_rhs(lhs: Expression, tokens: &mut VecDeque<Token>) -> Result<Expression, SyntacticError> {
    let token = expect_symbol(tokens, "Operator")?;

    match token {
        Token::AddOp => get_binary_expression(lhs, tokens, Operator::Add),
        Token::SubOp => get_binary_expression(lhs, tokens, Operator::Sub),
        Token::MultOp => get_binary_expression(lhs, tokens, Operator::Mult),
        Token::DivOp => get_binary_expression(lhs, tokens, Operator::Div),
        s => Err(SyntacticError::UnexpectedSymbol {
            before: Some(s),
            symbol: token,
        }),
    }
}

fn get_binary_expression(
    lhs: Expression,
    tokens: &mut VecDeque<Token>,
    op: Operator,
) -> Result<Expression, SyntacticError> {
    let rhs = get_expression(tokens)?;

    let exp = Expression::Binary {
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
        op,
    };

    Ok(exp)
}

fn get_scope(tokens: &mut VecDeque<Token>) -> Result<Expression, SyntacticError> {
    let _token = expect_symbol(tokens, "Expression")?;

    expect_symbol(tokens, "Expression")?;

    todo!()
}

fn get_negative_number(tokens: &mut VecDeque<Token>) -> Result<Expression, SyntacticError> {
    let next = expect_symbol(tokens, "Expression")?;

    match next {
        Token::Number(n) => Ok(Expression::Number(n)),
        Token::ScopeOpen => get_scope(tokens),
        _ => todo!(),
    }
}

fn expect_symbol(t: &mut VecDeque<Token>, e: &str) -> Result<Token, SyntacticError> {
    match t.pop_front() {
        Some(t) => Ok(t),
        None => Err(SyntacticError::ExpectedSymbol {
            expected: e.to_owned(),
        }),
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
            Expression::Binary { lhs, rhs, op } => write!(f, "{} {} {}", lhs, rhs, op),
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
            Operator::Add => b'+',
            Operator::Sub => b'-',
            Operator::Div => b'/',
            Operator::Mult => b'*',
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
