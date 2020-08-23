macro_rules! context {
    ($input:expr) => {
        match Context::new(&$input.ident, &$input.data, &$input.attrs) {
            Ok(ctxt) => ctxt,
            Err(e) => return e.to_compile_error().into()
        }
    }
}

extern crate proc_macro;
extern crate proc_macro2;

mod context;

mod create;
mod load;
mod update;
mod delete;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

use crate::context::Context;

#[proc_macro_derive(Create, attributes(table_name))]
pub fn derive_create(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ctxt = context!(input);
    create::impl_derive_create(ctxt)
}

#[proc_macro_derive(CreateChild, attributes(extern_column, parent_resource_name, parent_id, table_name))]
pub fn derive_create_child(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ctxt = context!(input);
    create::impl_derive_create_child(ctxt)
}

#[proc_macro_derive(Load, attributes(load_id, table_name))]
pub fn derive_load(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ctxt = context!(input);
    load::impl_derive_load(ctxt)
}

#[proc_macro_derive(LoadAll, attributes(load_id, order_by, table_name))]
pub fn derive_load_all(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ctxt = context!(input);
    load::impl_derive_load_all(ctxt)
}

#[proc_macro_derive(LoadSet, attributes(load_id, order_by, parent_id, table_name))]
pub fn derive_load_set(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ctxt = context!(input);
    load::impl_derive_load_set(ctxt)
}

#[proc_macro_derive(Update, attributes(table_name))]
pub fn derive_update(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ctxt = context!(input);
    update::impl_derive_update(ctxt)
}

#[proc_macro_derive(Delete, attributes(table_name))]
pub fn derive_delete(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ctxt = context!(input);
    delete::impl_derive_delete(ctxt)
}