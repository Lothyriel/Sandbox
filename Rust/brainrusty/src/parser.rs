use crate::{lexer::Token, Error};

pub fn parse(instructions: &[Token]) -> Result<Vec<Construct>, ParsingError> {
    let matches = get_loops_matches(instructions)?;

    instructions
        .iter()
        .enumerate()
        .map(|(idx, i)| {
            let construct = match i {
                Token::MoveRight => Construct::MoveRight,
                Token::MoveLeft => Construct::MoveLeft,
                Token::Increment => Construct::Increment,
                Token::Decrement => Construct::Decrement,
                Token::Write => Construct::Write,
                Token::Read => Construct::Read,
                Token::JumpIfFalsy => Construct::JumpIfFalsy {
                    jump_idx: matches[idx],
                },
                Token::JumpIfTruthy => Construct::JumpIfTruthy {
                    jump_idx: matches[idx],
                },
            };

            Ok(construct)
        })
        .collect()
}

fn get_loops_matches(instructions: &[Token]) -> Result<Vec<usize>, ParsingError> {
    let mut matches = vec![];
    let mut result = vec![0; instructions.len()];

    for (idx, i) in instructions.iter().enumerate() {
        match i {
            Token::JumpIfFalsy => {
                matches.push(idx);
            }
            Token::JumpIfTruthy => {
                let match_idx = matches
                    .pop()
                    .ok_or(ParsingError::UnmatchedLoopConstruct { idx })?;

                result[idx] = match_idx;
                result[match_idx] = idx;
            }
            _ => {}
        }
    }

    if let Some(remaining) = matches.last() {
        Err(ParsingError::UnmatchedLoopConstruct { idx: *remaining })
    } else {
        Ok(result)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Construct {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Write,
    Read,
    JumpIfFalsy { jump_idx: usize },
    JumpIfTruthy { jump_idx: usize },
}

#[derive(Debug, PartialEq)]
pub enum ParsingError {
    UnmatchedLoopConstruct { idx: usize },
}

impl From<ParsingError> for Error {
    fn from(value: ParsingError) -> Self {
        Error::Parsing(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_loop() {
        let instructions = vec![Token::JumpIfFalsy, Token::JumpIfTruthy];

        let result = parse(&instructions);

        let expected = vec![
            Construct::JumpIfFalsy { jump_idx: 1 },
            Construct::JumpIfTruthy { jump_idx: 0 },
        ];

        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn parsing_loop_2() {
        let instructions = vec![
            Token::Increment,
            Token::JumpIfFalsy,
            Token::MoveRight,
            Token::Increment,
            Token::JumpIfTruthy,
        ];

        let result = parse(&instructions);

        let expected = vec![
            Construct::Increment,
            Construct::JumpIfFalsy { jump_idx: 4 },
            Construct::MoveRight,
            Construct::Increment,
            Construct::JumpIfTruthy { jump_idx: 1 },
        ];

        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn parsing_nested_loop() {
        let instructions = vec![
            Token::JumpIfFalsy,
            Token::JumpIfFalsy,
            Token::JumpIfTruthy,
            Token::JumpIfTruthy,
        ];

        let result = parse(&instructions);

        let expected = vec![
            Construct::JumpIfFalsy { jump_idx: 3 },
            Construct::JumpIfFalsy { jump_idx: 2 },
            Construct::JumpIfTruthy { jump_idx: 1 },
            Construct::JumpIfTruthy { jump_idx: 0 },
        ];

        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn parsing_unmatched_opening() {
        let instructions = vec![Token::JumpIfFalsy];

        let result = parse(&instructions);

        assert_eq!(result, Err(ParsingError::UnmatchedLoopConstruct { idx: 0 }));
    }

    #[test]
    fn parsing_unmatched_closing() {
        let instructions = vec![Token::JumpIfTruthy];

        let result = parse(&instructions);

        assert_eq!(result, Err(ParsingError::UnmatchedLoopConstruct { idx: 0 }));
    }

    #[test]
    fn parsing_unmatched_nested_opening() {
        let instructions = vec![Token::JumpIfFalsy, Token::JumpIfFalsy, Token::JumpIfTruthy];

        let result = parse(&instructions);

        assert_eq!(result, Err(ParsingError::UnmatchedLoopConstruct { idx: 0 }));
    }

    #[test]
    fn parsing_unmatched_nested_closing() {
        let instructions = vec![Token::JumpIfFalsy, Token::JumpIfTruthy, Token::JumpIfTruthy];

        let result = parse(&instructions);

        assert_eq!(result, Err(ParsingError::UnmatchedLoopConstruct { idx: 2 }));
    }

    #[test]
    fn parsing_n_loops() {
        let instructions = vec![
            Token::JumpIfFalsy,
            Token::JumpIfTruthy,
            Token::JumpIfFalsy,
            Token::JumpIfTruthy,
        ];

        let result = parse(&instructions);

        let expected = vec![
            Construct::JumpIfFalsy { jump_idx: 1 },
            Construct::JumpIfTruthy { jump_idx: 0 },
            Construct::JumpIfFalsy { jump_idx: 3 },
            Construct::JumpIfTruthy { jump_idx: 2 },
        ];

        assert_eq!(result, Ok(expected));
    }
}
