extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Data, DeriveInput, Fields, GenericArgument, Lit, Meta, NestedMeta,
    PathArguments, Type,
};

#[proc_macro_derive(Builder, attributes(builder))]
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

        let segments = if let Type::Path(type_path) = &f.ty {
            &type_path.path.segments
        } else {
            unimplemented!();
        };

        let ty = if segments[0].ident == "Option" {
            if let PathArguments::AngleBracketed(args) = &segments[0].arguments {
                if let GenericArgument::Type(ty) = &args.args[0] {
                    ty
                } else {
                    unimplemented!()
                }
            } else {
                unimplemented!()
            }
        } else {
            &f.ty
        };

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

    let empty_builder_fields = named_fields.iter().map(|f| {
        let field_name = &f.ident;

        quote! {
            #field_name: None
        }
    });

    let empty_builder_body = quote! {
        #builder_name {
            #(
                #empty_builder_fields,
            )*
        }
    };

    let builder_methods_list = named_fields.iter().map(|f| {
        let field_name = &f.ident;

        let segments = if let Type::Path(type_path) = &f.ty {
            &type_path.path.segments
        } else {
            unimplemented!();
        };

        let ty = if segments[0].ident == "Option" {
            if let PathArguments::AngleBracketed(args) = &segments[0].arguments {
                if let GenericArgument::Type(ty) = &args.args[0] {
                    ty
                } else {
                    unimplemented!()
                }
            } else {
                unimplemented!()
            }
        } else {
            &f.ty
        };

        if let Some(attribute) = f.attrs.get(0) {
            // if let PathArguments::Parenthesized(arguments) = attribute.path.segments[0].arguments {

            // }
            if let Ok(Meta::List(list)) = attribute.parse_meta() {
                if let NestedMeta::Meta(Meta::NameValue(value)) = &list.nested[0] {
                    let lit = &value.lit;

                    if let Type::Path(path) = ty {
                        if let PathArguments::AngleBracketed(args) =
                            &path.path.segments[0].arguments
                        {
                            let actual_type = &args.args;

                            let v_name = if let Lit::Str(name) = lit {
                                format_ident!("{}", name.value())
                            } else {
                                unimplemented!()
                            };

                            let foo = args.gt_token;

                            eprintln!("11 {}", quote! {#foo});

                            let res = quote! {
                                fn #v_name(&mut self, #v_name: #actual_type) -> &mut Self {
                                    match &self.#field_name {
                                        None => self.#field_name = Some(vec![#v_name]),
                                        Some(vec) => {
                                            vec.push(#v_name);
                                        }
                                    };

                                    self
                                }
                            };

                            eprintln!("{}", res);

                            res
                        } else {
                            unimplemented!()
                        }
                    } else {
                        unimplemented!()
                    }
                // eprintln!("{}", quote! {#ty});

                // quote! { #lit }
                } else {
                    unimplemented!()
                }
            } else {
                unimplemented!()
            }
        } else {
            quote! {
                fn #field_name(&mut self, #field_name: #ty) -> &mut Self {
                    self.#field_name = Some(#field_name);
                    self
                }
            }
        }
    });

    let builder_errors = named_fields.iter().map(|f| {
        let field_name = &f.ident;

        let first_segment = if let Type::Path(type_path) = &f.ty {
            &type_path.path.segments[0].ident
        } else {
            unimplemented!();
        };

        if first_segment != "Option" {
            quote! {

                if self.#field_name.is_none() {
                    return Err(String::from("#field_name is not set").into());
                }
            }
        } else {
            quote! {}
        }
    });

    let new_struct_fields = named_fields.iter().map(|f| {
        let field_name = &f.ident;

        let segments = if let Type::Path(type_path) = &f.ty {
            &type_path.path.segments
        } else {
            unimplemented!();
        };

        if segments[0].ident == "Option" {
            quote! {
                #field_name: self.#field_name.clone()
            }
        } else {
            quote! {
                #field_name: self.#field_name.clone().unwrap()
            }
        }
    });

    let new_struct = quote! {
        #name {
            #(
                #new_struct_fields
            ),*
        }
    };

    let builder_impl = quote! {
        impl #builder_name {
            #(
                #builder_methods_list
            )*

            pub fn build(&mut self) -> Result<#name, Box<dyn Error>> {
                #(
                    #builder_errors
                )*

                Ok(#new_struct)
            }
        }
    };

    eprintln!("{}", builder_impl);

    let expanded = quote! {
        use std::error::Error;

        pub struct #builder_name {
            #quoted_fields
        }

        impl #name {
            pub fn builder() -> #builder_name {
                #empty_builder_body
            }
        }

        #builder_impl
    };

    proc_macro::TokenStream::from(expanded)
}
