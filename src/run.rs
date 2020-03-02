
use std::path::Path;
use std::path::PathBuf;

use failure::Error;
use quote::{quote, ToTokens};
use structopt::StructOpt;
// use syn::File;
use syn_inline_mod::{parse_and_inline_modules, InlinerBuilder};


#[derive(Debug, StructOpt)]
#[structopt(name = "cargo-reduce", about = "A Cargo plugin for Rust source code reduction.")]
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
    let result: syn::File = InlinerBuilder::new()
        .error_not_found(true)
        .parse_and_inline_modules(&filepath).unwrap();
    // println!("{}", &result.into_token_stream().to_string());
    // println!("{:?}", result);
    Ok(())
}