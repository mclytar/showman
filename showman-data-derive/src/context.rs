use syn::Ident;
use syn::{Attribute, Data};
use proc_macro2::{Span, TokenStream, TokenTree, Spacing};

pub struct Context {
    pub name: Ident,
    pub data: Data,
    pub table: Ident,
    pub id: Ident,
    pub parent_resource_name: Option<String>,
    pub parent_id: Option<Ident>,
    pub order_by: Vec<Ident>,
    pub extern_column: Vec<(Ident, (Ident, Ident))>
}

pub fn get_attribute_value_string<I>(attributes: &Vec<Attribute>, key: I) -> syn::Result<Option<String>>
where I: AsRef<str> {
    let attribute = attributes.iter()
        .find(|a| a.path.is_ident(&key));

    let lit_value_result = attribute.map(|a| match a.parse_meta()? {
        syn::Meta::NameValue(syn::MetaNameValue { lit, .. }) => match lit {
            syn::Lit::Str(lit) => Ok(lit),
            _ => Err(syn::Error::new(Span::call_site(), "Unexpected attribute value type."))
        },
        _ => Err(syn::Error::new(Span::call_site(), "Unexpected attribute type."))
    });

    let value = match lit_value_result {
        Some(Ok(value)) => Some(value.value()),
        Some(Err(e)) => return Err(e),
        None => None
    };

    Ok(value)
}

pub fn parse_extern_column(attributes: &Vec<Attribute>) -> syn::Result<Vec<(Ident, (Ident, Ident))>> {
    let values: syn::Result<Vec<TokenStream>> = attributes.iter()
        .filter(|a| a.path.is_ident("extern_column"))
        .map(|a| a.parse_args())
        .collect();
    values?.into_iter()
        .map(|ts| {
            let mut ts = ts.into_iter();
            let column = match ts.next() {
                Some(TokenTree::Ident(i)) => i,
                _ => return Err(syn::Error::new(Span::call_site(), "unexpected token"))
            };
            match ts.next() {
                Some(TokenTree::Punct(p)) => {
                    if p.as_char() != ':' || p.spacing() != Spacing::Alone {
                        return Err(syn::Error::new(Span::call_site(), "unexpected token"));
                    }
                },
                _ => return Err(syn::Error::new(Span::call_site(), "unexpected token"))
            };
            let col_type = match ts.next() {
                Some(TokenTree::Ident(i)) => i,
                _ => return Err(syn::Error::new(Span::call_site(), "unexpected token"))
            };
            match (ts.next(), ts.next()) {
                (Some(TokenTree::Punct(p1)), Some(TokenTree::Punct(p2))) => {
                    if p1.as_char() != '<' || p2.as_char() != '-' || p1.spacing() != Spacing::Joint || p2.spacing() != Spacing::Alone {
                        return Err(syn::Error::new(Span::call_site(), "unexpected token"));
                    }
                },
                _ => return Err(syn::Error::new(Span::call_site(), "unexpected token"))
            };
            let function = match ts.next() {
                Some(TokenTree::Ident(i)) => i,
                _ => return Err(syn::Error::new(Span::call_site(), "unexpected token"))
            };

            Ok((column, (col_type, function)))
        }).collect()
}

impl Context {
    pub fn new(name: &Ident, data: &Data, attributes: &Vec<Attribute>) -> syn::Result<Context> {
        let table = get_attribute_value_string(attributes, "table_name")?
            .unwrap_or(name.to_string());
        let id = get_attribute_value_string(attributes, "load_id")?
            .unwrap_or(table.clone() + "_id");
        let parent_id: Option<String> = get_attribute_value_string(attributes, "parent_id")?;
        let order_by: Option<String> = get_attribute_value_string(attributes, "order_by")?;
        let data = data.to_owned();
        let extern_column = parse_extern_column(attributes)?;
        let parent_resource_name = get_attribute_value_string(attributes, "parent_resource_name")?;

        let name = name.to_owned();
        let table = Ident::new(&table, Span::call_site());
        let id = Ident::new(&id, Span::call_site());
        let parent_id = if let Some(id) = parent_id {
            Some(Ident::new(&id, Span::call_site()))
        } else {
            None
        };
        let order_by = if let Some(order) = order_by {
            vec![Ident::new(&order, Span::call_site())]
        } else {
            vec![]
        };

        Ok(Context { name, data, table, id, parent_id, parent_resource_name, order_by, extern_column })
    }
}