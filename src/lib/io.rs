use std::fs::File;
use std::io::prelude::*;
use std::path::{PathBuf, Path};

use failure::Error;
use quote::ToTokens;
// use syn::File;
// use tempfile::tempdir;
use tempfile::tempfile;

pub(crate) fn write_tempfile(src: &str) -> Result<File, Error> {
    let mut f = tempfile()?;
    f.write_all(src.as_bytes())?;
    f.sync_data()?;
    Ok(f)
}

pub(crate) fn write_file(src: &str, filepath: &PathBuf) -> Result<(), Error> {
    let mut f = File::create(filepath)?;
    f.write_all(src.as_bytes())?;
    Ok(())
}

pub(crate) fn write_ast_to_rust(ast: syn::File, filepath: &PathBuf) -> Result<(), Error> {
    write_file(&ast.into_token_stream().to_string(), filepath)?;
    Ok(())
}

pub(crate) fn stdout_ast_to_rust(ast: syn::File) -> Result<(), Error> {
    print!("{}", &ast.into_token_stream().to_string());
    Ok(())
}
