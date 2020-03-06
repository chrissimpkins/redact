use quote::quote;
use syn::visit_mut::{self, VisitMut};
use syn::{parse_quote, Attribute, Expr, File, Lit, LitInt};

#[derive(Debug)]
pub(crate) struct CommentRemove;

impl VisitMut for CommentRemove {
    fn visit_attribute_mut(&mut self, node: &mut Attribute) {
        // identify documentation lines
        if node.path.is_ident("doc") {
            // TODO: need to support visitor `visit_attributes_mut(&mut self, node: &mut Vec<Attribute>)`
            println!("{:?}", node.tokens);
            println!("{:?}", node.path);
        }
        visit_mut::visit_attribute_mut(self, node);
    }

    // fn visit_file_mut(&mut self, f: &mut File) {
    //     for attribute in f.attrs.iter() {
    //         println!("{:?}", attribute);
    //     }
    //     visit_mut::visit_file_mut(self, f);
    // }
}
