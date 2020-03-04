use std::path::Path;
use std::path::PathBuf;

// use failure::Error;
// use log::{debug, info, trace, warn};
// use quote::{quote, ToTokens};
use structopt::StructOpt;
// use syn::File;

mod errors;
mod parse;
use parse::inline_crate_to_ast;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "cargo-reduce",
    about = "A Cargo plugin for Rust source code reduction."
)]
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

pub(crate) fn run() -> Result<(), failure::Error> {
    let opt = Opt::from_args();
    let filepath = Path::new(&opt.inpath);
    let ast: syn::File = inline_crate_to_ast(filepath)?;
    Ok(())
}
