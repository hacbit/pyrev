extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident};

#[proc_macro_derive(Expression)]
pub fn derive_expression(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let gen = quote! {
        impl Expression for #name {}
    };
    gen.into()
}

/// Query trait is depend on Queryable trait
/// It searches for the type T recursively in the struct
/// It will return immediately when if finds one, even if the return structures contains the target type T
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
            let variants = data_enum.variants.iter().map(|varient| {
                let varient_name = &varient.ident;
                quote! {
                    #name::#varient_name(inner) => inner.query::<T>(),
                }
            });
            // implement the is_*** function for each variant
            let functions = data_enum.variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                let function_name = &Ident::new(
                    &format!("is_{}", camel_case_to_snake_case(&variant_name.to_string())),
                    variant_name.span(),
                );
                quote! {
                    pub fn #function_name(&self) -> bool {
                        matches!(self, #name::#variant_name(_))
                    }
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

                impl #name {
                    #(#functions)*
                }
            };
            gen.into()
        }
        _ => panic!("only support struct"),
    }
}

fn camel_case_to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i != 0 {
            result.push('_');
        }
        result.push(c.to_lowercase().next().unwrap());
    }
    result
}