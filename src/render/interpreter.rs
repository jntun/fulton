use std::fmt::{Display, Formatter};

#[derive(Clone, Copy)]
pub struct Template {}

#[derive(Clone)]
pub enum Error {
    ParseError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParseError(msg) => f.write_str(msg)
        }
    }
}

#[derive(Debug)]
pub enum Literal {
    String(String),
    Float(f64),
    Integer(i64),
}

impl Template {
    pub fn interpret(self, input: &str) -> Result<Literal, Error> {
        let mut cursor = 0;
        println!("interpreting: {}", input);
        for (line_number, line) in input.split('\n').enumerate() {
            for char in  line.chars() {
                match char {
                    _ => cursor += 1
                }

                if char.is_digit(10) {
                    /* TODO */
                }

                if char.is_ascii_alphabetic() {
                    /* TODO */
                }
            }
        }
        Ok(Literal::Integer(cursor))
    }

    pub fn new() -> Self {
        Self {}
    }
}