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
            use crate::profiler::{begin_profiling, display_profile};
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
            use crate::profiler::{get_profiling_parent, push_time, set_profiling_parent, update_parent};
            use crate::time_tools::get_rdtsc;
            let parent = get_profiling_parent();
            set_profiling_parent(#label.to_string());
            let start = get_rdtsc();

            let __result = (|| #block)();

            let elapsed = get_rdtsc() - start;
            update_parent(parent.clone(), elapsed);
            push_time(#label.to_string(), elapsed);
            set_profiling_parent(parent.to_string());
            return __result;
            }
    }
    .into()
}
