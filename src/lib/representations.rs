use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use failure::{bail, Error};
use quote::{quote, ToTokens};
use syn::{parse_file, parse_str, File as SynFile};

use crate::lib::io::{write_tempfile, write_tempfile_get_filesize};
use crate::lib::parse::{file_to_ast, inline_crate_to_ast};

#[derive(Debug, Clone)]
pub(crate) struct Rust {
    pub(crate) src: String,
    pub(crate) size: u64,
}

impl Rust {
    pub(crate) fn new(src: &str) -> Self {
        Self {
            src: src.to_string(),
            size: Rust::set_filesize_with_str(src),
        }
    }

    fn set_filesize_with_str(src: &str) -> u64 {
        match write_tempfile_get_filesize(src) {
            Ok(filesize) => filesize,
            Err(error) => panic!("{}", error),
        }
    }

    pub(crate) fn to_ast(&self) -> Result<AST, Error> {
        let ast = parse_str::<SynFile>(&self.src);
        match ast {
            Ok(syn) => Ok(AST::new(syn)),
            Err(error) => bail!("{}", error),
        }
    }
}

impl fmt::Display for Rust {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self.src)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct AST {
    pub(crate) src: syn::File,
}

impl AST {
    pub(crate) fn new(src: SynFile) -> Self {
        Self { src }
    }

    pub(crate) fn new_from_rust_file(filepath: &PathBuf) -> Result<AST, Error> {
        match file_to_ast(filepath) {
            Ok(syn) => Ok(AST::new(syn)),
            Err(error) => bail!("{}", error),
        }
    }

    pub(crate) fn new_from_crate(filepath: &PathBuf) -> Result<AST, Error> {
        match inline_crate_to_ast(filepath) {
            Ok(syn) => Ok(AST::new(syn)),
            Err(error) => bail!("{}", error),
        }
    }

    pub(crate) fn to_rust(&self) -> Rust {
        Rust::new(&self.to_rust_str())
    }

    pub(crate) fn to_rust_str(&self) -> String {
        self.src.clone().into_token_stream().to_string()
    }
}

impl fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self.src)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_mock_source() -> String {
        let retstr = r#"
        use std::path::PathBuf;

        pub fn tester() {
            println!("Hello world!");
        }
        "#;
        String::from(retstr)
    }

    fn get_mock_source_modified() -> String {
        let retstr2 = r#"
        use std::path::PathBuf;

        pub fn tester2() {
            println!("Hello world again!");
        }
        "#;
        String::from(retstr2)
    }

    fn get_astobj_string() -> String {
        String::from("AST { src: File { shebang: None, attrs: [], items: [Use(ItemUse { attrs: [], vis: Inherited, use_token: Use, leading_colon: None, tree: Path(UsePath { ident: Ident(std), colon2_token: Colon2, tree: Path(UsePath { ident: Ident(path), colon2_token: Colon2, tree: Name(UseName { ident: Ident(PathBuf) }) }) }), semi_token: Semi }), Fn(ItemFn { attrs: [], vis: Public(VisPublic { pub_token: Pub }), sig: Signature { constness: None, asyncness: None, unsafety: None, abi: None, fn_token: Fn, ident: Ident(tester), generics: Generics { lt_token: None, params: [], gt_token: None, where_clause: None }, paren_token: Paren, inputs: [], variadic: None, output: Default }, block: Block { brace_token: Brace, stmts: [Semi(Macro(ExprMacro { attrs: [], mac: Macro { path: Path { leading_colon: None, segments: [PathSegment { ident: Ident(println), arguments: None }] }, bang_token: Bang, delimiter: Paren(Paren), tokens: TokenStream [Literal { lit: \"Hello world!\" }] } }), Semi)] } })] } }")
    }

    fn get_syn_string() -> String {
        String::from("File { shebang: None, attrs: [], items: [Use(ItemUse { attrs: [], vis: Inherited, use_token: Use, leading_colon: None, tree: Path(UsePath { ident: Ident(std), colon2_token: Colon2, tree: Path(UsePath { ident: Ident(path), colon2_token: Colon2, tree: Name(UseName { ident: Ident(PathBuf) }) }) }), semi_token: Semi }), Fn(ItemFn { attrs: [], vis: Public(VisPublic { pub_token: Pub }), sig: Signature { constness: None, asyncness: None, unsafety: None, abi: None, fn_token: Fn, ident: Ident(tester), generics: Generics { lt_token: None, params: [], gt_token: None, where_clause: None }, paren_token: Paren, inputs: [], variadic: None, output: Default }, block: Block { brace_token: Brace, stmts: [Semi(Macro(ExprMacro { attrs: [], mac: Macro { path: Path { leading_colon: None, segments: [PathSegment { ident: Ident(println), arguments: None }] }, bang_token: Bang, delimiter: Paren(Paren), tokens: TokenStream [Literal { lit: \"Hello world!\" }] } }), Semi)] } })] }")
    }

    #[test]
    fn test_rust_instantiation() {
        let rs = Rust::new(&get_mock_source());
        assert_eq!(rs.src, get_mock_source());
        assert_eq!(rs.size, 116);
        let rs2 = Rust::new(&get_mock_source_modified());
        assert_eq!(rs2.src, get_mock_source_modified());
        assert_eq!(rs2.size, 123);
    }

    #[test]
    fn test_rust_to_ast_method() {
        let rs = Rust::new(&get_mock_source());
        assert_eq!(rs.src, get_mock_source());
        assert_eq!(rs.size, 116);
        let ast = rs.to_ast();
        assert_eq!(ast.is_ok(), true);
        let ast_test = ast.unwrap();
        assert_eq!(format!("{:?}", ast_test), get_astobj_string()); // test debug format for AST struct
        assert_eq!(format!("{}", ast_test), get_syn_string()); // test display format for syn::File string
    }

    #[test]
    fn test_ast_instantiation() {
        let synfile = parse_str::<SynFile>(&get_mock_source());
        let ast = AST::new(synfile.unwrap());
        assert_eq!(format!("{:?}", ast), get_astobj_string()); // test debug format for AST struct
        assert_eq!(format!("{}", ast), get_syn_string()); // test display format for syn::File string
    }

    // TODO: add to Rust method tests
}
