extern crate proc_macro;
use proc_macro::TokenStream;

use syn::{ parse_macro_input, ItemImpl };

#[proc_macro_attribute]
pub fn pub_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ret = item.clone();
    let ast = parse_macro_input!(item as ItemImpl);
    dbg!(&ast);
    dbg!(&ast.items);
    ret
}
