use log::{debug, info, trace, warn};
use proc_macro2::TokenStream;
use syn::visit_mut::{self, VisitMut};
use syn::{parse_quote, Attribute, Expr, File, Lit, LitInt};

#[derive(Debug)]
pub(crate) struct Comments;

impl Comments {

    // TODO: after parse to ast followed by dump to Rust source, all comments have the following formats:
    // (1)   #![doc]
    // (2)   #[doc]
    pub(crate) fn remove(source: &str) -> String {
        // replace the instances of `#[doc]`
        let source_transform_1 = source.replace("#[doc}\n", "").replace("#[doc]", "");
        // replace the instances of `#![doc]`
        source_transform_1.replace("#![doc}\n", "").replace("#![doc]", "")
    }

}

impl VisitMut for Comments {
    fn visit_attribute_mut(&mut self, node: &mut Attribute) {
        // identify documentation lines
        if node.path.is_ident("doc") {
            debug!("{:?}", node.path);
            debug!("{:?}", node.tokens);
            // create empty TokenStream to replace the string literal in
            // comments.  The comments in formatted Rust source appear as
            // one of these two formats after this transformation:
            // (1)  #[doc]
            // (2)  #![doc]
            node.tokens = TokenStream::new();  // create empty TokenStream
        }
        visit_mut::visit_attribute_mut(self, node);
    }
}
