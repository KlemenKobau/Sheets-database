use std::ops::Deref;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, Data, DataStruct, DeriveInput, Field, FieldsNamed, PathSegment, Type,
};

struct Property<'a> {
    name: &'a Ident,
    type_name: &'a Ident,
}

#[proc_macro_derive(Entity)]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    dbg!("{:?}", &input);

    let ident = &input.ident;
    let data = &input.data;

    if let Data::Struct(data_struct) = data {
    } else {
        return quote! {
            compile_error!("Currently only works for structs");
        }
        .into();
    }

    quote! {
        impl Entity for #ident {
            fn save(&self) {

            }

            fn query(match_function: dyn Fn(&Self) -> bool) -> Vec<Self>
            where
                Self: std::marker::Sized {
                Vec::new()
            }
        }
    }
    .into()
}

fn implement_for_data(data_str: &DataStruct) {
    let fields = &data_str.fields;

    match fields {
        syn::Fields::Named(named_fields) => {
            let props = parse_named_fields(named_fields);
        }
        syn::Fields::Unnamed(_) | syn::Fields::Unit => todo!(),
    }
}

fn parse_named_fields(named_fields: &FieldsNamed) -> Vec<Property<'_>> {
    let mut fields = Vec::new();

    for Field { ident, ty, .. } in named_fields.named.iter() {
        let field_name = ident.as_ref().unwrap();

        if let Type::Path(path) = ty {
            let path = &path.path;
            for PathSegment { ident, .. } in path.segments.iter() {
                let property = Property {
                    name: field_name,
                    type_name: ident,
                };

                fields.push(property);
            }
        }
    }

    fields
}
