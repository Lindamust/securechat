use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod expand;
mod model;
mod parse;

use expand::expand_impl;
use parse::parse_input;

#[proc_macro_derive(ValidateDto, attributes(validate, from))]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let model = parse_input(input);
    expand_impl(model).into()
}
