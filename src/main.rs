use std::io::{self, Write};

use std::process;


// mod errors;
// use crate::errors::ReduceError;
mod run;
use crate::run::run;

fn main() {
    
    if let Err(error) = run() {
        let _ = writeln!(io::stderr(), "{}", error);
        process::exit(1);
    }
}


