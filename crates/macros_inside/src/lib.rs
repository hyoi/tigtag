// import external modules
use proc_macro2::TokenStream;
use quote::*;
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////

//#[derive( MyState )]
pub fn derive_mystate(input: TokenStream) -> TokenStream
{
    // 入力を分解する
    let ast: syn::DeriveInput = match syn::parse2(input)
    {
        Ok(ast) => ast,
        Err(err) => return err.to_compile_error(),
    };

    // データから識別子(名前)を抽出する
    let enum_type = ast.ident;
    let mut enum_variant = Vec::new();
    let mut is_variant = Vec::new();

    if let syn::Data::Enum(my_enum) = ast.data
    {
        for my_variant in my_enum.variants.into_iter()
        {
            {
                let lower_ident = my_variant.ident.to_string().to_lowercase();
                enum_variant.push(my_variant.ident);
                is_variant.push(format_ident!("is_{}", lower_ident));
            }
        }
    }
    else
    {
        panic!("Applicable to Enum only.")
    }

    // 文字列を作成して出力する
    quote! {
        //MyStateの遷移に使うTrait境界
        pub trait ChangeMyState
        {   fn state( &self ) -> #enum_type;
        }

        //バリアントと同名のstructからバリアントを取得させるための仕込み
        #(  #[derive( Default )]
            pub struct #enum_variant;
            impl ChangeMyState for #enum_variant
            {   fn state( &self ) -> #enum_type { #enum_type::#enum_variant }
            }
        )*

        //同名structによって指定されたMyStateへ遷移するSystem
        pub fn set_next_state<T: Send + Sync + Default + ChangeMyState>
        (   next: Local<T>,
            mut next_state: ResMut<NextState<#enum_type>>
        )
        {   next_state.set( next.state() );
        }

        //ResourceにセットされたMyStateへ遷移するSystem
        pub fn change_state_by<T: Resource + ChangeMyState>
        (   opt_state: Option<Res<T>>,
            mut next_state: ResMut<NextState<#enum_type>>
        )
        {   let Some ( next ) = opt_state else { warn!( "opt_state is None." ); return };
            next_state.set( next.state() );
        }

        // impl MyState
        // {   pub fn is_pause( &self ) -> bool { *self == MyState::Pause }
        //         :
        //         :
        // }
        impl #enum_type
        {   #(  pub fn #is_variant ( &self ) -> bool { *self == #enum_type::#enum_variant }
            )*
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

//#[derive( CountDown )]
pub fn derive_countdown(input: TokenStream) -> TokenStream
{
    // 入力を分解する
    let ast: syn::DeriveInput = match syn::parse2(input)
    {
        Ok(ast) => ast,
        Err(err) => return err.to_compile_error(),
    };

    // 構造体の内容が想定外なら
    let required_field_type: &[(&str, &str)] = &[("countdown", "CountDownParams")];
    match is_valid_struct(&ast, required_field_type)
    {
        Ok(_) => (),
        Err(err) => return err.to_compile_error(),
    }

    // データから識別子(名前)を抽出する
    let type_name = ast.ident;

    // 文字列を作成して出力する
    quote! {
        impl popup_text_ui::effect::CountDown for #type_name
        {
            fn init(&mut self) { *self = Self::default(); }
            fn index(&self) -> usize { self.countdown.spans_index }
            fn start_value(&self) -> i32 { self.countdown.start_value }
            fn timer(&mut self) -> &mut Timer { &mut self.countdown.timer }
            fn counter(&mut self) -> &mut i32 { &mut self.countdown.counter }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

//#[derive( Blinking )]
pub fn derive_blinking(input: TokenStream) -> TokenStream
{
    // 入力を分解する
    let ast: syn::DeriveInput = match syn::parse2(input)
    {
        Ok(ast) => ast,
        Err(err) => return err.to_compile_error(),
    };

    // 構造体の内容が想定外なら
    let required_field_type: &[(&str, &str)] = &[("blinking", "BlinkingParams")];
    match is_valid_struct(&ast, required_field_type)
    {
        Ok(_) => (),
        Err(err) => return err.to_compile_error(),
    }

    // データから識別子(名前)を抽出する
    let type_name = ast.ident;

    // 文字列を作成して出力する
    quote! {
        impl popup_text_ui::effect::Blinking for #type_name
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

//#[derive( HitAnyKey )]
pub fn derive_hitanykey(input: TokenStream) -> TokenStream
{
    // 入力を分解する
    let ast: syn::DeriveInput = match syn::parse2(input)
    {
        Ok(ast) => ast,
        Err(err) => return err.to_compile_error(),
    };

    // 構造体の内容が想定外なら
    let required_field_type: &[(&str, &str)] = &[("hit_any_key", "HitAnyKeyParams")];
    match is_valid_struct(&ast, required_field_type)
    {
        Ok(_) => (),
        Err(err) => return err.to_compile_error(),
    }

    // データから識別子(名前)を抽出する
    let type_name = ast.ident;

    // 文字列を作成して出力する
    quote! {
        impl popup_text_ui::effect::HitAnyKey for #type_name
        {
            fn ignore_keys(&self) -> &[KeyCode] { self.hit_any_key.ignore_keys }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

// 構造体か？ 必須のフィールドがあるか？ を調べる
fn is_valid_struct(
    ast: &syn::DeriveInput,
    required_field_type: &[(&str, &str)],
) -> Result<(), syn::Error>
{
    // 構造体ではないなら
    let struct_fields = match &ast.data
    {
        syn::Data::Struct(s) => &s.fields,
        _ =>
        {
            let error_span = ast.ident.span();
            let error_msg = "This macro can only be used on structs.";
            return Err(syn::Error::new(error_span, error_msg));
        }
    };

    // 構造体に名前付きフィールドがないなら
    let named_fields = match struct_fields
    {
        syn::Fields::Named(fields) => &fields.named,
        _ =>
        {
            let error_span = ast.ident.span();
            let error_msg = "This macro requires named fields.";
            return Err(syn::Error::new(error_span, error_msg));
        }
    };

    // 必須のフィールド名と型のペアで連想配列を作る
    let fields_hash_map = named_fields
        .iter()
        .filter_map(|field| {
            field
                .ident
                .as_ref()
                .map(|ident| (ident.to_string(), &field.ty))
        })
        .collect::<HashMap<_, _>>();

    // 必須のフィールド名と型のペアが全て存在するか確認する
    for (required_name, required_type_str) in required_field_type
    {
        match fields_hash_map.get(*required_name)
        {
            Some(field_type) =>
            {
                // 型を文字列に変換する
                let field_type_str =
                    quote! {#field_type}.to_string().replace(' ', "");

                // フィールド名は存在するが、型が一致しないなら
                if &field_type_str != required_type_str
                {
                    let err = syn::Error::new_spanned(
                        field_type,
                        format!(
                            "Invalid type for field `{}`. Expected `{}`, but found `{}`.",
                            required_name, required_type_str, field_type_str
                        ),
                    );
                    return Err(err);
                }
            }
            None =>
            {
                // フィールド名が存在しなかった場合
                let err = syn::Error::new_spanned(
                    &ast.ident,
                    format!("The required field `{}` is missing.", required_name),
                );
                return Err(err);
            }
        }
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
