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
            begin_profiling();

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
            let parent = get_profiling_parent();
            set_profiling_parent(#label.to_string());
            let start = get_rdtsc();
            let __result = (|| #block)();
            push_time(#label.to_string(), get_rdtsc() - start, parent.clone());
            set_profiling_parent(parent);
            return __result;
            }
    }
    .into()
}
