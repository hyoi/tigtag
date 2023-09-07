//external crates
use bevy::
{   prelude::*, sprite::*, audio::*,
    window::WindowMode::*, diagnostic::*
};
use once_cell::sync::*;
use rand::prelude::*;
use counted_array::*;

//standard library
use std::{ ops::*, cmp::*, collections::* };

//internal submodules
mod public;
use public::*;

mod init_game;
mod game_play;
mod title_demo;

////////////////////////////////////////////////////////////////////////////////

//メイン関数
fn main()
{   //ログのコンソールへの出力を抑制
    #[cfg( not( target_arch = "wasm32" ) )]
    std::env::set_var( "RUST_LOG", "OFF" );

    //アプリの生成
    let mut app = App::new();

    //メインウィンドウの設定等
    let primary_window = MAIN_WINDOW.clone();
    app
    .insert_resource( ClearColor( SCREEN_BACKGROUND_COLOR ) )
    .insert_resource( Msaa::Sample4 ) //アンチエイリアス
    .add_plugins
    (   DefaultPlugins
        .set( WindowPlugin { primary_window, ..default() } ) //メインウィンドウ設定
        .set( ImagePlugin::default_nearest() ) //ピクセルパーフェクト設定
    )
    .add_systems( Startup, misc::spawn_2d_camera ) //bevyのカメラ
    // .add_systems( Update, bevy::window::close_on_esc ) //[ESC]で終了
    .add_systems( Update, misc::toggle_window_mode.run_if( not( misc::WASM ) ) ) //フルスクリーン切換
    .add_systems( Update, misc::catch_gamepad_connection_changes ) //gamepadの特定
    ;

    //アプリの実行
    app
    .add_plugins( init_game::Schedule )  //ゲームの準備
    .add_plugins( game_play::Schedule )  //ゲームのメイン処理
    .add_plugins( title_demo::Schedule ) //タイトルデモ
    .run()
    ;
}

////////////////////////////////////////////////////////////////////////////////

//End of code.