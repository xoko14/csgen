use std::{fmt::Display, error::Error, num::ParseIntError};

#[derive(Debug)]
pub struct GenError{
    msg: String
}

impl GenError{
    pub fn new(message: &str) -> Self{
        GenError { msg: message.to_owned() }
    }
}

impl Display for GenError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\u{001b}[31mError:\u{001b}[0m {}", self.msg)
    }
}

impl From<ParseIntError> for GenError{
    fn from(_: ParseIntError) -> Self {
        GenError::new("Invalid input. Could not parse enum index.")
    }
}

impl Error for GenError{}