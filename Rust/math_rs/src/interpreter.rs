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
        self.evaluate()?
            .checked_add(rhs.evaluate()?)
            .ok_or_else(|| SemanticError::AddFail {
                lhs: self.to_owned(),
                rhs: rhs.to_owned(),
            })
    }

    pub fn sub(&self, rhs: &Expression) -> Result<Decimal, SemanticError> {
        self.evaluate()?
            .checked_add(rhs.evaluate()?)
            .ok_or_else(|| SemanticError::SubFail {
                lhs: self.to_owned(),
                rhs: rhs.to_owned(),
            })
    }

    pub fn mult(&self, rhs: &Expression) -> Result<Decimal, SemanticError> {
        self.evaluate()?
            .checked_add(rhs.evaluate()?)
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
    #[error("Addition failed")]
    AddFail { lhs: Expression, rhs: Expression },
    #[error("Subtraction failed")]
    SubFail { lhs: Expression, rhs: Expression },
    #[error("Multiplication failed")]
    MultFail { lhs: Expression, rhs: Expression },
    #[error("Division failed")]
    DivFail { lhs: Expression, rhs: Expression },
}
