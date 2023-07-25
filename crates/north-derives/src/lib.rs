#[macro_use]
extern crate syn;
extern crate proc_macro;
use quote::{quote, quote_spanned};
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;
use proc_macro::TokenStream;
use syn::{parse_macro_input, Expr, ExprArray, Ident, Token, parse_quote};
use north_common::state::NorthStateData;

struct PoemBeauty {
    router: Ident,
    state: ExprArray,
}

impl Parse for PoemBeauty {
    fn parse(input: ParseStream) -> Result<Self> {
        let router: Ident = input.parse()?;
        input.parse::<Token![,]>()?;

        let state= input.parse::<ExprArray>()?;

        // let stmt: Stmt = parse_quote! {
        //     #state
        // };

        Ok(PoemBeauty {
            router,
            state
        })
    }
}


#[proc_macro]
pub fn process_poem(input: TokenStream) -> TokenStream {
    let PoemBeauty { router, state, .. } = parse_macro_input!(input as PoemBeauty);

    process_poem_impl(router, state)
}

fn process_poem_impl(router: Ident, state: ExprArray) -> TokenStream {
    // println!("******************************");
    // println!("Helllo sdsd");
    // println!("******************************");


    // let assert_sync = quote_spanned! {ty.span()=>
    //    state.span()
    // };

    for s in state.elems {
        // println!("Helllo yessss");
    }
    let expanded = quote! {
        #router.with(AddData::new(""))
    };

    TokenStream::from(expanded)
}