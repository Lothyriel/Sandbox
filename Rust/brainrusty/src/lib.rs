use interpreter::RuntimeError;
use parser::ParsingError;

mod interpreter;
mod lexer;
mod parser;

pub fn interpret(code: &str, input: Option<&str>) -> Result<String, Error> {
    let input = input.map(|i| i.bytes().rev().collect()).unwrap_or_default();

    let instructions = lexer::tokenize(code);
    let parsed = parser::parse(&instructions)?;
    let output = interpreter::interpret(&parsed, input)?;

    Ok(String::from_utf8(output).expect("Output should be valid UTF-8"))
}

#[derive(Debug, PartialEq)]
pub enum Error {
    Parsing(ParsingError),
    Runtime(RuntimeError),
}

#[cfg(test)]
mod tests {
    use super::interpret;

    #[test]
    fn output_hello_world() {
        let input = "-[------->+<]>-.-[->+++++<]>++.+++++++..+++.[--->+<]>-----.---[->+++<]>.-[--->+<]>---.+++.------.--------.-[--->+<]>.";
        let result = interpret(input, None);
        assert_eq!(result, Ok("Hello World!".to_string()));
    }

    #[test]
    fn output_hello_world_nested() {
        let input = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.";
        let result = interpret(input, None);
        assert_eq!(result, Ok("Hello World!".to_string()));
    }

    #[test]
    fn output_a() {
        let input = "--[----->+<]>-----.";
        let result = interpret(input, None);
        assert_eq!(result, Ok("a".to_string()));
    }

    #[test]
    fn output_exclamation() {
        let input = "++++[->++++++++<]>+.";
        let result = interpret(input, None);
        assert_eq!(result, Ok("!".to_string()));
    }

    #[test]
    fn output_head_10() {
        let input = "+>>>>>>>>>>-[,+[-.----------[[-]>]<->]<]";

        let file = include_str!("lexer.rs");

        let result = interpret(input, Some(file));

        let expected = file.lines().take(10).fold(String::new(), |mut acc, l| {
            acc += l;
            acc += "\n";
            acc
        });

        assert_eq!(result, Ok(expected));
    }
}
