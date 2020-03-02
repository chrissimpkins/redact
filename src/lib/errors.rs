use std::fmt::{self, Display};
use std::io::{self, Write};

use failure::{bail, format_err, Fail, Error};
use syn_inline_mod::Error as SynError;

#[derive(Fail, Debug)]
pub(crate) enum ReduceError {
    UsageError(String),
    InlineError(SynError),
    IOError(io::Error),
}

impl Display for ReduceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ReduceError::*;

        match self {
            UsageError(error) => write!(f, "UsageError: {}", error),
            InlineError(_) => write!(f, "InlineError: An error occurred during the inline attempt"),
            IOError(error) => write!(f, "IOError: {}", error),
        }
    }
}