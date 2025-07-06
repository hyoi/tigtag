use super::*;

// internal submodules
mod consts_and_types; // プレイヤーの定数と型の宣言
pub use consts_and_types::*;

mod player; // プレイヤーのスプライト処理
pub use player::*;

mod input; // キー・ゲームパッドの入力処理
pub use input::*;

// End of code.
