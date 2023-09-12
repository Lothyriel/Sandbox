use std::io::BufRead;

use math_rs::MathError;

fn main() -> Result<(), ApplicationError> {
    loop {
        let stdin = std::io::stdin();
        let mut line = String::new();

        println!("Digite uma expressão matemática");
        stdin.lock().read_line(&mut line)?;

        match math_rs::evaluate(&line) {
            Ok(r) => println!("{}", r),
            Err(e) => eprintln!("{}", e),
        };
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ApplicationError {
    #[error("Attempted to divide by zero")]
    Math(#[from] MathError),
    #[error("{0}")]
    IO(#[from] std::io::Error),
}
