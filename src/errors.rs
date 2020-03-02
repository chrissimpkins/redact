use std::fmt::{self, Display};
use std::io::{self, Write};

pub(crate) enum ReduceError {
    UsageError(String),
    IOError(io::Error),
}

impl Display for ReduceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ReduceError::*;

        match self {
            UsageError(error) => write!(f, "UsageError: {}", error),
            IOError(error) => write!(f, "IOError: {}", error),
        }
    }
}