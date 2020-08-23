use proc_macro::TokenStream;
use quote::quote;

use crate::context::Context;

pub fn impl_derive_delete(context: Context) -> TokenStream {
    let Context { name, table, id, .. } = context;

    let impl_block = quote! {
        impl Delete for #name {
            fn delete(dbc: &DbConnection, id: u32) -> Result<()> {
                use self::#table::dsl::*;

                diesel::delete(#table.filter(#id.eq(id)))
                    .execute(dbc)
                    .map_err(|e: DBError| match e {
                        DBError::NotFound => actix_web::error::ErrorNotFound(""),
                        _ => actix_web::error::ErrorInternalServerError(format!("{}", e))
                    })?;

                Ok(())
            }
        }
    };

    impl_block.into()
}