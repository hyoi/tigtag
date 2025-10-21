use super::*;

////////////////////////////////////////////////////////////////////////////////

// #[derive(OverlayMessage, Blinking, CountDown)]の実装
mod overlay_message;
pub use overlay_message::*;

// #[derive(OverlayMenu, ScalingItem)]の実装
mod overlay_menu;
pub use overlay_menu::*;

////////////////////////////////////////////////////////////////////////////////

// overlay_message と overlay_menu 共通の関数。
// 構造体か？ 必須のフィールドがあるか？ を調べる。
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
