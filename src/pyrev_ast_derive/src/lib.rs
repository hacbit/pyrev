extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, token::Struct, Data, DeriveInput, Fields};
use quote::ToTokens;

#[proc_macro_derive(Expression)]
pub fn derive_expression(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let fields = if let Data::Struct(data_struct) = ast.data {
        if let Fields::Named(fields) = data_struct.fields {
            fields
        } else {
            panic!("only support struct with named fields")
        }
    } else {
        panic!("only support struct with named fields")
    };
    let field_names = fields.named.iter().map(|f| &f.ident);
    let field_types = fields.named.iter().map(|f| &f.ty);
    let gen = quote! {
        impl Expression for #name {
            fn build_code(&self) -> Vec<(usize, String)> {
                [(0, "".to_string())].into()
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
