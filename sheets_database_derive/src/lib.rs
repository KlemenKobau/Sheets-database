use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput};

#[proc_macro_derive(Entity)]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    dbg!("{:?}", &input);

    let data = &input.data;

    if let Data::Struct(data_struct) = data {
        todo!();
    } else {
        quote! {
            compile_error!("Currently only works for structs");
        }
        .into()
    }
}

fn implement_for_data(data_str: &DataStruct) {
    let fields = &data_str.fields;

    match fields {
        syn::Fields::Named(_) => todo!(),
        syn::Fields::Unnamed(_) | syn::Fields::Unit => todo!(),
    }
}
