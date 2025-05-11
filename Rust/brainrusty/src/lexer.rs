// Brainfuck has 8 basic instructions:
//
// > - move the pointer right
// < - move the pointer left
// + - increment the current cell
// - - decrement the current cell
// . - output the value of the current cell
// , - replace the value of the current cell with input
// [ - jump to the matching ] instruction if the current value is zero
// ] - jump to the matching [ instruction if the current value is not zero

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Write,
    Read,
    JumpIfFalsy,
    JumpIfTruthy,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    input
        .bytes()
        .filter_map(|b| match b {
            b'>' => Some(Token::MoveRight),
            b'<' => Some(Token::MoveLeft),
            b'+' => Some(Token::Increment),
            b'-' => Some(Token::Decrement),
            b'.' => Some(Token::Write),
            b',' => Some(Token::Read),
            b'[' => Some(Token::JumpIfFalsy),
            b']' => Some(Token::JumpIfTruthy),
            _ => None,
        })
        .collect()
}
