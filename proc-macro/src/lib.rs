// dacho/proc-macro/src/lib.rs

extern crate proc_macro;

use {
    proc_macro::TokenStream,
    quote::quote,
    syn::{FnArg, ItemFn, Pat, Type, parse_macro_input}
};


#[proc_macro_attribute]
pub fn system(_: TokenStream, item: TokenStream) -> TokenStream {
    let in_fn = parse_macro_input!(item as ItemFn);

    let fn_name  = &in_fn.sig.ident;
    let fn_block = &in_fn.block;

    let args = &in_fn.sig.inputs;

    let arg_types: Vec<Type> = args
        .iter()
        .filter_map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                Some(*pat_type.ty.clone())
            } else {
                None
            }
        })
        .collect();

    let arg_names: Vec<syn::Ident> = args
        .iter()
        .filter_map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                if let Pat::Ident(pat_ident) = &*pat_type.pat {
                    return Some(pat_ident.ident.clone())
                }
            }

            None
        })
        .collect();

    let out_type = quote! { (#(#arg_types,)*) };

    let out_fn = quote! {
        fn #fn_name((#(#arg_names,)*): #out_type) {
            #fn_block
        }
    };

    TokenStream::from(out_fn)
}

