use proc_macro::TokenStream;
use quote::quote;

pub fn impl_global_value(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl GlobalValue<#name> for #name {
            fn try_get_global() -> Option<#name> {
                match storage::try_get::<#name>() {
                    Some(val) => Some(*val),
                    _ => None,
                }
            }

            fn get_global() -> #name {
                *storage::get::<#name>()
            }

            fn set_global(self) {
                storage::store::<#name>(self);
            }
        }
    };
    gen.into()
}
