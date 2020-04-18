extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let named_fields = if let Data::Struct(strct) = input.data {
        if let Fields::Named(fields) = strct.fields {
            fields.named
        } else {
            unimplemented!();
        }
    } else {
        unimplemented!();
    };

    let quoted_field_list = named_fields.iter().map(|f| {
        let field_name = &f.ident;
        let ty = &f.ty;

        quote! {
            #field_name: Option<#ty>
        }
    });

    let quoted_fields = quote! {
        #(
            #quoted_field_list,
        )*
    };

    let name = input.ident;
    let builder_name = format_ident!("{}Builder", name);

    let expanded = quote! {
        pub struct #builder_name {
            #quoted_fields
        }

        impl #name {
            pub fn builder() {}
        }
    };

    proc_macro::TokenStream::from(expanded)
}
