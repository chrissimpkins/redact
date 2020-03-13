use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use failure::Error;
// use log::{debug, info, trace, warn};
use quote::{quote, ToTokens};
use structopt::StructOpt;
use syn::visit_mut::{self, VisitMut};
// use syn::File;
use tempfile::{tempdir, tempfile};

mod errors;
mod fmt;
mod io;
mod parse;
mod toolchains;
mod transforms;
use fmt::rustformat;
use io::{
    read_filepath, stdout_ast_to_rust, write_ast_to_rust, write_filepath,
    write_tempfile_ast_to_rust,
};
use parse::{file_to_ast, inline_crate_to_ast};
use toolchains::Toolchain;
use transforms::comments::Comments;

#[derive(Debug, StructOpt)]
#[structopt(name = "redact", about = "A tool for Rust source code reduction.")]
struct Opt {
    #[structopt(long = "ast", help = "Dump abstract syntax tree")]
    ast: bool,

    /// Output file, stdout if not present
    #[structopt(
        short = "o",
        long = "outpath",
        parse(from_os_str),
        help = "Out file path"
    )]
    outpath: Option<PathBuf>,

    /// Input file
    #[structopt(parse(from_os_str), help = "In file path")]
    inpath: PathBuf,
}

pub(crate) fn run() -> Result<(), Error> {
    let opt = Opt::from_args();

    let mut ast: syn::File = inline_crate_to_ast(&opt.inpath)?;
    // dump AST to stdout
    if opt.ast {
        print!("{:#?}", ast);
        std::io::stdout().flush()?;
        return Ok(());
    }

    // begin transforms
    Comments.visit_file_mut(&mut ast);
    let pre_source = ast.into_token_stream().to_string();
    let comments_removed_text = Comments::remove(&pre_source);

    let filepath = opt.outpath.unwrap();

    write_filepath(&comments_removed_text, &filepath)?;

    // TODO: parse to AST

    // TODO: AST transforms + testing

    // dump final reduced file with rustfmt formatting
    match rustformat(Toolchain::Nightly, &filepath) {
        Ok(_) => return Ok(()),
        Err(error) => return Err(error.into()),
    }
}
