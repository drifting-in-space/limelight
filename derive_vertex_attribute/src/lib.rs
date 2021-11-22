extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemStruct;

#[proc_macro_derive(VertexAttribute)]
pub fn vertex_attribute_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impl_vertex_attribute_derive(input.into()).into()
}

fn bind(field: &syn::Field) -> TokenStream {
    let name = field.ident.as_ref().unwrap().to_string();
    let kind = &field.ty;

    quote! {
        gl_layers::vertex_attribute::VertexAttributeBinding {
            variable_name: (#name).to_string(),
            kind: <#kind as gl_layers::types::AsSizedDataType>::as_sized_data_type(),
        }
    }
}

fn impl_vertex_attribute_derive(input: TokenStream) -> TokenStream {
    let ast: ItemStruct = syn::parse2(input).expect("Should decorate a struct.");

    let name = &ast.ident;

    let bindings: Vec<TokenStream> = match &ast.fields {
        syn::Fields::Named(fields) => fields.named.iter().map(bind).collect(),
        _ => panic!("Only structs with named fields can derive StateMachine currently."),
    };

    quote! {
        impl VertexAttribute for #name {
            fn describe() -> Vec<gl_layers::vertex_attribute::VertexAttributeBinding> {
                vec![
                    #(#bindings),*
                ]
            }
        }
    }
}
