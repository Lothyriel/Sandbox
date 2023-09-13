use rust_decimal::Decimal;

use crate::parser::{Expression, Operator};

pub fn evaluate(expression: Expression) -> Result<Decimal, SemanticError> {
    expression.evaluate()
}

impl Expression {
    pub fn evaluate(&self) -> Result<Decimal, SemanticError> {
        match self {
            Expression::Number(n) => Ok(*n),
            Expression::Binary { lhs, rhs, op } => match op {
                Operator::Add => lhs.add(rhs),
                Operator::Sub => lhs.sub(rhs),
                Operator::Div => lhs.div(rhs),
                Operator::Mult => lhs.mult(rhs),
            },
        }
    }

    pub fn add(&self, rhs: &Expression) -> Result<Decimal, SemanticError> {
        self.evaluate()?
            .checked_add(rhs.evaluate()?)
            .ok_or_else(|| SemanticError::AddFail {
                lhs: self.to_owned(),
                rhs: rhs.to_owned(),
            })
    }

    pub fn sub(&self, rhs: &Expression) -> Result<Decimal, SemanticError> {
        self.evaluate()?
            .checked_sub(rhs.evaluate()?)
            .ok_or_else(|| SemanticError::SubFail {
                lhs: self.to_owned(),
                rhs: rhs.to_owned(),
            })
    }

    pub fn mult(&self, rhs: &Expression) -> Result<Decimal, SemanticError> {
        self.evaluate()?
            .checked_mul(rhs.evaluate()?)
            .ok_or_else(|| SemanticError::MultFail {
                lhs: self.to_owned(),
                rhs: rhs.to_owned(),
            })
    }

    pub fn div(&self, rhs: &Expression) -> Result<Decimal, SemanticError> {
        let rhs_v = rhs.evaluate()?;

        if rhs_v.is_zero() {
            Err(SemanticError::DivisionByZero {
                lhs: self.to_owned(),
                rhs: rhs.to_owned(),
            })
        } else {
            self.evaluate()?
                .checked_div(rhs_v)
                .ok_or_else(|| SemanticError::DivFail {
                    lhs: self.to_owned(),
                    rhs: rhs.to_owned(),
                })
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SemanticError {
    #[error("Attempted to divide by zero in {lhs} / {rhs}")]
    DivisionByZero { lhs: Expression, rhs: Expression },
    #[error("Addition failed in {lhs} + {rhs}")]
    AddFail { lhs: Expression, rhs: Expression },
    #[error("Subtraction failed in {lhs} - {rhs}")]
    SubFail { lhs: Expression, rhs: Expression },
    #[error("Multiplication failed in {lhs} * {rhs}")]
    MultFail { lhs: Expression, rhs: Expression },
    #[error("Division failed in {lhs} / {rhs}")]
    DivFail { lhs: Expression, rhs: Expression },
}
