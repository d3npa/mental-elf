use std::error::Error;
use std::fmt;
use std::io;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct StringError {
    details: String
}

impl StringError {
    pub fn new(msg: &str) -> StringError {
        StringError { details: String::from(msg) }
    }

    pub fn boxed(msg: &str) -> Box<StringError> {
        Box::new(StringError::new(msg))
    }
}

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for StringError {}

impl From<io::Error> for StringError {
    fn from(error: io::Error) -> StringError {
        StringError::new(&error.to_string())
    }
}