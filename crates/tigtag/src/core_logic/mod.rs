use super::*;

////////////////////////////////////////////////////////////////////////////////

// サブモジュール（pub modしない）
mod config; // 設定ファイル
mod schedule; // メインスケジュール
mod load_assets; // アセットの事前ローディング
mod core_utils; // 共通その他

mod overlay_ui; // 全画面メッセージ関連

// mod demo_play; // demoロジック

//------------------------------------------------------------------------------

// core_logic::Schedule として公開したい
pub use schedule::Schedule;

////////////////////////////////////////////////////////////////////////////////

// 親モジュールへ識別子を輸出する
#[rustfmt::skip]
pub mod common
{
    // 原則はサブモジュール名を必須とする
    pub mod overlay_ui {pub use super::super::overlay_ui::*;}

    // サブモジュール名を不要にしたい識別子はpub useする
    pub use super::config::common::*;
    pub use super::core_utils::I32x2TypeExt;
    pub use super::core_utils::set_next_state;
}

////////////////////////////////////////////////////////////////////////////////

// mod consts_and_types; // 定数＆型定義
// pub use consts_and_types::*;

// pub mod information; // 表示情報更新（ヘッダー・フッター）

// pub mod detecting_change; // ステージクリアとゲームオーバーの判定

// pub mod map; // 迷路生成
// pub mod player; // プレイヤー
// pub mod chaser; // チェイサー

// mod animate_sprites; //スプライトアニメーション
// pub use animate_sprites::*;

// End of code.
