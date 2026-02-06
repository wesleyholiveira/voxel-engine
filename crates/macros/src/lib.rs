extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(Component)]
pub fn derive_component(_input: TokenStream) -> TokenStream {
    // placeholder derive
    "".parse().unwrap()
}
