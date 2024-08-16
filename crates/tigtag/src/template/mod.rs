use super::*;

////////////////////////////////////////////////////////////////////////////////

//アプリの設定
mod config;
pub use config::*; //名前を公開

//型定義
mod types;
pub use types::*; //名前を公開

//ユーティリティ
pub mod misc; //misc::xxxxでのアクセスを許可
pub use misc::constants::*; //名前を公開

//debug用
mod debug;

//Plugins
mod main_window;
mod load_assets;
mod init_app;

//プラグインの設定
pub struct Schedule;
impl Plugin for Schedule
{   fn build( &self, app: &mut App )
    {   app
        .add_plugins( main_window::Schedule ) //メインウィンドウ設定と簡易表示テスト
        .init_state::<MyState>()              //Stateの初期値はenumの#[default]で指定する
        .add_plugins( load_assets::Schedule ) //assetsの事前ロード
        .add_plugins( init_app::Schedule    ) //事前処理
        ;
    }
}

////////////////////////////////////////////////////////////////////////////////

//End of code.