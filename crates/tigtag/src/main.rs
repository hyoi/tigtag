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

//internal submodules
mod template;
use template::*;

//ゲームロジック
mod tigtag_inside;

////////////////////////////////////////////////////////////////////////////////

//コンパイル オプションの定数
pub const SPRITE_OFF: fn() -> bool = || cfg!( feature = "sprite_off" );

////////////////////////////////////////////////////////////////////////////////

//メイン関数
fn main()
{   //アプリの生成
    let mut app = App::new();

    //メイン処理
    app
    .add_plugins( template::Schedule      ) //アプリの雛型
    .add_plugins( tigtag_inside::Schedule ) //ゲームロジック
    ;

    //アプリの実行
    app.run();
}

////////////////////////////////////////////////////////////////////////////////

//End of code.