use std::path::Path;
use std::path::PathBuf;

// use failure::Error;
// use log::{debug, info, trace, warn};
// use quote::{quote, ToTokens};
use structopt::StructOpt;
// use syn::File;

mod errors;
mod io;
mod parse;
use io::write_file;
use parse::{file_to_ast, inline_crate_to_ast};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "cargo-reduce",
    about = "A Cargo plugin for Rust source code reduction."
)]
struct Opt {
    // Inline and parse crate instead of single file
    #[structopt(long, help = "Inline crate and reduce")]
    inline: bool,

    /// Input file
    #[structopt(parse(from_os_str), help = "In file path")]
    inpath: PathBuf,

    /// Output file, stdout if not present
    #[structopt(parse(from_os_str), help = "Out file path")]
    outpath: Option<PathBuf>,
}

pub(crate) fn run() -> Result<(), failure::Error> {
    let opt = Opt::from_args();
    let filepath = Path::new(&opt.inpath);
    let ast: syn::File = match opt.inline {
        false => file_to_ast(filepath)?,
        true => inline_crate_to_ast(filepath)?,
    };
    match &opt.outpath {
        Some(filepath) => write_file(&format!("{:?}", ast), &filepath),
        None => {
            println!("{:?}", ast);
            Ok(())
        }
    }
}
