use std::collections::VecDeque;

mod lexical;

pub use lexical::{LexicalError, Token};

pub fn parse_tokens(input: &str) -> Result<VecDeque<Token>, LexicalError> {
    lexical::Lexer::new(input).parse_tokens()
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;
    use std::str::FromStr;

    use super::{lexical::*, parse_tokens};

    #[test]
    fn should_assert_simple_add() -> Result<(), LexicalError> {
        let tokens = [
            Token::Number(4.into()),
            Token::AddOp,
            Token::Number(5.into()),
        ];

        assert_eq!(parse_tokens("4+5")?, tokens);
        Ok(())
    }

    #[test]
    fn should_assert_number_with_decimal() -> Result<(), LexicalError> {
        let tokens = [Token::Number(Decimal::from_str("4.2")?)];

        assert_eq!(parse_tokens("4.2")?, tokens);

        Ok(())
    }

    #[test]
    fn should_assert_complex_add() -> Result<(), LexicalError> {
        let tokens = [
            Token::Number(4.into()),
            Token::AddOp,
            Token::Number(5.into()),
            Token::AddOp,
            Token::Number(10.into()),
            Token::AddOp,
            Token::Number(200.into()),
        ];

        assert_eq!(parse_tokens("4+5+10+200")?, tokens);
        Ok(())
    }

    #[test]
    fn should_assert_number_with_whitespaces() -> Result<(), LexicalError> {
        let tokens = [
            Token::Number(Decimal::from_str("4.2")?),
            Token::AddOp,
            Token::Number(Decimal::from_str("10.5")?),
        ];

        assert_eq!(parse_tokens("  4.2  + 10.5 \n\r")?, tokens);
        Ok(())
    }

    #[test]
    fn should_assert_complex_add_with_scope() -> Result<(), LexicalError> {
        let tokens = [
            Token::ScopeOpen,
            Token::ScopeOpen,
            Token::Number(4.into()),
            Token::AddOp,
            Token::Number(5.into()),
            Token::ScopeClose,
            Token::AddOp,
            Token::ScopeOpen,
            Token::Number(10.into()),
            Token::AddOp,
            Token::Number(10.into()),
            Token::ScopeClose,
            Token::ScopeClose,
            Token::AddOp,
            Token::Number(10.into()),
        ];

        assert_eq!(parse_tokens("((4+5)+(10+10))+10")?, tokens);
        Ok(())
    }
}
