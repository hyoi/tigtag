////////////////////////////////////////////////////////////////////////////////

// サブモジュール（pub modしない）
mod misc; // 共通
mod simple_camera; // シンプルカメラ

////////////////////////////////////////////////////////////////////////////////

// 親モジュールへ識別子を輸出する
#[rustfmt::skip]
pub mod common
{
    // 原則はサブモジュール名を必須とする
    pub mod misc          {pub use super::super::misc::*;}
    pub mod simple_camera {pub use super::super::simple_camera::*;}

    // サブモジュール名を不要にしたい識別子はpub useする
    // pub use super::controller::InputDeviceIsPressed;
}

////////////////////////////////////////////////////////////////////////////////

// pub mod header_footer; // シンプルヘッダー＆フッター
// pub mod handle_input; // 入力処理
// pub mod orbit_camera; // 球座標カメラ

// 名前空間のトップレベルへ識別子を輸出する
// #[allow(unused_imports)]
// pub mod prelude
// {
//     // pub use super::header_footer;

//     // pub use super::handle_input;
//     // pub use super::handle_input::prelude::*;

//     // pub use super::orbit_camera;
// }

// End of code.
