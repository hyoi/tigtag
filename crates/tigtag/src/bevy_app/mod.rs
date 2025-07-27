use super::*;

// internal submodules
mod game_logic; // ゲーム本体
pub use game_logic::Schedule;

mod consts_and_types; // 定数 ＆ 型定義
use consts_and_types::*; // 識別子をモジュール名不要にして外へ公開

mod map; // 迷路生成
pub mod player; // プレイヤー
pub mod chaser; // チェイサー

mod animate_sprites; //スプライトアニメーション
use animate_sprites::*;

mod detecting_change; // ステージクリアとゲームオーバーの判定

pub mod header_info; //ヘッダー情報
use header_info::*;

// End of code.
