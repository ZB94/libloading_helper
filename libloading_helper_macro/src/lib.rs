use std::ffi::CString;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ForeignItem, ForeignItemFn, Item, LitByteStr};

#[proc_macro_attribute]
pub fn library(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as syn::AttributeArgs);

    let mut item = parse_macro_input!(item as syn::ItemMod);

    if let Some((_, l)) = &mut item.content {
        let mut fn_list = vec![];

        l.retain_mut(|item| {
            if let Item::ForeignMod(fm) = item {
                fm.items.retain(|item| {
                    if let ForeignItem::Fn(item) = item {
                        fn_list.push(fn_impl(item));
                        false
                    } else {
                        true
                    }
                });

                !fm.items.is_empty()
            } else {
                true
            }
        });

        for mut s in fn_list {
            l.append(&mut s);
        }
    }

    quote! {
        #( #attr )*
        #item
    }
    .into()
}

fn fn_impl(item: &ForeignItemFn) -> Vec<Item> {
    let doc = item
        .attrs
        .iter()
        .filter(|attr| attr.path.segments.len() == 1 && attr.path.segments[0].ident == "doc");

    let vis = &item.vis;

    let name = item.sig.ident.to_string();
    let symbol = LitByteStr::new(
        CString::new(name.as_bytes()).unwrap().as_bytes_with_nul(),
        item.sig.ident.span(),
    );
    let ident = quote::format_ident!("{}", name);

    let def = quote! {
        #( #doc )*
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
        #vis struct #ident;
    };
    let def: Item = syn::parse2(def).unwrap();

    let args = item.sig.inputs.iter();
    let v = item.sig.variadic.iter();
    let out = &item.sig.output;

    let impl_item = quote! {
        impl ::libloading_helper::LibrarySymbol for #ident {
            const NAME: &'static str = #name;
            const SYMBOL: &'static [u8] = #symbol;
            type Type = unsafe extern "C" fn(#(#args),* #(, #v)*) #out;
        }
    };
    let impl_item: Item = syn::parse2(impl_item).unwrap();

    vec![def, impl_item]
}
