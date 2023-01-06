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
    let input = parse_macro_input!(input);
    let action = get_action(&input);

    let DeriveInput { ident, data, .. } = input;

    let arguments_stream = build_arguments(data);
    quote! {
        impl IntoShortnameRPCEvent for #ident {
            fn action_shortname(&self) -> u32 {
                #action
            }

        