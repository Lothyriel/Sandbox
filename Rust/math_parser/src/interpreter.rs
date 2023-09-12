use rust_decimal::Decimal;

use crate::parser::{Expression, Operator};

pub fn evaluate(expression: Expression) -> Result<Decimal, SemanticError> {
    Ok(expression.evaluate())
}

impl Expression {
    pub fn evaluate(&self) -> Decimal {
        match self {
            Expression::Number(n) => *n,
            Expression::Binary { lhs, rhs, op } => binary(lhs, rhs, op),
        }
    }
}

fn binary(lhs: &Expression, rhs: &Expression, op: &Operator) -> Decimal {
    match op {
        Operator::Add => lhs.evaluate() + rhs.evaluate(),
        Operator::Sub => lhs.evaluate() - rhs.evaluate(),
        Operator::Div => lhs.evaluate() / rhs.evaluate(),
        Operator::Mult => lhs.evaluate() * rhs.evaluate(),
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SemanticError {
    #[error("Attempted to divide by zero")]
    DivisionByZero { lhs: Expression, rhs: Expression },
}
