use thiserror::Error;

#[derive(Error, Debug)]
pub enum AdventError {
    #[error("Input line does not contain enough elements")]
    NotEnoughElements,
    #[error("Input line does not contain too many or not enough elements")]
    WrongNumberOfElements,

    #[error("Input line contains unexpected Element (expected one of {expected:?}, found {found:?})")]
    UnexpectedElement {
        found: String,
        expected: Vec<String>
    },

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Parser(#[from] std::num::ParseIntError),
}