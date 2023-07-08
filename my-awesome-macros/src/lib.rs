//! My Awesome Macro crate
//!
//! ## Home for rust macros

extern crate proc_macro;
use proc_macro::TokenStream;

/// ## simple_func_lke_macro!()
///
/// In the following example simply ignores the input\
/// and replace with following func definition irrespective of input
///
/// <pre>
///   fn give_me_number() -> u32 {
///     86543
///   }
/// </pre>
///
#[proc_macro]
pub fn simple_func_like_macro(_item: TokenStream) -> TokenStream {
    "fn give_me_the_lucky_number() -> i32 { 86543 }"
        .parse()
        .unwrap()
}

/// JustForFunNoDerive capability
/// I'll not make any changes in the code
///
#[proc_macro_derive(JustForFunNoDerive, attributes(field_one, field_two))]
pub fn just_for_fun_no_derive(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}
