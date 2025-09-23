// import external modules
use proc_macro::TokenStream;

////////////////////////////////////////////////////////////////////////////////

#[proc_macro_derive(MyState)]
pub fn derive_mystate(input: TokenStream) -> TokenStream
{
    macros_inside::derive_mystate(input.into()).into()
}

////////////////////////////////////////////////////////////////////////////////

#[proc_macro_derive(OverlayMessage)]
pub fn derive_overlay_message(input: TokenStream) -> TokenStream
{
    macros_inside::derive_overlay_message(input.into()).into()
}

#[proc_macro_derive(Blinking)]
pub fn derive_blinking(input: TokenStream) -> TokenStream
{
    macros_inside::derive_blinking(input.into()).into()
}

#[proc_macro_derive(CountDown)]
pub fn derive_countdown(input: TokenStream) -> TokenStream
{
    macros_inside::derive_countdown(input.into()).into()
}

////////////////////////////////////////////////////////////////////////////////

#[proc_macro_derive(OverlayMenu)]
pub fn derive_overlay_menu(input: TokenStream) -> TokenStream
{
    macros_inside::derive_overlay_menu(input.into()).into()
}

#[proc_macro_derive(ScalingItem)]
pub fn derive_scaling_item(input: TokenStream) -> TokenStream
{
    macros_inside::derive_scaling_item(input.into()).into()
}

////////////////////////////////////////////////////////////////////////////////

#[proc_macro_attribute]
pub fn derive_appctrl_input(_attr: TokenStream, item: TokenStream) -> TokenStream
{
    macros_inside::derive_appctrl(item.into()).into()
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
