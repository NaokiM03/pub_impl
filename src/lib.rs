extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn pub_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = dbg!(item);
    dbg!(_attr);
    item
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
