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
                pub fn #unwrap_function_name(self) -> #variant_name {
                    match self {
                        #name::#variant_name(inner) => inner,
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
#[proc_macro_derive(Offset)]
pub fn derive_get_offset(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    if let Data::Enum(data_enum) = input.data {
        let get_variants = data_enum.variants.iter().map(|variant| {
            let variant_name = &variant.ident;
            quote! {
                #name::#variant_name(inner) => (inner.start_offset, inner.end_offset),
            }
        });
        let set_variants = data_enum.variants.iter().map(|variant| {
            let variant_name = &variant.ident;
            quote! {
                #name::#variant_name(inner) => {
                    inner.start_offset = start_offset;
                    inner.end_offset = end_offset;
                }
            }
        });
        let gen = quote! {
            impl #name {
                pub fn get_offset(&self) -> (usize, usize) {
                    match self {
                        #( #get_variants )*
                    }
                }

                pub fn set_offset(&mut self, start_offset: usize, end_offset: usize) {
                    match self {
                        #( #set_variants )*
                    }
                }
            }
        };
        gen.into()
    } else {
        panic!("only support enum")
    }
}

/// Implement From<T: Expression> for expression enum
#[proc_macro_derive(FromExpression)]
pub fn derive_from_expression(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    if let Data::Enum(data_enum) = input.data {
        let from_variants = data_enum.variants.iter().map(|variant| {
            let variant_name = &variant.ident;
            quote! {
                impl From<#variant_name> for #name {
                    fn from(inner: #variant_name) -> Self {
                        #name::#variant_name(inner)
                    }
                }
            }
        });

        let gen = quote! {
            #(#from_variants)*
        };
        gen.into()
    } else {
        panic!("only support enum")
    }
}

/// Implement as ref function for each variant
#[proc_macro_derive(AsRef)]
pub fn derive_as_ref(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    if let Data::Enum(data_enum) = input.data {
        let as_ref_variants = data_enum.variants.iter().map(|variant| {
            let variant_name = &variant.ident;

            let as_ref_function_name = &Ident::new(
                &format!("as_ref_{}", camel_to_snake(variant_name.to_string())),
                variant_name.span(),
            );

            quote! {
                pub fn #as_ref_function_name(&self) -> Option<&#variant_name> {
                    match *self {
                        #name::#variant_name(ref inner) => Some(inner),
                        _ => None,
                    }
                }
            }
        });

        let gen = quote! {
            impl #name {
                #(#as_ref_variants)*
            }
        };
        gen.into()
    } else {
        panic!("only support enum");
    }
}

#[proc_macro_derive(TypeIter)]
pub fn derive_type_iter(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    if let Data::Enum(data_enum) = input.data {
        let fields = data_enum.variants.iter().map(|variant| {
            let variant_name = &variant.ident;
            quote! {
                #variant_name,
            }
        });

        let iter_fields = fields.map(|field| {
            let field_name = field;
            quote! {
                #field_name
            }
        });
        quote! {
            impl #name {
                pub fn type_iter() -> Vec<std::any::TypeId> {
                    let mut result = Vec::new();
                    #(
                        result.push(std::any::TypeId::of::<#iter_fields>());
                    )*
                    result
                }
            }
        }.into()
    } else {
        panic!("only support enum");
    }
}
