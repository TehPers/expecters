mod expect_inner;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

use crate::expect_inner::ExpectInner;

/// Internal macro for `expecters`.
///
/// Outputs the raw output of an assertion.
#[proc_macro]
#[doc(hidden)]
pub fn expect_inner(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ExpectInner);
    input.to_token_stream().into()
}
