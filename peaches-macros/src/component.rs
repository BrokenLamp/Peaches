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

pub struct Component {
    name: Ident,
    params: Punctuated<ComponentProp, Token![,]>,
    init: Expr,
}

impl Parse for Component {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let content;
        parenthesized!(content in input);
        let params: Fields = content.parse()?;
        input.parse::<Token![=>]>()?;
        let init: Expr = input.parse()?;
        Ok(Component { name, params, init })
    }
}
