use std::fmt::{self, Display};
use std::io::{self, Write};

pub(crate) enum Error {
    UsageError(String),
    IOError(io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;

        match self {
            UsageError(error) => write!(f, "{}", error),
            IOError(error) => write!(f, "Unable to read file: {}", error),
        }
    }
}