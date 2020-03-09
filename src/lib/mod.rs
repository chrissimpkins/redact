use std::path::Path;
use std::path::PathBuf;

// use failure::Error;
// use log::{debug, info, trace, warn};
// use quote::{quote, ToTokens};
use structopt::StructOpt;
use syn::visit_mut::{self, VisitMut};
// use syn::File;

mod errors;
mod io;
mod parse;
mod transforms;
use io::{stdout_ast_to_rust, write_file, write_ast_to_rust};
use parse::{file_to_ast, inline_crate_to_ast};
use transforms::comments::Comments;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "redact",
    about = "A tool for Rust source code reduction."
)]
struct Opt {
    /// Output file, stdout if not present
    #[structopt(short = "o", long = "outpath", parse(from_os_str), help = "Out file path")]
    outpath: Option<PathBuf>,

    /// Input file
    #[structopt(parse(from_os_str), help = "In file path")]
    inpath: PathBuf,

}

pub(crate) fn run() -> Result<(), failure::Error> {
    let opt = Opt::from_args();
    println!("{:?} ; {:?}", opt.inpath, opt.outpath);
    std::process::exit(0);

    // let filepath = Path::new(&opt.inpath);
    let mut ast: syn::File = inline_crate_to_ast(&opt.inpath)?;
    Comments.visit_file_mut(&mut ast);
    // TODO: dump inlined ast to Rust source file and format
    
    // if opt.outpath {
    //     write_ast_to_rust(ast, &filepath)?;
    // else {
    //     stdout_ast_to_rust(ast)?;
    // }

    // Comments.remove()  TODO: add comments remove stage after add file formatting source
    

    // TODO: read inlined source file to mutable string

    // TODO: pre-ast parsing transforms + testing

    // TODO: parse to AST

    // TODO: AST transforms + testing

    // TODO: dump final reduced file

    Ok(())

}
