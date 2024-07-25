//! Provides some macro for app

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Ident, LitInt, Token,
};

struct Info {
    pub macro_ident: Ident,
    pub start: usize,
    pub end: usize,
}

impl Parse for Info {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let macro_ident = input.parse::<Ident>()?;
        input.parse::<Token![,]>()?;
        let start = input.parse::<LitInt>()?.base10_parse()?;
        input.parse::<Token![,]>()?;
        let end = input.parse::<LitInt>()?.base10_parse()?;

        Ok(Self {
            macro_ident,
            start,
            end,
        })
    }
}

#[proc_macro]
pub fn impl_plugin_all_tuples(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Info);
    let macro_ident = &input.macro_ident;

    let mut ident_tuples = Vec::with_capacity(input.end - input.start);

    for i in input.start..=input.end {
        let ident = format_ident!("P{}", i);
        ident_tuples.push(quote! { #ident });
    }

    let impls = (0..ident_tuples.len()).map(|i| {
        let tuple = &ident_tuples[..i];
        quote! {
            #macro_ident!((#( #tuple ),*));
        }
    });

    quote! {
        #( #impls )*
    }
    .into()
}
