// import external modules
use proc_macro::TokenStream;

// macros_inside::derive_mystateのラッパー
#[proc_macro_derive(MyState)]
pub fn derive_mystate(input: TokenStream) -> TokenStream
{
    macros_inside::derive_mystate(input.into()).into()
}

// macros_inside::derive_countdownのラッパー
#[proc_macro_derive(CountDown)]
pub fn derive_countdown(input: TokenStream) -> TokenStream
{
    macros_inside::derive_countdown(input.into()).into()
}

// macros_inside::derive_blinkingのラッパー
#[proc_macro_derive(Blinking)]
pub fn derive_blinking(input: TokenStream) -> TokenStream
{
    macros_inside::derive_blinking(input.into()).into()
}

// macros_inside::derive_hitanykeyのラッパー
#[proc_macro_derive(HitAnyKey)]
pub fn derive_hitanykey(input: TokenStream) -> TokenStream
{
    macros_inside::derive_hitanykey(input.into()).into()
}

// End of code.
