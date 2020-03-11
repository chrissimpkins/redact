use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use failure::Error;
// use log::{debug, info, trace, warn};
use quote::{quote, ToTokens};
use structopt::StructOpt;
use syn::visit_mut::{self, VisitMut};
// use syn::File;
use tempfile::{tempfile, tempdir};

mod errors;
mod io;
mod parse;
mod transforms;
use io::{read_filepath, stdout_ast_to_rust, write_ast_to_rust, write_filepath, write_tempfile_ast_to_rust};
use parse::{file_to_ast, inline_crate_to_ast};
use transforms::comments::Comments;

#[derive(Debug, StructOpt)]
#[structopt(name = "redact", about = "A tool for Rust source code reduction.")]
struct Opt {
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
    Comments.visit_file_mut(&mut ast);
    let pre_source = ast.into_token_stream().to_string();
    let comments_removed_text = Comments::remove(&pre_source);
    
    match opt.outpath {
        Some(filepath) => write_filepath(&comments_removed_text, &filepath)?,
        None => print!("{}", comments_removed_text),
    }
    
    // Comments.remove()  TODO: add comments remove stage after add file formatting source

    // TODO: read inlined source file to mutable string

    // TODO: pre-ast parsing transforms + testing

    // TODO: parse to AST

    // TODO: AST transforms + testing

    // TODO: dump final reduced file

    Ok(())
}
