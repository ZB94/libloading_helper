use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item};

mod library;

#[proc_macro_attribute]
pub fn library(attr: TokenStream, item: TokenStream) -> TokenStream {
    use library::*;

    let attr = parse_macro_input!(attr as syn::AttributeArgs);

    let item = match parse_macro_input!(item as Item) {
        Item::Mod(item) => parse_mod(item),
        Item::ForeignMod(mut block) => {
            let list = parse_extern_c_block(&mut block);
            quote! {
                #block
                #( #list )*
            }
        }
        item => quote!( #item ),
    };

    quote! {
        #( #attr )*
        #item
    }
    .into()
}
