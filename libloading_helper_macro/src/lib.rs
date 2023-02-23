use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item};

mod library;

/// `#[library(ident)]`
///
/// **Target**
///
/// - `mod` block
/// - `extern "C"` block
#[proc_macro_attribute]
pub fn library(attr: TokenStream, item: TokenStream) -> TokenStream {
    use library::*;

    let ident = parse_macro_input!(attr as syn::Ident);

    let item = match parse_macro_input!(item as Item) {
        Item::Mod(mut item) => {
            if let Some((_, items)) = &mut item.content {
                let mut append = gen(&ident, &parse_items(items));
                items.append(&mut append);
            }

            quote!(#item)
        }
        Item::ForeignMod(mut block) => {
            let items = parse_extern_c_block(&mut block);
            let mut gen = gen(&ident, &items);

            if !block.items.is_empty() {
                gen.push(Item::ForeignMod(block))
            }

            quote! {
                #( #gen )*
            }
        }
        item => quote!( #item ),
    };

    quote! {
        #item
    }
    .into()
}
