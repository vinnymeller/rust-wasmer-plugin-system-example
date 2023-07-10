#![recursion_limit = "128"]
extern crate proc_macro;
use proc_macro::TokenStream;

use proc_macro2::{Ident, Span};
use quote::quote;
use syn::Item as SynItem;

#[proc_macro_attribute]
pub fn plugin_helper(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    let tokens2 = proc_macro2::TokenStream::from(tokens);
    let parse2 = syn::parse2::<SynItem>(tokens2).expect("Failed to parse tokens");
    match parse2 {
        SynItem::Fn(func) => handle_func(func),
        _ => panic!("Only functions are currently supported"),
    }
}

fn handle_func(func: syn::ItemFn) -> TokenStream {
    // check that the function only takes one argument and panic if not
    if func.sig.inputs.len() != 1 {
        panic!("fns marked with plugin_helper can only take 1 argument");
    }
    // copy the function's identifier
    let ident = func.sig.ident.clone();
    // create a new identifier with a underscore in front of the original
    let shadows_ident = Ident::new(&format!("_{}", ident), Span::call_site());
    // generate some rust with the original and new shadowed
    let ret = quote! {
        #func

        #[no_mangle]
        pub fn #shadows_ident(ptr: i32, len: u32) -> i32 {
            let value = unsafe {
                std::slice::from_raw_parts(ptr as _, len as _)
            };
            let arg = convert_data(value);
            let ret = #ident(arg);
            let bytes = revert_data(&ret);
            let len = bytes.len() as u32;
            unsafe {
                std::ptr::write(1 as _, len);
            }
            bytes.as_ptr() as _
        }
    };
    ret.into()
}
