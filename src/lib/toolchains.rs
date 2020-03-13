use std::fmt::{self, Display};
#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum Toolchain {
    Stable,
    Beta,
    Nightly,
}

impl Display for Toolchain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Toolchain::Stable => write!(f, "stable"),
            Toolchain::Beta => write!(f, "beta"),
            Toolchain::Nightly => write!(f, "nightly"),
        }
    }
}
