extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use regex::Regex;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident};

#[proc_macro_derive(Expression)]
pub fn derive_expression(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let gen = quote! {
        impl Expression for #name {}
    };
    gen.into()
}

/// Query trait is depend on Queryable trait
/// It searches for the type T recursively in the struct
#[proc_macro_derive(Query)]
pub fn derive_query(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    match input.data {
        Data::Struct(data_struct) => {
            let fields = if let Fields::Named(fields) = data_struct.fields {
                fields
            } else {
                panic!("only support struct with named fields")
            };
            let field_names = fields.named.iter().map(|f| &f.ident).collect::<Vec<_>>();
            let gen = quote! {
                impl Query for #name
                where
                    Self: std::fmt::Debug + Queryable + 'static,
                {
                    fn query<T: std::fmt::Debug + Expression +'static>(&self) -> Vec<&T> {
                        match self {
                            #name { #(#field_names),* } => {
                                let mut result = Vec::new();
                                // if the type is T, return it
                                if let Some(v) = self.try_query::<T>() {
                                    result.push(v);
                                }
                                // search for the type T in the fields
                                #(
                                    result.extend(#field_names.query::<T>());
                                )*
                                result
                            }
                        }
                    }
                }
            };
            gen.into()
        }
        Data::Enum(data_enum) => {
            let variants = data_enum.variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                quote! {
                    #name::#variant_name(inner) => inner.query::<T>(),
                }
            });

            let gen = quote! {
                impl Query for #name
                where
                    Self: std::fmt::Debug + Queryable + 'static,
                {
                    fn query<T: std::fmt::Debug + Expression +'static>(&self) -> Vec<&T> {
                        match self {
                            #(#variants)*
                        }
                    }
                }
            };
            gen.into()
        }
        _ => panic!("only support struct"),
    }
}

fn camel_to_snake<S: AsRef<str>>(s: S) -> String {
    Regex::new(r"(?P<lower>[a-z])(?P<upper>[A-Z])")
        .unwrap()
        .replace_all(s.as_ref(), "${lower}_${upper}")
        .to_lowercase()
}

/// Implement the is_xxx function for each variant
#[proc_macro_derive(Is)]
pub fn derive_is(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    if let Data::Enum(data_enum) = input.data {
        // implement the is_xxx function for each variant
        let functions = data_enum.variants.iter().map(|variant| {
            let variant_name = &variant.ident;
            let is_function_name = &Ident::new(
                &format!("is_{}", camel_to_snake(variant_name.to_string())),
                variant_name.span(),
            );
            quote! {
                pub fn #is_function_name(&self) -> bool {
                    matches!(self, #name::#variant_name(_))
                }
            }
        });

        let gen = quote! {
            impl #name {
                #(#functions)*
            }
        };
        gen.into()
    } else {
        panic!("only support enum");
    }
}

/// Implement the unwrap_xxx function for each variant
#[proc_macro_derive(Unwrap)]
pub fn derive_unwrap(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    if let Data::Enum(data_enum) = input.data {
        // implement the is_xxx function for each variant
        let functions = data_enum.variants.iter().map(|variant| {
            let variant_name = &variant.ident;
            let unwrap_function_name = &Ident::new(
                &format!("unwrap_{}", camel_to_snake(variant_name.to_string())),
                variant_name.span(),
            );
            quote! {
                pub fn #unwrap_function_name(&self) -> #variant_name {
                    match self {
                        #name::#variant_name(inner) => inner.clone(),
                        _ => panic!("unwrap_{} failed", stringify!(#variant_name)),
                    }
                }
            }
        });

        let gen = quote! {
            impl #name {
                #(#functions)*
            }
        };
        gen.into()
    } else {
        panic!("only support enum");
    }
}

/// Implement the get_offset function for each variant
///
/// the structure must have start_offset and end_offset fields
#[proc_macro_derive(GetOffset)]
pub fn derive_get_offset(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    if let Data::Enum(data_enum) = input.data {
        let variants = data_enum.variants.iter().map(|variant| {
            let variant_name = &variant.ident;
            quote! {
                #name::#variant_name(inner) => (inner.start_offset, inner.end_offset),
            }
        });
        let gen = quote! {
            impl #name {
                pub fn get_offset(&self) -> (usize, usize) {
                    match self {
                        #( #variants )*
                    }
                }
            }
        };
        gen.into()
    } else {
        panic!("only support enum")
    }
}
