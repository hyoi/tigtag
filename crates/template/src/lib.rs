//external crates
use bevy::
{   prelude::*,
    log::LogPlugin,
    color::palettes::css,
    window::WindowMode,
    input::mouse::{ MouseMotion, MouseWheel },
    ecs::query::QueryFilter,
    asset::{ LoadState, LoadedUntypedAsset },
    diagnostic::{ FrameTimeDiagnosticsPlugin, DiagnosticsStore },
    utils::Duration,
};
use rand::prelude::*;
use chrono::prelude::Local as time_local; //「Local」がbevyとバッティングするのでaliasを使う

//standard library
use std::
{   sync::LazyLock,
    f32::consts::{ PI, TAU },
    ops::Range,
};

//アプリの設定
mod config;
pub use config::*; //名前を公開

//proc-macro crates
use macros::MyState;

//型定義
mod types;
pub use types::*; //名前を公開

//ユーティリティ
pub mod misc; //misc::xxxxでのアクセスを許可
pub use misc::constants::*; //名前を公開

//debug用
mod debug;

////////////////////////////////////////////////////////////////////////////////

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