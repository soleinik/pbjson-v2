#![recursion_limit = "128"]

mod pb_float;

use proc_macro::TokenStream;

#[proc_macro_derive(SerializeFloat32)]
pub fn derive_serialize_float32(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    pb_float::impl_serialize_float32(&ast)
}
