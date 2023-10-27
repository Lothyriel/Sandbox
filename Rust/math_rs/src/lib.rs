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
    Lexical(#[from] lexer::LexicalError),
    #[error("{0}")]
    Syntactic(#[from] parser::SyntacticError),
    #[error("{0}")]
    Semantic(#[from] interpreter::SemanticError),
}

impl From<rust_decimal::Error> for MathError {
    fn from(value: rust_decimal::Error) -> Self {
        Self::Lexical(value.into())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rust_decimal::Decimal;

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

    #[test]
    fn should_assert_simple_scoped_expression() -> Result<(), MathError> {
        assert_eq!(evaluate("(5+4)")?, 9.into());
        Ok(())
    }

    #[test]
    fn should_assert_pemdas_expression() -> Result<(), MathError> {
        assert_eq!(evaluate("3+5*2")?, 13.into());
        Ok(())
    }

    #[test]
    fn should_assert_pemdas_expression2() -> Result<(), MathError> {
        assert_eq!(evaluate("5*2+3")?, 13.into());
        Ok(())
    }

    #[test]
    fn should_assert_scoped_expression() -> Result<(), MathError> {
        assert_eq!(evaluate("3*(5+4)")?, 27.into());
        Ok(())
    }

    #[test]
    fn should_assert_scoped_expression2() -> Result<(), MathError> {
        assert_eq!(evaluate("(3*(2+5))+4")?, 25.into());
        Ok(())
    }

    #[test]
    fn should_assert_scoped_expression3() -> Result<(), MathError> {
        assert_eq!(
            evaluate("(3*(2+5))+4+17*3/14")?,
            Decimal::from_str("28.642857142857142857142857143")?
        );
        Ok(())
    }

    #[test]
    fn should_assert_scoped_expression4() -> Result<(), MathError> {
        assert_eq!(evaluate("(1+2*10)/(2+5)")?, 3.into());
        Ok(())
    }
}
