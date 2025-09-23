use super::*;

////////////////////////////////////////////////////////////////////////////////

// #[derive_appctrl_input]
pub fn derive_appctrl(input: TokenStream) -> TokenStream
{
    // 入力を分解する
    let ast: syn::DeriveInput = match syn::parse2(input)
    {
        Ok(ast) => ast,
        Err(err) => return err.to_compile_error(),
    };

    // 既存のフィールド情報を抽出してTokenStreamに変換
    let existing_fields = match &ast.data
    {
        syn::Data::Struct(s) => match &s.fields
        {
            // struct MyStruct { a: i32 }のような名前付きフィールドの場合
            syn::Fields::Named(fields) =>
            {
                let fields = &fields.named;
                // 既存フィールドのリスト＋区切りのカンマを生成
                quote! { #fields, }
            }
            // struct MyStruct; のようなユニット構造体の場合
            syn::Fields::Unit =>
            {
                // フィールドは無いので空のTokenStreamを返す
                quote! {}
            }
            // タプル構造体は今回サポートしない
            syn::Fields::Unnamed(_) =>
                panic!("AppCtrl does not support tuple structs."),
        },
        // 構造体以外（enumなど）はサポートしない
        _ => panic!("AppCtrl can only be used on structs."),
    };

    // 構造体の情報（構造体名、公開(pub)、ジェネリクス）を取得
    let type_name = &ast.ident;
    let is_pub = &ast.vis;
    let generics = &ast.generics;

    // 文字列を作成して出力する
    quote! {
        #is_pub struct #type_name #generics {
            #existing_fields

            pub keys: Vec<MainKeyAndModifiers>,
            pub buttons: Vec<GamepadButton>,
        }
        impl AppCtrl for #type_name
        {
            fn keys_iter(&self) -> Iter<'_, MainKeyAndModifiers> { self.keys.iter() }
            fn buttons_iter(&self) -> Iter<'_, GamepadButton> { self.buttons.iter() }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
