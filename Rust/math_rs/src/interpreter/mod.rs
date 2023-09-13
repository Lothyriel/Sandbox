use rust_decimal::Decimal;

use crate::parser::Expression;

mod interpreter;

pub use interpreter::SemanticError;

pub fn evaluate(expression: Expression) -> Result<Decimal, SemanticError> {
    expression.evaluate()
}
