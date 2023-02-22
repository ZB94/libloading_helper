use std::ffi::CString;

use proc_macro2::Ident;
use quote::quote;
use syn::{
    parse_quote, Attribute, ForeignItem, ForeignItemFn, ForeignItemStatic, Item, ItemForeignMod,
    ItemMod, LitByteStr, Type, Visibility,
};

pub fn parse_mod(mut item: ItemMod) -> proc_macro2::TokenStream {
    if let Some((_, l)) = &mut item.content {
        let mut list = vec![];

        l.retain_mut(|item| {
            if let Item::ForeignMod(fm) = item {
                list.append(&mut parse_extern_c_block(fm));

                !fm.items.is_empty()
            } else {
                true
            }
        });

        l.append(&mut list);
    }

    quote!( #item )
}

pub fn parse_extern_c_block(block: &mut ItemForeignMod) -> Vec<Item> {
    let mut list = vec![];

    block.items.retain(|item| match item {
        ForeignItem::Fn(fn_item) => {
            list.append(&mut fn_impl(fn_item));
            false
        }
        ForeignItem::Static(static_item) => {
            list.append(&mut static_impl(static_item));
            false
        }
        _ => true,
    });

    list
}

fn fn_impl(item: &ForeignItemFn) -> Vec<Item> {
    let args = item.sig.inputs.iter();
    let v = item.sig.variadic.iter();
    let out = &item.sig.output;

    let ty = parse_quote!(unsafe extern "C" fn(#(#args),* #(, #v)*) #out);

    gen(&item.sig.ident, &item.attrs, &item.vis, ty)
}

fn static_impl(item: &ForeignItemStatic) -> Vec<Item> {
    let ty = &item.ty;
    let ty = parse_quote!(*mut #ty);

    gen(&item.ident, &item.attrs, &item.vis, ty)
}

fn gen(ident: &Ident, attrs: &[Attribute], vis: &Visibility, ty: Type) -> Vec<Item> {
    let doc = attrs
        .iter()
        .filter(|attr| attr.path.segments.len() == 1 && attr.path.segments[0].ident == "doc");

    let name = ident.to_string();
    let symbol = LitByteStr::new(
        CString::new(name.as_bytes()).unwrap().as_bytes_with_nul(),
        ident.span(),
    );

    let def = parse_quote! {
        #( #doc )*
        #[allow(non_camel_case_types)]
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
        #vis struct #ident;
    };

    let impl_item = parse_quote! {
        impl ::libloading_helper::LibrarySymbol for #ident {
            const NAME: &'static str = #name;
            const SYMBOL: &'static [u8] = #symbol;
            type Type = #ty;
        }
    };

    vec![def, impl_item]
}
