use proc_macro::TokenStream;
use quote::quote;

pub fn impl_map_object(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl MapObject for #name {
            fn map_object_capabilities() -> MapObjectCapabilities {
                fn get_id(handle: HandleUntyped) -> String {
                    let node = scene::get_untyped_node(handle).unwrap().to_typed::<#name>();
                    node.id.to_string()
                }

                fn get_position(handle: HandleUntyped) -> Vec2 {
                    let node = scene::get_untyped_node(handle).unwrap().to_typed::<#name>();
                    node.position
                }

                MapObjectCapabilities {
                    get_id,
                    get_position,
                }
            }

            fn apply_map_object_provider<T>(mut node: RefMut<T>) {
                node.provides::<MapObjectProvider>((
                    node.handle().untyped(),
                    Self::map_object_capabilities(),
                ));
            }
        }
    };
    gen.into()
}
