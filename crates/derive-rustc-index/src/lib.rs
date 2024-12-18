mod internal;

#[proc_macro_derive(Idx)]
pub fn derive_idx(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    internal::derive_idx(input).into()
}
