#![recursion_limit = "128"]
#![feature(proc_macro_diagnostic)]

extern crate proc_macro;
use self::proc_macro::{Punct, Spacing, TokenStream, TokenTree};

use quote::{quote, quote_spanned, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;
use syn::{
    parenthesized, parse::Peek, parse_macro_input, punctuated::Punctuated, token, Expr, Field,
    FieldsNamed, FnArg, Ident, Token, Type, Visibility,
};

mod component;
use component::Component;

#[proc_macro]
pub fn component(input: TokenStream) -> TokenStream {
    let Component { name, params, init } = parse_macro_input!(input as Component);

    let result = quote! {
        pub struct #name {
            children: Vec<Option<Box<dyn Component>>>,
        }

        impl #name {
            pub fn new(children: Vec<Option<Box<dyn Component>>>) -> $name {
                #name {
                    children: Vec::new(),
                }
            }
        }

        impl Component for #name {
            fn render(&mut self, state: &mut State) -> Option<Box<dyn Component>> {
                // $block(self, state)
            }
        }
    };

    result.into()
}
