use std::fmt::{self, Display};
use std::io::{self, Write};

use failure::{bail, format_err, Error, Fail};
use syn_inline_mod::Error as SynInlineError;

#[derive(Fail, Debug)]
pub(crate) enum ReduceError {
    Usage(String),
    Inline(SynInlineError),
    IO(io::Error),
}

impl Display for ReduceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReduceError::Usage(error) => write!(f, "Usage Error: {}", error),
            ReduceError::Inline(error) => write!(
                f,
                "Inline Error: An error occurred during the inline attempt: {:?}",
                error
            ),
            ReduceError::IO(error) => write!(f, "IO Error: {}", error),
        }
    }
}
