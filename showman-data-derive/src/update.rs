use proc_macro::TokenStream;
use quote::quote;

use crate::context::Context;

pub fn impl_derive_update(context: Context) -> TokenStream {
    let Context { name, table, id, .. } = context;

    let impl_block = quote! {
        impl Update for #name {
            fn update(self, dbc: &DbConnection, id: u32) -> Result<()> {
                use self::#table::dsl::*;

                diesel::update(#table.filter(#id.eq(id)))
                    .set(self)
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