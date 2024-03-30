//import external modules
use proc_macro::TokenStream;

//macros_inside::derive_mystateのラッパー
#[proc_macro_derive( MyState )]
pub fn derive_mystate( input: TokenStream ) -> TokenStream
{   macros_inside::derive_mystate( input.into() ).into()
}

//End of code.