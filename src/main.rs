use std::io::{self, Write};
use std::process;

mod lib;
use crate::lib::run;

fn main() {
    // initialize logger
    env_logger::init();

    if let Err(error) = run() {
        let _ = writeln!(io::stderr(), "Error: {}", error);
        process::exit(1);
    }
    process::exit(0);
}
