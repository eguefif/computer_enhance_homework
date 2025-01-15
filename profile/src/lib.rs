use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn profile(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let statements = input.block.stmts.clone();
    let sig = input.sig.clone();
    let vis = input.vis.clone();
    quote! {
        #vis #sig {
            push_time("start", get_rdtsc());

            #(#statements)*
            display_profile();
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn zone(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let attrs = &input.attrs;
    let block = &input.block;
    let sig = &input.sig;
    let vis = &input.vis;
    let label = format!("{}", sig.ident);
    quote! {
        #(#attrs)*
        #vis #sig {
            let start = get_rdtsc();
            let __result = (|| #block)();
            push_time(#label, get_rdtsc() - start);
            return __result;
            }
    }
    .into()
}
