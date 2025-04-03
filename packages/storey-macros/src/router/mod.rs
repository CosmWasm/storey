//! This module contains the macro for generating a Router.
//!
//! For ease of maintenance, the macro is split into three parts:
//! - the `parse` module, which parses the input, validates it, and generates a `RouterDef`
//! - the `gen` module, which generates code from the `RouterDef`
//! - the `def` module, which contains the `RouterDef` struct
//!
//! `RouterDef` is the interface between the `parse` and `gen` modules, making sure they are
//! not coupled. So don't couple them! ;)

mod def;
mod gen;
mod parse;

use proc_macro2::TokenStream;
use syn::ItemStruct;

pub fn entry(input: ItemStruct) -> Result<TokenStream, syn::Error> {
    let def = parse::parse(input)?;

    gen::gen(def)
}
