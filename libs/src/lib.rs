mod log;

use log::log_duration_impl;

use proc_macro::TokenStream;
use quote::quote;
use syn::Data;

#[proc_macro_attribute]
pub fn log_duration(args: TokenStream, item: TokenStream) -> TokenStream {
    log_duration_impl(args, item)
}

