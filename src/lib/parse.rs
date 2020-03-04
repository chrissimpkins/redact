use std::path::Path;

use failure::{bail, Error};
use log::debug;
use quote::{quote, ToTokens};
use syn::{parse_file, File};
use syn_inline_mod::{Error as SynInlineError, InlinerBuilder};

pub(crate) fn inline_crate_to_ast(filepath: &Path) -> Result<syn::File, failure::Error> {
    let inline_ast: Result<syn::File, SynInlineError> = InlinerBuilder::new()
        .error_not_found(true)
        .parse_and_inline_modules(&filepath);
    debug!("{:?}", inline_ast);
    // debug!("{}", &inline_text.unwrap().into_token_stream().to_string());

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
