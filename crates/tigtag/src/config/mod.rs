use super::*;

mod general_settings; // ゲーム関係の諸々の設定
mod header_footer_settings; // ヘッダー・フッターの設定
mod overlay_ui_settings; // 全画面UIの設定

// 名前空間のトップレベルへ識別子を輸出する
pub mod prelude
{
    pub use super::general_settings::*;
    pub use super::header_footer_settings::*;
    pub use super::overlay_ui_settings::*;
}

// End of code.
