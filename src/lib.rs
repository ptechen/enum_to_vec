extern crate proc_macro;

use proc_macro::TokenStream;
use heck::ToSnakeCase;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, ItemEnum};

/// ignore_field
#[proc_macro_derive(ToVec, attributes(snake_case))]
pub fn enum_to_vec_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemEnum);
    impl_enum_to_vec(input)
}

fn impl_enum_to_vec(input: ItemEnum) -> TokenStream {
    let name = &input.ident;
    let mut to_vec_tokens = vec![];
    let mut snake_case = false;
    for attr in &input.attrs {
        if attr.path().is_ident("snake_case") {
            snake_case = true;
        }
    }

    for variant in input.variants {
        let ident = &variant.ident;
        if snake_case {
            let ident = Ident::new(ident.to_string().to_snake_case().as_str(), Span::call_site());
            to_vec_tokens.push(quote! {stringify!(#ident).to_string()});
        } else {
            to_vec_tokens.push(quote! {stringify!(#ident).to_string()});
        }
    }
    let token = quote! {
        impl #name {
            pub fn to_vec() -> Vec<String> {
                vec![#(#to_vec_tokens),*]
            }
        }
    };
    TokenStream::from(token)
}