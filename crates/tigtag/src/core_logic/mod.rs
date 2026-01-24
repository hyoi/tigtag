use super::*;

////////////////////////////////////////////////////////////////////////////////

// サブモジュール（pub modしない）
mod schedule; // メインスケジュール
// mod load_assets; // アセットの事前ローディング
// mod core_utils; // 共通その他

//------------------------------------------------------------------------------

// core_logic::Schedule として公開したい
pub use schedule::Schedule;

////////////////////////////////////////////////////////////////////////////////

// 親モジュールへ識別子を輸出する
#[rustfmt::skip]
pub mod common
{
    // 原則はサブモジュール名を必須とする
    // pub mod misc {pub use super::super::misc::*;}

    // サブモジュール名を不要にしたい識別子はpub useする
    // pub use super::core_utils::set_next_state;
    // pub use super::core_utils::I32x2TypeExt;
}

////////////////////////////////////////////////////////////////////////////////

// mod consts_and_types; // 定数＆型定義
// pub use consts_and_types::*;

// pub mod information; // 表示情報更新（ヘッダー・フッター）
// pub mod overlay_ui; // 全画面メッセージ関連
// use overlay_ui::messages::OverlayMessage;

// pub mod detecting_change; // ステージクリアとゲームオーバーの判定

// pub mod map; // 迷路生成
// pub mod player; // プレイヤー
// pub mod chaser; // チェイサー

// mod animate_sprites; //スプライトアニメーション
// pub use animate_sprites::*;

// End of code.
