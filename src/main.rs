//external crates
use bevy::
{   prelude::*,
    log::LogPlugin,
    window::WindowMode,
    render::camera,
    asset::{ LoadState, LoadedUntypedAsset },
    diagnostic::{ DiagnosticsStore, FrameTimeDiagnosticsPlugin },
    sprite::{ Anchor, MaterialMesh2dBundle },
    utils::{ HashSet, HashMap },
    audio::{ Volume, VolumeLevel },
};
use once_cell::sync::Lazy;
use counted_array::counted_array;
use rand::prelude::*;
use regex::Regex;

//standard library
use std::ops::{ Range, Add, AddAssign };
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::f32::consts::{ PI, TAU };

//internal submodules
mod public;
use public::*;

mod load_assets;
mod init_app;
mod play_game;
mod title_demo;

////////////////////////////////////////////////////////////////////////////////

//メイン関数
fn main()
{   //アプリの生成
    let mut app = App::new();

    //メインウィンドウの設定
    let primary_window = MAIN_WINDOW.clone();
    let log_level = if DEBUG() { LOG_LEVEL_DEV } else { LOG_LEVEL_REL };
    let filter = log_level.into();
    app
    .insert_resource( Msaa::Sample4 ) //アンチエイリアス
    .insert_resource( ClearColor( SCREEN_BACKGROUND_COLOR ) ) //背景色
    .add_plugins
    (   DefaultPlugins
        .set( WindowPlugin { primary_window, ..default() } ) //メインウィンドウ
        .set( ImagePlugin::default_nearest() ) //ピクセルパーフェクト
        .set( LogPlugin { filter, ..default() } ) //ロギング
    )
    .add_systems
    (   Startup,
        (   //カメラを作る
            misc::spawn_2d_camera, //2D camera

            //テスト用：オブジェクト表示
            debug::spawn_2d_sprites //2D表示テスト
            .run_if( DEBUG )
            .run_if( not( resource_exists::<Map>() ) ) //debug表示が被るので
        )
    )
    .init_resource::<ConnectedGamepad>() //操作を受け付けるgamepadのID
    .add_systems
    (   Update,
        (   misc::choose_gamepad_connection, //<ConnectedGamepad>を設定
            (   misc::toggle_window_mode,    //フルスクリーン切換
                // bevy::window::close_on_esc,  //[ESC]で終了
            )
            .run_if( not( WASM ) ),
        )
    );

    //メイン処理
    app
    .add_state::<MyState>() //Stateを初期化する。enumの#[default]で初期値指定
    .add_plugins( load_assets::Schedule ) //assetsの事前ロード
    .add_plugins( init_app::Schedule )    //ゲーム枠・FPSの表示等、事前処理
    .add_plugins( play_game::Schedule )   //ゲームロジック
    .add_plugins( title_demo::Schedule )  //タイトルデモ
    ;

    //アプリの実行
    app.run();
}

////////////////////////////////////////////////////////////////////////////////

//End of code.