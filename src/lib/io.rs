use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use failure::Error;
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
