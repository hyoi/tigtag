use super::*;

// internal submodules
mod spawn_textui; //ポップアップメッセージのspawn
pub use spawn_textui::*; //下位モジュールから輸入した識別子を上位へ輸出

pub mod effect; //ポップアップメッセージの表示効果

// End of code.
