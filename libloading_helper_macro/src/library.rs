use std::ffi::CString;

use proc_macro2::Ident;
use quote::quote;
use syn::{
    parse_quote, Attribute, FnArg, ForeignItem, ForeignItemFn, ForeignItemStatic, Item,
    ItemForeignMod, LitByteStr, Signature, Type,
};

pub fn parse_items(items: &mut Vec<Item>) -> Vec<ExternCItem> {
    let mut ret = vec![];

    items.retain_mut(|item| {
        if let Item::ForeignMod(fm) = item {
            ret.append(&mut parse_extern_c_block(fm));

            !fm.items.is_empty()
        } else {
            true
        }
    });

    ret
}

pub fn parse_extern_c_block(block: &mut ItemForeignMod) -> Vec<ExternCItem> {
    let mut ret = vec![];

    block.items.retain(|item| match item {
        ForeignItem::Fn(fn_item) => {
            ret.push(fn_item.into());
            false
        }
        ForeignItem::Static(static_item) => {
            ret.push(static_item.into());
            false
        }
        _ => true,
    });

    ret
}

pub fn gen(ident: &Ident, items: &[ExternCItem]) -> Vec<Item> {
    let fields_def = items.iter().map(
        |ExternCItem {
             ident,
             ty,
             doc,
             sig,
             ..
         }| {
            let (vis, doc) = if sig.is_some() {
                (quote!(), quote!())
            } else {
                (quote!(pub), quote!(#( #doc)*))
            };

            quote! {
                #doc
                #vis #ident: libloading_helper::Symbol<'lib, #ty>
            }
        },
    );

    let def = parse_quote! {
        pub struct #ident<'lib> {
            #( #fields_def ),*
        }
    };

    let fields_value = items.iter().map(
        |ExternCItem {
             ident, ty, symbol, ..
         }| quote!(#ident: library.get::<#ty>(#symbol)?),
    );

    let fn_impl = items.iter().filter_map(|item| {
        let ExternCItem {
            ident, doc, sig, ..
        } = item;
        if let Some(Signature { inputs, output, .. }) = sig {
            let args = inputs.iter().filter_map(|a| {
                if let FnArg::Typed(t) = a {
                    Some(&t.pat)
                } else {
                    None
                }
            });

            let inputs = inputs.iter();
            Some(quote! {
                #( #doc )*
                pub unsafe fn #ident(&self, #( #inputs ),*) #output {
                    (self.#ident)( #( #args ),* )
                }
            })
        } else {
            None
        }
    });

    let impl_ = parse_quote! {
        impl<'lib> #ident<'lib> {
            pub unsafe fn load(library: &'lib libloading_helper::Library) -> std::result::Result<Self, libloading_helper::Error> {
                std::result::Result::Ok(Self {
                    #( #fields_value ),*
                })
            }

            #( #fn_impl )*
        }
    };

    vec![def, impl_]
}

pub struct ExternCItem {
    pub ident: Ident,
    pub ty: Type,
    pub doc: Vec<Attribute>,
    pub symbol: LitByteStr,
    pub sig: Option<Signature>,
}

impl ExternCItem {
    pub fn new(ident: &Ident, attrs: &[Attribute], ty: Type, sig: Option<Signature>) -> Self {
        let name = ident.to_string();
        let symbol = LitByteStr::new(
            CString::new(name.as_bytes()).unwrap().as_bytes_with_nul(),
            ident.span(),
        );

        let doc = get_doc(attrs);

        Self {
            ident: ident.clone(),
            ty,
            doc,
            symbol,
            sig,
        }
    }
}

impl From<&ForeignItemFn> for ExternCItem {
    fn from(item: &ForeignItemFn) -> Self {
        let args = item.sig.inputs.iter();
        let v = item.sig.variadic.iter();
        let out = &item.sig.output;

        let ty = parse_quote!(unsafe extern "C" fn(#(#args),* #(, #v)*) #out);

        Self::new(&item.sig.ident, &item.attrs, ty, Some(item.sig.clone()))
    }
}

impl From<&ForeignItemStatic> for ExternCItem {
    fn from(item: &ForeignItemStatic) -> Self {
        let ty = &item.ty;
        let ty = parse_quote!(*mut #ty);

        Self::new(&item.ident, &item.attrs, ty, None)
    }
}

fn get_doc(attrs: &[Attribute]) -> Vec<Attribute> {
    attrs
        .iter()
        .filter(|attr| attr.path.segments.len() == 1 && attr.path.segments[0].ident == "doc")
        .cloned()
        .collect()
}
