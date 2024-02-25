extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Expression)]
pub fn derive_expression(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let gen = quote! {
        impl Expression for #name {
            fn build_code(&self) -> Vec<(usize, String)> {
                vec![(0, "self".to_string())]
            }

            fn query<S, U>(&self, field_name: S) -> Option<&U>
            where
                S: AsRef<str>,
                U: ?Sized,
            {
                None
            }
        }
    };
    gen.into()
}
