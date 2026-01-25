use super::*;

////////////////////////////////////////////////////////////////////////////////

// サブモジュール（pub modしない）
mod misc; // 共通

////////////////////////////////////////////////////////////////////////////////

// 親モジュールへ識別子を輸出する
#[rustfmt::skip]
pub mod common
{
    // 原則はサブモジュール名を必須とする
    pub mod misc          {pub use super::super::misc::*;}

    // サブモジュール名を不要にしたい識別子はpub useする
    // pub use super::controller::InputDeviceIsPressed;
}

////////////////////////////////////////////////////////////////////////////////

// pub mod init_app; // アセットのロードと各種の初期化
// pub mod appctrl_input; // アプリの汎用的な操作
// pub mod simple_camera; // シンプルカメラ
// pub mod header_footer; // シンプルヘッダー＆フッター
// pub mod handle_input; // 入力処理
// pub mod orbit_camera; // 球座標カメラ

// 名前空間のトップレベルへ識別子を輸出する
// #[allow(unused_imports)]
// pub mod prelude
// {

//     // pub use super::init_app;

//     // pub use super::misc::prelude::*;

//     // pub use super::appctrl_input;
//     // pub use super::simple_camera;
//     // pub use super::header_footer;

//     // pub use super::handle_input;
//     // pub use super::handle_input::prelude::*;

//     // pub use super::orbit_camera;
// }

// End of code.
