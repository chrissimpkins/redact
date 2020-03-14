use std::io::Write;
use std::path::PathBuf;

use failure::{bail, Error};
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
pub(crate) struct Opt {
    #[structopt(long = "ast", help = "Dump abstract syntax tree")]
    ast: bool,

    /// Output file, stdout if not present
    #[structopt(short = "f", long = "fmt", help = "Format output")]
    format: bool,

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

    /// Rust toolchain for testing and source formatting
    #[structopt(
        short = "t",
        long = "toolchain",
        help = "Rust toolchain (stable, beta, nightly)"
    )]
    toolchain: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct Config {
    pub inpath: PathBuf,
    pub outpath: PathBuf,
    pub toolchain: Toolchain,
    pub is_ast_request: bool,
    pub is_format_request: bool,
}

impl Config {
    pub(crate) fn new(opt: Opt) -> Self {
        let tc = match opt.toolchain {
            Some(t) => match t.as_ref() {
                "stable" => Toolchain::Stable,
                "beta" => Toolchain::Beta,
                "nightly" => Toolchain::Nightly,
                _ => Toolchain::Stable,
            },
            None => Toolchain::Stable,
        };

        let opath = match opt.outpath {
            Some(fp) => fp,
            None => PathBuf::from("./reduced_.rs"), // default outpath
        };

        Self {
            inpath: opt.inpath,
            outpath: opath,
            is_ast_request: opt.ast,
            is_format_request: opt.format,
            toolchain: tc,
        }
    }
}

pub(crate) fn run() -> Result<(), Error> {
    let opt = Opt::from_args();

    // ======================
    // Early bail validations
    // ======================

    // Validate inpath exists
    if !&opt.inpath.as_path().is_file() {
        bail!("the file path {:?} does not appear to exist.", opt.inpath);
    }
    // Validate toolchain
    if opt.toolchain.is_some() {
        let tc = &opt.toolchain.clone().unwrap();
        if !(tc == "stable" || tc == "beta" || tc == "nightly") {
            bail!(
                "the toolchain must be defined as 'stable', 'beta', or 'nightly'. Received '{}'.",
                tc
            )
        }
    }

    // ======================
    // Parse configuration
    // ======================
    let config = Config::new(opt);

    // ======================
    // Begin transforms
    // ======================
    // inline source files
    let mut ast: syn::File = inline_crate_to_ast(&config.inpath)?;

    // dump AST to stdout (optional)
    if config.is_ast_request {
        if config.is_format_request {
            print!("{:#?}", ast);
        } else {
            print!("{:?}", ast);
        }
        std::io::stdout().flush()?;
        return Ok(());
    }

    Comments.visit_file_mut(&mut ast);
    let pre_source = ast.into_token_stream().to_string();
    let comments_removed_text = Comments::remove(&pre_source);

    write_filepath(&comments_removed_text, &config.outpath)?;

    // TODO: add AST transforms + testing

    // run of rustfmt on the final source file (optional)
    if config.is_format_request {
        match rustformat(config.toolchain, &config.outpath) {
            Ok(_) => return Ok(()),
            Err(error) => return Err(error.into()),
        }
    }
    Ok(())
}
