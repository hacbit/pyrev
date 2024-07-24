extern crate proc_macro;
use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Expr, ExprLit, Lit};

#[proc_macro_derive(FromNum)]
pub fn derive_from_num(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    if let Data::Enum(data) = input.data {
        let variants = data.variants.iter().map(|variant| {
            let ident = &variant.ident;
            let num = match variant.discriminant {
                Some((
                    _,
                    Expr::Lit(ExprLit {
                        lit: Lit::Int(ref num),
                        ..
                    }),
                )) => num,
                _ => panic!("All variants must have a number"),
            };
            let num = num.base10_parse::<u8>().unwrap();
            quote! {
                #num => Self::#ident,
            }
        });

        let expanded = quote! {
            impl From<u8> for #name {
                fn from(num: u8) -> Self {
                    match num {
                        #(#variants)*
                        _ => Self::Nop,
                    }
                }
            }
        };

        expanded.into()
    } else {
        let msg = "FromNum can only be derived for enums";
        let span = Span::call_site();
        syn::Error::new(span.into(), msg).to_compile_error().into()
    }
}
