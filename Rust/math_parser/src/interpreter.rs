use rust_decimal::Decimal;

use crate::parser::{Expression, Operator};

pub fn evaluate(expression: Expression) -> Result<Decimal, SemanticError> {
    expression.evaluate()
}

impl Expression {
    pub fn evaluate(&self) -> Result<Decimal, SemanticError> {
        match self {
            Expression::Number(n) => Ok(*n),
            Expression::Binary { lhs, rhs, op } => binary(lhs, rhs, op),
        }
    }

    pub fn add(&self, rhs: &Expression) -> Result<Decimal, SemanticError> {
        Ok(self.evaluate()? + rhs.evaluate()?)
    }

    pub fn sub(&self, rhs: &Expression) -> Result<Decimal, SemanticError> {
        Ok(self.evaluate()? - rhs.evaluate()?)
    }

    pub fn mult(&self, rhs: &Expression) -> Result<Decimal, SemanticError> {
        Ok(self.evaluate()? * rhs.evaluate()?)
    }

    pub fn div(&self, rhs: &Expression) -> Result<Decimal, SemanticError> {
        let rhs_v = rhs.evaluate()?;

        if rhs_v.is_zero() {
            Err(SemanticError::DivisionByZero {
                lhs: self.to_owned(),
                rhs: rhs.to_owned(),
            })
        } else {
            Ok(self.evaluate()? / rhs_v)
        }
    }
}

fn binary(lhs: &Expression, rhs: &Expression, op: &Operator) -> Result<Decimal, SemanticError> {
    match op {
        Operator::Add => lhs.add(rhs),
        Operator::Sub => lhs.sub(rhs),
        Operator::Div => lhs.div(rhs),
        Operator::Mult => lhs.mult(rhs),
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SemanticError {
    #[error("Attempted to divide by zero")]
    DivisionByZero { lhs: Expression, rhs: Expression },
}
