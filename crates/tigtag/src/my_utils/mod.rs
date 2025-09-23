use super::*;

pub mod init_app; // アプリ初期化とアセットのロード

pub mod misc; // 共通
pub use misc::I32x2TypeExt; // トレイトの公開

pub mod appctrl_input; // アプリの汎用的な操作
pub mod simple_camera; // シンプルカメラ
pub mod header_footer; // シンプル ヘッダー＆フッター

// End of code.
