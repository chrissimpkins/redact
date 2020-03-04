use std::path::Path;

use failure::{bail, Error};
use log::debug;
use quote::{quote, ToTokens};
// use syn::File;
use syn_inline_mod::{Error as SynInlineError, InlinerBuilder};

pub(crate) fn inline_crate(filepath: &Path) -> Result<syn::File, failure::Error> {
    let inline_text: Result<syn::File, SynInlineError> = InlinerBuilder::new()
        .error_not_found(true)
        .parse_and_inline_modules(&filepath);
    debug!("{:?}", inline_text);
    // debug!("{}", &inline_text.unwrap().into_token_stream().to_string());

    match inline_text {
        Ok(inline) => Ok(inline),
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
