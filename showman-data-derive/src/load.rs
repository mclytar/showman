use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;

use crate::context::Context;

pub fn impl_derive_load(context: Context) -> TokenStream {
    let Context { name, table, id, .. } = context;

    let impl_block = quote! {
        impl Load for #name {
            fn load(dbc: &DbConnection, id: u32) -> Result<Self> {
                use self::#table::dsl::*;

                let result = #table
                    .filter(#id.eq(id))
                    .first(dbc)
                    .map_err(|e: DBError| match e {
                        DBError::NotFound => actix_web::error::ErrorNotFound(""),
                        _ => actix_web::error::ErrorInternalServerError(format!("{}", e))
                    })?;

                Ok(result)
            }
        }
    };

    impl_block.into()
}

pub fn impl_derive_load_all(context: Context) -> TokenStream {
    let Context { name, table, order_by, .. } = context;

    let impl_block = quote! {
        impl LoadAll for #name {
            fn load_all(dbc: &DbConnection) -> Result<Vec<Self>> {
                use self::#table::dsl::*;

                let result = #table
                    #(.order(#order_by.asc()))*
                    .load(dbc)
                    .map_err(|e: DBError| actix_web::error::ErrorInternalServerError(format!("{}", e)))?;

                Ok(result)
            }
        }
    };

    impl_block.into()
}

pub fn impl_derive_load_set(context: Context) -> TokenStream {
    let Context { name, table, parent_id, order_by, .. } = context;
    let parent_id = if let Some(id) = parent_id {
        id
    } else {
        return syn::Error::new(Span::call_site(), "Attribute `parent_id` is mandatory in order to implement trait `LoadFromParent`.")
            .to_compile_error()
            .into()
    };

    let impl_block = quote! {
        impl LoadSet for #name {
            fn load_set(dbc: &DbConnection, parent_id: u32) -> Result<Vec<Self>> {
                use self::#table::dsl::*;

                let result = #table
                    .filter(#parent_id.eq(parent_id))
                    #(.order(#order_by.asc()))*
                    .load(dbc)
                    .map_err(|e: DBError| match e {
                        DBError::NotFound => actix_web::error::ErrorNotFound(""),
                        _ => actix_web::error::ErrorInternalServerError(format!("{}", e))
                    })?;

                Ok(result)
            }
        }
    };

    impl_block.into()
}