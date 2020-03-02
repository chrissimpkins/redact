use std::path::Path;

use failure::{bail, Error};
use syn::File;
use syn_inline_mod::{parse_and_inline_modules, InlinerBuilder};

pub(crate) fn inline_crate(filepath: &Path) -> Result<syn::File, failure::Error> {
    let inline_text: Result<syn::File, syn_inline_mod::Error> = InlinerBuilder::new()
        .error_not_found(true)
        .parse_and_inline_modules(&filepath);
    match inline_text {
        Ok(inline) => Ok(inline),
        Err(error) => bail!("{:?}", error),
    }
}
