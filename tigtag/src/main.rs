//external crates
use bevy::{ prelude::*, sprite::*, audio::*,
            input::gamepad::*, diagnostic::*, window::WindowMode::* };
use once_cell::sync::*;
use rand::prelude::*;
use counted_array::*;

//standard library
use std::ops::*;
use std::cmp::*;
use std::collections::*;

//internal submodules
mod a_public;
mod b_init_app;
mod c_game_play;
mod d_demo_play;

use a_public::*;
use b_init_app::*;
use c_game_play::*;
use d_demo_play::*;

//メイン関数
fn main()
{   //ログのコンソールへの出力を抑止
    #[cfg( not( target_arch = "wasm32" ) )]
    std::env::set_var( "RUST_LOG", "OFF" );

    //メインウィンドウの設定
    let primary_window = MAIN_WINDOW.clone();
    let mut app = App::new();
    app
    .insert_resource( ClearColor( SCREEN_BACKGROUND_COLOR ) )
    .insert_resource( Msaa::Sample4 ) //アンチエイリアス
    .add_plugins
    (   DefaultPlugins
        .set( WindowPlugin { primary_window, ..default() } ) //メインウィンドウ設定
        .set( ImagePlugin::default_nearest() ) //ピクセルパーフェクト設定
    );

    //特別なSystem
    app
    .add_systems( Startup, misc::spawn_2d_camera ) //bevyのカメラ
    // .add_systems( Update, bevy::window::close_on_esc ) //[ESC]で終了
    .add_systems( Update, misc::pause_with_esc_key ) //[Esc]でPause
    .add_systems( Update, misc::toggle_window_mode.run_if( not( misc::WASM ) ) ) //フルスクリーン切換
    ;

    //ResourceとEvent
    app
    .add_state::<MyState>()       //Stateの初期化
    .init_resource::<Record>()    //スコア等の初期化
    .init_resource::<CountDown>() //カウントダウンタイマーの初期化
    .init_resource::<Map>()       //迷路情報の初期化
    .add_event::<EventClear>()    //ステージクリアイベント
    .add_event::<EventOver>()     //ゲームオーバーイベント
    ;

    //アプリの実行
    app
    .add_plugins( InitApp  )
    .add_plugins( GamePlay )
    .add_plugins( DemoPlay )
    .run()
    ;
}

//End of code.