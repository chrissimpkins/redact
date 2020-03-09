use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use failure::bail;
use log::debug;
// use quote::{quote, ToTokens};
use syn::{parse_file, File as SynFile};
use syn_inline_mod::{Error as SynInlineError, InlinerBuilder};

pub(crate) fn inline_crate_to_ast(filepath: &PathBuf) -> Result<SynFile, failure::Error> {
    let inline_ast: Result<syn::File, SynInlineError> = InlinerBuilder::new()
        .error_not_found(true)
        .parse_and_inline_modules(&filepath);
    debug!("{:?}", inline_ast);
    // debug!("{}", &inline_ast.unwrap().into_token_stream().to_string());

    match inline_ast {
        Ok(ast) => Ok(ast),
        Err(error) => match error {
            SynInlineError::NotFound(krate) => {
                let (module, sourceloc) = &krate[0];
                bail!(
                    "Unable to load module '{}' on path '{}'",
                    module,
                    sourceloc.path.display()
                )
            }
            _ => bail!("{:?}", error),
        },
    }
}

pub(crate) fn file_to_ast(filepath: &PathBuf) -> Result<SynFile, failure::Error> {
    let file = File::open(filepath)?;
    let mut buf_reader = BufReader::new(file);
    let mut src = String::new();
    buf_reader.read_to_string(&mut src)?;

    match parse_file(&src) {
        Ok(ast) => Ok(ast),
        Err(error) => bail!(
            "Unable to parse file '{}' with error: {}",
            filepath.display(),
            error
        ),
    }
}
