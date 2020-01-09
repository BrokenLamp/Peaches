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

struct Component {
    name: Ident,
    params: Fields,
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

        impl Component for $name {
            fn render(&mut self, state: &mut State) -> Option<Box<dyn Component>> {
                $block(self, state)
            }
        }
    };

    result.into()
}

struct ComponentArg {
    name: Ident,
    ty: Type,
}

impl Parse for ComponentArg {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty: Type = input.parse()?;
        Ok(ComponentArg { name, ty })
    }
}

struct Fields {
    segments: Punctuated<ComponentArg, Token![,]>,
}

impl Parse for Fields {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut segments = Punctuated::new();

        segments.push_value(input.parse::<ComponentArg>()?);

        while input.peek(Token![,]) {
            segments.push_punct(input.parse()?);
            segments.push_value(input.parse::<ComponentArg>()?);
        }

        Ok(Fields { segments })
    }
}
