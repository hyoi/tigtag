use super::*;

////////////////////////////////////////////////////////////////////////////////

// #[derive( OverlayMessage )]
pub fn derive_overlay_message(input: TokenStream) -> TokenStream
{
    // 入力を分解する
    let ast: syn::DeriveInput = match syn::parse2(input)
    {
        Ok(ast) => ast,
        Err(err) => return err.to_compile_error(),
    };

    // 構造体の内容が想定外なら
    let required_field_type: &[(&str, &str)] = &[(
        "text_spans",
        "&'static[core_logic::overlay_ui::messages::TextUiSpan]",
    )];
    match is_valid_struct(&ast, required_field_type)
    {
        Ok(_) => (),
        Err(err) => return err.to_compile_error(),
    }

    // データから識別子(名前)を抽出する
    let type_name = ast.ident;

    // 文字列を作成して出力する
    quote! {
        impl core_logic::overlay_ui::messages::OverlayMessage for #type_name
        {
            fn text_spans(&self) -> &'static [core_logic::overlay_ui::messages::TextUiSpan]
            { self.text_spans }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

// #[derive( Blinking )]
pub fn derive_blinking(input: TokenStream) -> TokenStream
{
    // 入力を分解する
    let ast: syn::DeriveInput = match syn::parse2(input)
    {
        Ok(ast) => ast,
        Err(err) => return err.to_compile_error(),
    };

    // 構造体の内容が想定外なら
    let required_field_type: &[(&str, &str)] =
        &[("blinking", "core_logic::overlay_ui::effect::BlinkingParams")];
    match is_valid_struct(&ast, required_field_type)
    {
        Ok(_) => (),
        Err(err) => return err.to_compile_error(),
    }

    // データから識別子(名前)を抽出する
    let type_name = ast.ident;

    // 文字列を作成して出力する
    quote! {
        impl core_logic::overlay_ui::effect::Blinking for #type_name
        {
            fn alpha(&mut self, time_delta: f32) -> f32
            {
                let radian = &mut self.blinking.cycle;
                *radian += TAU * time_delta;
                *radian -= if *radian > TAU { TAU } else { 0.0 };

                (*radian).sin() * 0.5 + 0.5 //0.0 ～ 1.0
            }
            fn index(&self) -> usize { self.blinking.spans_index }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

// #[derive( CountDown )]
pub fn derive_countdown(input: TokenStream) -> TokenStream
{
    // 入力を分解する
    let ast: syn::DeriveInput = match syn::parse2(input)
    {
        Ok(ast) => ast,
        Err(err) => return err.to_compile_error(),
    };

    // 構造体の内容が想定外なら
    let required_field_type: &[(&str, &str)] = &[(
        "countdown",
        "core_logic::overlay_ui::effect::CountDownParams",
    )];
    match is_valid_struct(&ast, required_field_type)
    {
        Ok(_) => (),
        Err(err) => return err.to_compile_error(),
    }

    // データから識別子(名前)を抽出する
    let type_name = ast.ident;

    // 文字列を作成して出力する
    quote! {
        impl core_logic::overlay_ui::effect::CountDown for #type_name
        {
            fn index(&self) -> usize { self.countdown.spans_index }
            fn start_value(&self) -> i32 { self.countdown.start_value }
            fn timer(&mut self) -> &mut Timer { &mut self.countdown.timer }
            fn counter(&mut self) -> &mut i32 { &mut self.countdown.counter }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
