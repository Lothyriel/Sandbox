use rust_decimal::Decimal;

use crate::parser::Expression;

mod interpretation;

pub use interpretation::SemanticError;

pub fn evaluate(expression: Expression) -> Result<Decimal, SemanticError> {
    expression.evaluate()
}
