use std::io::{self, Write};

use std::process;

mod lib;
use crate::lib::run;

fn main() {
    if let Err(error) = run() {
        let _ = writeln!(io::stderr(), "{}", error);
        process::exit(1);
    }
}
