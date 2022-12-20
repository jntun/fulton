pub mod template;
pub mod interpreter;

#[derive(Clone)]
pub enum Error {
    HTML(String),
    JSON(),
    Interpreter(interpreter::Error),
}

pub trait Item<R: Clone> {
    fn compile(self) -> Result<R, Error>;
}