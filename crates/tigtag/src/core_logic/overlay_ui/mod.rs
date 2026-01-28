// use super::*;

////////////////////////////////////////////////////////////////////////////////

// サブモジュール（pub modしない）
mod messages; // 全画面メッセージのspawn
mod effect; // 全画面メッセージの表示効果
mod pause_menu; // Pauseメニュー関連

////////////////////////////////////////////////////////////////////////////////

// 親モジュールへ識別子を輸出する
#[rustfmt::skip]
pub mod common
{
    // 原則はサブモジュール名を必須とする
    // pub mod misc          {pub use super::super::misc::*;}

    // サブモジュール名を不要にしたい識別子はpub useする
    // pub use super::core_utils::I32x2TypeExt;
    // pub use super::core_utils::set_next_state;
}

////////////////////////////////////////////////////////////////////////////////


// End of code.
