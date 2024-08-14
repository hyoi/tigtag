//external crates
use bevy::
{   prelude::*,
    input::keyboard::NativeKeyCode,
    sprite::MaterialMesh2dBundle,
    utils::{ HashMap, HashSet },
    audio::Volume,
};
use rand::prelude::*;

//standard library
use std::
{   sync::LazyLock,
    ops::Range,
    f32::consts::{ PI, TAU },
    ops::{ Add, AddAssign },
    cmp::Ordering,
    collections::VecDeque,
};

//other crates in this package
use template::*;

//ゲームロジック
mod play_game;

// mod schedule;
// pub use schedule::Schedule;

// //play_game内共通
// mod common;
// use common::*;

// //UIの処理
// mod header;
// mod ui;
// use ui::*;

// //pause処理
// mod pause;

// //マップ、自機、追手の処理
// mod map;
// use map::GridToPixelOnMap;

// mod player;
// mod chasers;
// mod detection;

// //デモ
// mod demo;

////////////////////////////////////////////////////////////////////////////////

//メイン関数
fn main()
{   //アプリの生成
    let mut app = App::new();

    //メイン処理
    app
    .add_plugins( template::Schedule  ) //アプリの雛型
    .add_plugins( play_game::Schedule ) //ゲームロジック
    ;

    //アプリの実行
    app.run();
}

////////////////////////////////////////////////////////////////////////////////

//End of code.