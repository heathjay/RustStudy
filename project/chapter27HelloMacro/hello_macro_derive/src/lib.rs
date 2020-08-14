
extern crate proc_macro;
use crate::proc_macro::TokenStream;
use quote::quote;
use syn;
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input:TokenStream)->TokenStream{
    let ast = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
