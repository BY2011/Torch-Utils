use darling::FromDeriveInput;
use proc_macro::{self, TokenStream};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, FieldsNamed};

#[derive(FromDeriveInput, Default)]
#[darling(attributes(rpc_msg), forward_attrs(allow, doc, cfg))]
struct RpcMsgOpts {
    action: u32,
}

#[proc_macro_derive(IntoShortnameRPCEvent, attributes(rpc_msg))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input