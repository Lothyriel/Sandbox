use std::{collections::VecDeque, str::FromStr};

use rust_decimal::Decimal;

pub struct Lexer {
    output: VecDeque<Token>,
    tokens: VecDeque<u8>,
    input_len: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input_len: input.len(),
            tokens: input.as_bytes().to_vec().into(),
            output: VecDeque::new(),
        }
    }

    pub fn parse_tokens(mut self) -> Result<VecDeque<Token>, LexicalError> {
        while let Some(token) = self.tokens.front() {
            let token = match token {
                b'+' => self.consume(Token::AddOp),
                b'-' => self.consume(Token::SubOp),
                b'*' => self.consume(Token::MultOp),
                b'/' => self.consume(Token::DivOp),
                b'(' => self.consume(Token::ScopeOpen),
                b')' => self.consume(Token::ScopeClose),
                n if n.is_ascii_digit() => self.parse_number()?,
                n if n.is_ascii_whitespace() => {
                    self.tokens.pop_front();
                    None
                }
                _ => {
                    return Err(LexicalError::UnknownSymbol {
                        index: self
                            .input_len
                            .checked_sub(self.tokens.len())
                            .unwrap_or_default(),
                        symbol: *token as char,
                    })
                }
            };

            if let Some(t) = token {
                self.output.push_back(t);
            }
        }

        Ok(self.output)
    }

    fn parse_number(&mut self) -> Result<Option<Token>, LexicalError> {
        let mut number_parts = vec![];

        while let Some(&n) = self.tokens.front() {
            if n == b'.' {
                number_parts.push(n);
                self.tokens.pop_front();
                self.get_digits(&mut number_parts);
            } else if n.is_ascii_digit() {
                number_parts.push(n);
                self.tokens.pop_front();
            } else {
                break;
            }
        }

        let digits = String::from_utf8(number_parts)?;

        let decimal = Decimal::from_str(&digits)?;

        Ok(Some(Token::Number(decimal)))
    }

    fn get_digits(&mut self, number_parts: &mut Vec<u8>) {
        while let Some(n) = self.tokens.front() {
            if n.is_ascii_digit() {
                number_parts.push(*n);
                self.tokens.pop_front();
            } else {
                break;
            }
        }
    }

    fn consume(&mut self, token: Token) -> Option<Token> {
        self.tokens.pop_front();
        Some(token)
    }
}

#[derive(PartialEq, Debug, Clone, Copy, strum_macros::Display)]
pub enum Token {
    Number(Decimal),
    AddOp,
    SubOp,
    MultOp,
    DivOp,
    ScopeOpen,
    ScopeClose,
}

#[derive(thiserror::Error, Debug)]
pub enum LexicalError {
    #[error("{0}")]
    DecimalParsingError(#[from] rust_decimal::Error),
    #[error("{0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("Unknown Symbol: {symbol} in index: {index}")]
    UnknownSymbol { index: usize, symbol: char },
}
