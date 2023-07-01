//import external modules
use macros::*;
use bevy::{ prelude::*, sprite::* };
use once_cell::sync::*;
use rand::prelude::*;
use counted_array::*;

//internal submodules
mod a_public;
// mod b_init_app;
// mod c_game_play;
// mod d_demo_play;

use a_public::*;
// use b_init_app::*;
// use c_game_play::*;
// use d_demo_play::*;

fn main()
{   //コンソールへログを出力するのを抑止
    #[cfg( not( target_arch = "wasm32" ) )]
    std::env::set_var( "RUST_LOG", "OFF" );

    //アプリの実行
    App::new()
    // .add_plugin( InitApp  )
    // .add_plugin( GamePlay )
    // .add_plugin( DemoPlay )
    .run()
    ;
}

//End of code.