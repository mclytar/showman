use proc_macro::{TokenStream};
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{Data, Fields};

use crate::context::Context;

pub fn impl_derive_create(context: Context) -> TokenStream {
    let Context { name, table, .. } = context;

    let impl_block = quote! {
        impl Create for #name {
            fn create(self, dbc: &DbConnection) -> Result<u32> {
                use self::#table::dsl::*;

                let id = dbc.transaction(|| {
                    diesel::insert_into(#table)
                        .values(&self)
                        .execute(dbc)?;
                    let id = diesel::select(last_insert_id)
                        .first(dbc)?;
                    Ok(id)
                }).map_err(|e: DBError| HttpResponse::InternalServerError().body(format!("{}", e)))?;

                Ok(id)
            }
        }
    };

    impl_block.into()
}

pub fn impl_derive_create_child(context: Context) -> TokenStream {
    let Context { name, data, table, parent_id, extern_column, .. } = context;

    let parent_id = if let Some(id) = parent_id {
        id
    } else {
        return syn::Error::new(Span::call_site(), "Attribute `parent_id` is mandatory in order to implement trait `CreateChild`.")
            .to_compile_error().into()
    };
    let fields = match data {
        Data::Struct(s) => match s.fields {
            Fields::Named(nf) => nf.named,
            _ => return syn::Error::new(Span::call_site(), "Unsupported struct type.")
                .to_compile_error().into()
        },
        _ => return syn::Error::new(Span::call_site(), "Unsupported type.")
            .to_compile_error().into()
    };
    let field_names: Vec<Ident> = fields.iter()
        .map(|f| f.ident.clone().unwrap())
        .collect();
    let (col_names, col_extra): (Vec<_>, Vec<_>) = extern_column.into_iter()
        .unzip();
    let (col_types, col_processors): (Vec<_>, Vec<_>) = col_extra.into_iter()
        .unzip();

    let table_name = table.to_string();
    let inserter_table_name = name.to_string() + "Inserter";
    let inserter_table = Ident::new(&inserter_table_name, Span::call_site());

    let impl_block = quote! {
        impl CreateChild for #name {
            fn create_child(self, dbc: &DbConnection, parent_id: u32) -> Result < u32 > {
                #[derive(Insertable, Deserialize)]
                #[table_name = #table_name]
                struct #inserter_table {
                    pub #parent_id: u32,
                    #(pub #col_names: #col_types,)*
                    #fields
                }

                let id = dbc.transaction(move || {
                    use self::#table::dsl::*;

                    let inserter = #inserter_table {
                        #parent_id: parent_id,
                        #( #col_names: #col_processors(dbc, parent_id)?,)*
                        #( #field_names : self.#field_names ),*
                    };

                    diesel::insert_into(#table)
                        .values(&inserter)
                        .execute(dbc)?;

                    let result = diesel::select(last_insert_id)
                        .first(dbc)?;

                    Ok(result)
                }).map_err(|e: DBError| match e {
                    DBError::NotFound => HttpResponse::NotFound().finish(),
                    DBError::DatabaseError(DBErrorKind::ForeignKeyViolation, _) => HttpResponse::NotFound().finish(),
                    _ => HttpResponse::InternalServerError().body(format!("{}", e))
                })?;

                Ok(id)
            }
        }
    };

    impl_block.into()
}