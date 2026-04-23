
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;


#[proc_macro_derive(Component)]
pub fn component_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    
    impl_component_macro(&ast)
}

fn impl_component_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generate = quote! {
        impl Component for #name {
            fn name(&self) -> &str {
                stringify!(#name)
            }
        }
    };

    generate.into()
} 