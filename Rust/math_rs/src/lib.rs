mod interpreter;
mod lexer;
mod parser;

pub fn evaluate(input: &str) -> Result<rust_decimal::Decimal, MathError> {
    let tokens = lexer::parse_tokens(input)?;

    let expression = parser::parse(tokens)?;

    let value = interpreter::evaluate(expression)?;

    Ok(value)
}

#[derive(thiserror::Error, Debug)]
pub enum MathError {
    #[error("{0}")]
    Syntactic(#[from] lexer::LexicalError),
    #[error("{0}")]
    Syntax(#[from] parser::SyntacticError),
    #[error("{0}")]
    Semantic(#[from] interpreter::SemanticError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_assert_simple_expression() -> Result<(), MathError> {
        assert_eq!(evaluate("4+5")?, 9.into());
        Ok(())
    }

    #[test]
    fn should_assert_long_expression() -> Result<(), MathError> {
        assert_eq!(evaluate("1+2+3+4+5")?, 15.into());
        Ok(())
    }
}
