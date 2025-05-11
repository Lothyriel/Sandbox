use crate::{parser::Construct, Error};

const TAPE_SIZE: usize = 1024;

#[derive(Debug)]
struct Interpreter {
    input: Vec<u8>,
    output: Vec<u8>,
    tape: [u8; TAPE_SIZE],
    cell_idx: usize,
    instruction_idx: usize,
}

impl Interpreter {
    fn new(input: Vec<u8>) -> Self {
        Self {
            tape: [0; TAPE_SIZE],
            output: vec![],
            cell_idx: 0,
            instruction_idx: 0,
            input,
        }
    }

    fn process(&mut self, instructions: &[Construct]) -> Result<(), RuntimeError> {
        while let Some(instruction) = instructions.get(self.instruction_idx) {
            self.process_instruction(instruction)?;
            self.instruction_idx += 1;
        }

        Ok(())
    }

    fn process_instruction(&mut self, instruction: &Construct) -> Result<(), RuntimeError> {
        match instruction {
            Construct::MoveRight => self.move_right(),
            Construct::MoveLeft => self.move_left(),
            Construct::Increment => self.increment(),
            Construct::Decrement => self.decrement(),
            Construct::Write => self.write(),
            Construct::Read => self.read(),
            Construct::JumpIfFalsy {
                jump_idx: match_idx,
            } => self.jump(self.jump_falsy()?, *match_idx),
            Construct::JumpIfTruthy {
                jump_idx: match_idx,
            } => self.jump(self.jump_truthy()?, *match_idx),
        }?;

        Ok(())
    }

    fn jump(&mut self, should_jump: bool, match_idx: usize) -> Result<(), RuntimeError> {
        if should_jump {
            self.instruction_idx = match_idx;
        }

        Ok(())
    }

    fn move_right(&mut self) -> Result<(), RuntimeError> {
        if self.cell_idx + 1 < self.tape.len() {
            self.cell_idx += 1;
            Ok(())
        } else {
            Err(RuntimeError::OOBMoveRight)
        }
    }

    fn move_left(&mut self) -> Result<(), RuntimeError> {
        if self.cell_idx.checked_sub(1).is_some() {
            self.cell_idx -= 1;
            Ok(())
        } else {
            Err(RuntimeError::OOBMoveLeft)
        }
    }

    fn increment(&mut self) -> Result<(), RuntimeError> {
        let cell = self.cell_mut()?;

        *cell = cell.wrapping_add(1);

        Ok(())
    }

    fn decrement(&mut self) -> Result<(), RuntimeError> {
        let cell = self.cell_mut()?;

        *cell = cell.wrapping_sub(1);

        Ok(())
    }

    fn write(&mut self) -> Result<(), RuntimeError> {
        let cell = *self.cell()?;

        self.output.push(cell);

        Ok(())
    }

    fn read(&mut self) -> Result<(), RuntimeError> {
        let input = self.input.pop().ok_or(RuntimeError::ExpectedInput)?;
        let cell = self.cell_mut()?;

        *cell = input;

        Ok(())
    }

    fn jump_falsy(&self) -> Result<bool, RuntimeError> {
        let cell = *self.cell()?;

        Ok(cell == 0)
    }

    fn jump_truthy(&self) -> Result<bool, RuntimeError> {
        let cell = *self.cell()?;

        Ok(cell != 0)
    }

    fn cell(&self) -> Result<&u8, RuntimeError> {
        self.tape
            .get(self.cell_idx)
            .ok_or(RuntimeError::OOBCell { idx: self.cell_idx })
    }

    fn cell_mut(&mut self) -> Result<&mut u8, RuntimeError> {
        self.tape
            .get_mut(self.cell_idx)
            .ok_or(RuntimeError::OOBCell { idx: self.cell_idx })
    }
}

#[derive(Debug, PartialEq)]
pub enum RuntimeError {
    OOBMoveLeft,
    OOBMoveRight,
    OOBCell { idx: usize },
    ExpectedInput,
}

impl From<RuntimeError> for Error {
    fn from(value: RuntimeError) -> Self {
        Error::Runtime(value)
    }
}

pub fn interpret(instructions: &[Construct], input: Vec<u8>) -> Result<Vec<u8>, RuntimeError> {
    let mut interpreter = Interpreter::new(input);

    interpreter.process(instructions)?;

    Ok(interpreter.output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oob_left() {
        let instructions = vec![Construct::MoveLeft];
        let result = interpret(&instructions, vec![]);
        assert_eq!(result, Err(RuntimeError::OOBMoveLeft));
    }

    #[test]
    fn oob_right() {
        let instructions: Vec<_> = (0..1024).map(|_| Construct::MoveRight).collect();
        let result = interpret(&instructions, vec![]);
        assert_eq!(result, Err(RuntimeError::OOBMoveRight));
    }
}
