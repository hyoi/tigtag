//external crates
use bevy::{ prelude::*, sprite::*,  audio::*, diagnostic::*, window::WindowMode::* };
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

    //アプリの実行
    App::new()
    .add_plugins( InitApp  )
    .add_plugins( GamePlay )
    .add_plugins( DemoPlay )
    .run()
    ;
}

//End of code.