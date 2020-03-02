use std::io::{self, Write};
use std::path::Path;
use std::path::PathBuf;
use std::process;

use quote::{quote, ToTokens};
use structopt::StructOpt;
// use syn::File;
use syn_inline_mod::{parse_and_inline_modules, InlinerBuilder};

mod errors;
use crate::errors::Error;

#[derive(Debug, StructOpt)]
#[structopt(name = "cargo-reduce", about = "A Cargo plugin for source code reduction.")]
struct Opt {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long)]
    debug: bool,

    /// Input file
    #[structopt(parse(from_os_str))]
    inpath: PathBuf,

    /// Output file, stdout if not present
    #[structopt(parse(from_os_str))]
    output: Option<PathBuf>,
}

fn main() {
    if let Err(error) = run() {
        let _ = writeln!(io::stderr(), "Error: {}", error);
        process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    let opt = Opt::from_args();
    Ok(())
}
