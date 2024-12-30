// dacho/proc-macro/src/lib.rs

extern crate proc_macro;

use {
    proc_macro::TokenStream,
    quote::quote,
    syn::{DeriveInput, parse_macro_input}
};

// TODO: refactor this to not require manual updates
static mut COMPONENT_ID: u32 = 1; // amount of built-in components


#[proc_macro_derive(Component)]
pub fn component(input: TokenStream) -> TokenStream {
    let in_t = parse_macro_input!(input as DeriveInput);
    let name = in_t.ident;

    // SAFETY: static mut
    let trait_impl = unsafe { quote! {
        impl dacho::ecs::Component for #name {
            #[inline]
            #[must_use]
            fn id() -> u32 { #COMPONENT_ID }
        }
    } };

    // SAFETY: static mut
    unsafe { COMPONENT_ID += 1; }

    TokenStream::from(trait_impl)
}

