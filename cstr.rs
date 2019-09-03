extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn cstr(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    panic!("TODO: Implement it!")
}
