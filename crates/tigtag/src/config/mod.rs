use super::*;

////////////////////////////////////////////////////////////////////////////////

// サブモジュール（pub modしない）
mod initialize; // アプリの初期化
mod input; // 入力処理の設定
mod state; // 状態管理の設定
mod assets; // アセット関係
mod camera; // カメラの設定

mod ui_top_bottom; // ヘッダー・フッターの設定

////////////////////////////////////////////////////////////////////////////////

// 親モジュールへ識別子を輸出する
// #[rustfmt::skip]
pub mod common
{
    // サブモジュール名を不要にしたい識別子はpub useする
    pub use super::initialize::common::*;
    pub use super::input::*;
    pub use super::state::*;
    pub use super::assets::*;
    pub use super::camera::*;

    pub use super::ui_top_bottom::*;
}

////////////////////////////////////////////////////////////////////////////////

// mod general_settings; // ゲーム関係の諸々の設定
// mod overlay_ui_settings; // 全画面UIの設定

// 名前空間のトップレベルへ識別子を輸出する
// pub mod prelude
// {
//     // pub use super::general_settings::*;
//     // pub use super::overlay_ui_settings::*;
// }

// End of code.
