use std::collections::VecDeque;

use rust_decimal::Decimal;

use crate::lexer::Token;

pub struct Parser {
    tokens: VecDeque<Token>,
}

type ParseResult = Result<Expression, SyntacticError>;

impl Parser {
    pub fn new(tokens: VecDeque<Token>) -> Self {
        Self { tokens }
    }

    pub fn get_expression(&mut self) -> ParseResult {
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

    fn resolve(&mut self, lhs: Expression) -> ParseResult {
        if let Some(token) = self.tokens.front() {
            match token {
                Token::AddOp | Token::SubOp | Token::MultOp | Token::DivOp => self.get_rhs(lhs),
                Token::ScopeClose => Ok(lhs),
                Token::ScopeOpen | Token::Number(_) => Err(SyntacticError::UnexpectedSymbol {
                    before: Some(*token),
                    symbol: *token,
                }),
            }
        } else {
            Ok(lhs)
        }
    }

    fn get_rhs(&mut self, lhs: Expression) -> ParseResult {
        let token = self.expect_symbol("Operator")?;

        match token {
            Token::AddOp => self.get_binary_expression(lhs, Operator::Add),
            Token::SubOp => self.get_binary_expression(lhs, Operator::Sub),
            Token::MultOp => self.get_binary_expression(lhs, Operator::Mult),
            Token::DivOp => self.get_binary_expression(lhs, Operator::Div),
            _ => Err(SyntacticError::UnexpectedSymbol {
                before: Some(token),
                symbol: token,
            }),
        }
    }

    fn get_binary_expression(&mut self, lhs: Expression, op: Operator) -> ParseResult {
        let rhs = self.get_expression()?;

        let exp = Expression::Binary {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            op,
        };

        Ok(exp)
    }

    fn get_scope(&mut self) -> ParseResult {
        let expression = self.get_expression()?;

        self.expect_symbol("ScopeClose")?;

        self.resolve(expression)
    }

    fn get_negative_number(&mut self) -> ParseResult {
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
        match self {
            Expression::Number(n) => write!(f, "{}", n),
            Expression::Binary { lhs, rhs, op } => {
                let rhs = match rhs.as_ref() {
                    Expression::Number(n) => format!("{}", n),
                    _ => format!("({})", rhs),
                };

                write!(f, "{} {} {}", lhs, op, rhs)
            }
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
