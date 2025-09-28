use super::*;

////////////////////////////////////////////////////////////////////////////////

// #[derive( OverlayMenu )]
pub fn derive_overlay_menu(input: TokenStream) -> TokenStream
{
    // 入力を分解する
    let ast: syn::DeriveInput = match syn::parse2(input)
    {
        Ok(ast) => ast,
        Err(err) => return err.to_compile_error(),
    };

    // 構造体の内容が想定外なら
    let required_field_type: &[(&str, &str)] = &[(
        "overlay_menu",
        "core_logic::overlay_ui::pause_menu::OverlayMenuParams",
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
        impl core_logic::overlay_ui::pause_menu::OverlayMenu for #type_name
        {
            fn init(&mut self) { *self = Self::default() }
            fn settings(&self) -> &core_logic::overlay_ui::pause_menu::MenuItemSettings
            {
                &self.overlay_menu.settings
            }
            fn selected_index_mut(&mut self) -> &mut i32 { &mut self.overlay_menu.selected_index }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

// #[derive( ScalingItem )]
pub fn derive_scaling_item(input: TokenStream) -> TokenStream
{
    // 入力を分解する
    let ast: syn::DeriveInput = match syn::parse2(input)
    {
        Ok(ast) => ast,
        Err(err) => return err.to_compile_error(),
    };

    // 構造体の内容が想定外なら
    let required_field_type: &[(&str, &str)] = &[(
        "scaling_item",
        "core_logic::overlay_ui::pause_menu::ScalingItemParams",
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
        impl core_logic::overlay_ui::pause_menu::ScalingItem for #type_name
        {
            fn scale_cycle_mut(&mut self) -> &mut f32 { &mut self.scaling_item.scale_cycle }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
