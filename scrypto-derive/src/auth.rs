use proc_macro2::{TokenStream};
use quote::quote;
use syn::*;

pub fn handle_auth(input: TokenStream) -> Result<TokenStream> {
    let result = quote! {
        ::scrypto::resource::AuthRule::ProofRule(#input)
    };
    Ok(result)
}