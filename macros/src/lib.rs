#![recursion_limit="128"]

extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;

mod global_value;
mod string_id;
mod map_object;

use global_value::impl_global_value;
use map_object::impl_map_object;
use string_id::{
    impl_get_string_id,
    impl_set_string_id,
    impl_string_id,
};

#[proc_macro_derive(MapObject)]
pub fn map_object_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_map_object(&ast)
}

#[proc_macro_derive(GetStringId)]
pub fn get_string_id_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_get_string_id(&ast)
}

#[proc_macro_derive(SetStringId)]
pub fn set_string_id_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_set_string_id(&ast)
}

#[proc_macro_derive(StringId)]
pub fn string_id_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_string_id(&ast)
}

#[proc_macro_derive(GlobalValue)]
pub fn global_value_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_global_value(&ast)
}
