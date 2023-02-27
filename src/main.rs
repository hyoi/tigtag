//import external modules
use bevy::{ prelude::*, sprite::* };
//# use bevy_kira_audio::{ Audio, AudioPlugin, AudioControl };
use once_cell::sync::*;
use rand::prelude::*;

//internal submodules
mod public;
mod init_app;
mod game_play;
mod demo_play;

use public::*;
use init_app::*;
use game_play::*;
use demo_play::*;

fn main()
{   //bevy_kira_audioが標準出力へInfoを出力するのを抑止
    #[cfg( not( target_arch = "wasm32" ) )]
    std::env::set_var( "RUST_LOG", "OFF" );

    //アプリの実行
    App::new()
    .add_plugin( InitApp  )
    .add_plugin( GamePlay )
    .add_plugin( DemoPlay )
    .run()
    ;
}

//End of code.