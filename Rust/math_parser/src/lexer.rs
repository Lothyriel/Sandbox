use std::{collections::VecDeque, str::FromStr};

use rust_decimal::Decimal;

pub fn parse_tokens(input: &str) -> Result<VecDeque<Token>, LexicalError> {
    let mut tokens: VecDeque<_> = input.as_bytes().to_vec().into();
    let mut output = VecDeque::new();

    while let Some(token) = tokens.front() {
        let token = match token {
            b'+' => consume(&mut tokens, Token::AddOp),
            b'-' => consume(&mut tokens, Token::SubOp),
            b'*' => consume(&mut tokens, Token::MultOp),
            b'/' => consume(&mut tokens, Token::DivOp),
            b'(' => consume(&mut tokens, Token::ScopeOpen),
            b')' => consume(&mut tokens, Token::ScopeClose),
            n if n.is_ascii_digit() => parse_number(&mut tokens)?,
            n if n.is_ascii_whitespace() => {
                tokens.pop_front();
                None
            }
            _ => {
                return Err(LexicalError::UnknownSymbol {
                    index: input.len().checked_div(tokens.len()).unwrap_or_default(),
                    symbol: *token,
                })
            }
        };

        if let Some(t) = token {
            output.push_back(t);
        }
    }

    Ok(output)
}

fn parse_number(tokens: &mut VecDeque<u8>) -> Result<Option<Token>, LexicalError> {
    let mut number_parts = vec![];

    while let Some(&n) = tokens.front() {
        if n == b'.' {
            number_parts.push(n);
            tokens.pop_front();
            get_digits(tokens, &mut number_parts);
        } else if n.is_ascii_digit() {
            number_parts.push(n);
            tokens.pop_front();
        } else {
            break;
        }
    }

    let digits = String::from_utf8(number_parts)?;

    let decimal = Decimal::from_str(&digits)?;

    Ok(Some(Token::Number(decimal)))
}

fn get_digits(input: &mut VecDeque<u8>, number_parts: &mut Vec<u8>) {
    while let Some(n) = input.front() {
        if n.is_ascii_digit() {
            number_parts.push(*n);
            input.pop_front();
        } else {
            break;
        }
    }
}

fn consume(tokens: &mut VecDeque<u8>, token: Token) -> Option<Token> {
    tokens.pop_front();
    Some(token)
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Token {
    Number(Decimal),
    AddOp,
    SubOp,
    MultOp,
    DivOp,
    ScopeOpen,
    ScopeClose,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum LexicalError {
    #[error("{0}")]
    DecimalParsingError(#[from] rust_decimal::Error),
    #[error("{0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("Unknown Symbol: {symbol} in index: {index}")]
    UnknownSymbol { index: usize, symbol: u8 },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_assert_simple_add() -> Result<(), LexicalError> {
        let tokens = [
            Token::Number(4.into()),
            Token::AddOp,
            Token::Number(5.into()),
        ];

        assert_eq!(parse_tokens("4+5").unwrap(), tokens);
        Ok(())
    }

    #[test]
    fn should_assert_number_with_decimal() {
        let tokens = [Token::Number(Decimal::from_str("4.2").unwrap())];

        assert_eq!(parse_tokens("4.2").unwrap(), tokens);
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
