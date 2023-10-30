use quote::quote;

pub(super) fn impl_serialize_float32(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl serde::Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
           {
               if self.value.is_nan() {
                   return Err(
                       serde::ser::Error::custom(
                           "Cannot serialize NaN as google.protobuf.Value.number_value",
                       )
                   );
               } else if self.value.is_infinite() {
                   return Err(
                       serde::ser::Error::custom(
                           "Cannot serialize infinity as google.protobuf.Value.number_value",
                       )
                   );
               }

               serializer.serialize_f32(self.value)
            }
        };
    };
    gen.into()
}
