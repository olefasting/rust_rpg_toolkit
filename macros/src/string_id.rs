use proc_macro::TokenStream;
use quote::quote;

pub fn impl_get_string_id(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl GetStringId for #name {
            fn get_id(&self) -> String {
                self.id.clone()
            }
        }
    };
    gen.into()
}

pub fn impl_set_string_id(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl SetStringId for #name {
            fn set_id(&mut self, id: &str) {
                self.id = id.to_string();
            }
        }
    };
    gen.into()
}

pub fn impl_string_id(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl StringId for #name {
            fn get_id(&self) -> String {
                self.id.clone()
            }

            fn set_id(&mut self, id: &str) {
                self.id = id.to_string();
            }
        }
    };
    gen.into()
}
