use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn profile(_args: TokenStream, input: TokenStream) -> TokenStream{
    let input = parse_macro_input!(input as ItemFn);
    let statements = input.block.stmts.clone();
    let sig = input.sig.clone();
    let vis = input.vis.clone();
    quote!{
        #vis #sig {
            push_time("start");

            #(#statements)*
            display_profile();
        }
    }.into()
}

#[proc_macro_attribute]
pub fn zone(_args: TokenStream, input: TokenStream) -> TokenStream{
    let input = parse_macro_input!(input as ItemFn);
    let statements = input.block.stmts.clone();
    let sig = input.sig.clone();
    let vis = input.vis.clone();
    quote!{
        #vis #sig {
            push_time("{}", #sig);

            let __result = {
                #(#statements)*
            }
            push_time("{} stop", #sig);
        __result
        }
    }.into()
}
