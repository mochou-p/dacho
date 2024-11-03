// dacho/proc-macro/src/lib.rs

extern crate proc_macro;

use {
    proc_macro::TokenStream,
    quote::quote,
    syn::{FnArg, ItemFn, Pat, Type, PathArguments, GenericArgument, parse2, parse_macro_input}
};


#[proc_macro_attribute]
pub fn system(_: TokenStream, item: TokenStream) -> TokenStream {
    let in_fn    = parse_macro_input!(item as ItemFn);

    let fn_name  = &in_fn.sig.ident;
    let fn_block = &in_fn.block;

    let (mut arg_types, mut arg_names) = (vec![], vec![]);

    for arg in &in_fn.sig.inputs {
        if let FnArg::Typed(pat_type) = arg {
            let ty     = &*pat_type.ty.clone();
            let new_ty = fix_query(ty);

            arg_types.push(new_ty);

            if let Pat::Ident(pat_ident) = &*pat_type.pat {
                arg_names.push(pat_ident.ident.clone());
            }
        }
    }

    let out_type = quote! { (#(#arg_types,)*) };

    let out_fn = quote! {
        fn #fn_name((#(#arg_names,)*): #out_type) {
            #fn_block
        }
    };

    TokenStream::from(out_fn)
}

fn fix_query(ty: &Type) -> Type {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Query" {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    if args.args.len() == 1 {
                        let first_arg = &args.args[0];

                        if !matches!(first_arg, GenericArgument::Type(Type::Tuple(_))) {
                            let new_tuple = quote! { (#first_arg,) };

                            return parse2(quote! { Query<#new_tuple> }).expect("could not parse new Query<(T,)>");
                        }
                    }
                }
            }
        }
    }

    ty.clone()
}

