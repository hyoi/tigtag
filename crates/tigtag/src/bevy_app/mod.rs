use super::*;

// internal submodules
mod game_logic; // ゲーム本体
pub use game_logic::Schedule;

mod consts_and_types; // 定数 ＆ 型定義
use consts_and_types::*; // 識別子をモジュール名不要にして外へ公開

mod map; // 迷路生成
mod player; // プレイヤー
mod chasers; // チェイサー

// End of code.
