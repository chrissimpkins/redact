use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use failure::Error;
use quote::ToTokens;
// use syn::File;
// use tempfile::tempdir;
use tempfile::tempfile;

pub(crate) fn read_filepath(filepath: &PathBuf) -> Result<String, Error> {
    let f = File::open(filepath)?;
    let mut reader = BufReader::new(f);
    let mut text = String::new();
    reader.read_line(&mut text)?;
    Ok(text)
}

pub(crate) fn read_file(f: File) -> Result<String, Error> {
    let mut reader = BufReader::new(f);
    let mut text = String::new();
    reader.read_line(&mut text)?;
    Ok(text)
}

pub(crate) fn write_tempfile(src: &str) -> Result<File, Error> {
    let mut f = tempfile()?;
    f.write_all(src.as_bytes())?;
    f.sync_data()?;
    Ok(f)
}

pub(crate) fn write_tempfile_get_filesize(src: &str) -> Result<u64, Error> {
    let mut f = tempfile()?;
    f.write_all(src.as_bytes())?;
    f.sync_data()?;
    Ok(f.metadata()?.len())
}

pub(crate) fn write_filepath(src: &str, filepath: &PathBuf) -> Result<(), Error> {
    let mut f = File::create(filepath)?;
    f.write_all(src.as_bytes())?;
    f.sync_data()?;
    Ok(())
}

pub(crate) fn write_ast_to_rust(ast: syn::File, filepath: &PathBuf) -> Result<(), Error> {
    write_filepath(&ast.into_token_stream().to_string(), filepath)?;
    Ok(())
}

pub(crate) fn write_tempfile_ast_to_rust(ast: syn::File) -> Result<File, Error> {
    let mut f = tempfile()?;
    f.write_all(&ast.into_token_stream().to_string().as_bytes())?;
    f.sync_data()?;
    Ok(f)
}

pub(crate) fn stdout_ast_to_rust(ast: syn::File) -> Result<(), Error> {
    print!("{}", &ast.into_token_stream().to_string());
    std::io::stdout().flush()?;
    Ok(())
}
