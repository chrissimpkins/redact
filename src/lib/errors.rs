use std::fmt::{self, Display};
use std::io::{self, Write};

use failure::{bail, format_err, Error, Fail};

#[derive(Fail, Debug)]
pub(crate) enum ReduceError {
    Usage(String),
    Inline(String),
    IO(io::Error),
}

impl Display for ReduceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReduceError::Usage(error) => write!(f, "Usage Error: {}", error),
            ReduceError::Inline(error) => write!(f, "Inline Error: {}", error),
            ReduceError::IO(error) => write!(f, "IO Error: {}", error),
        }
    }
}
