use std::path::Path;
use std::path::PathBuf;

use failure::Error;
use quote::{quote, ToTokens};
use structopt::StructOpt;
use syn::File;

// mod errors;
mod inline;
use inline::inline_crate;

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
    let result: syn::File = inline_crate(filepath)?;
    println!("{}", &result.into_token_stream().to_string());
    // println!("{:?}", result);
    Ok(())
}
