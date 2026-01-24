////////////////////////////////////////////////////////////////////////////////

// サブモジュール（pub modしない）
mod controller; // 入力管理
#[cfg(not(target_arch = "wasm32"))]
mod fullscreen; // フルスクリーン切替処理
mod misc; // 共通・その他


////////////////////////////////////////////////////////////////////////////////

// 親モジュールへ識別子を輸出する
#[rustfmt::skip]
pub mod common
{
    // 原則はサブモジュール名を必須とする
    pub mod controller {pub use super::super::controller::*;}
    #[cfg(not(target_arch = "wasm32"))]
    pub mod fullscreen {pub use super::super::fullscreen::*;}
    pub mod misc       {pub use super::super::misc::*;}
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
