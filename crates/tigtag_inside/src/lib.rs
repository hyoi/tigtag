//external crates
use bevy::
{   prelude::*,
    input::keyboard::NativeKeyCode,
    sprite::MaterialMesh2dBundle,
    utils::{ HashMap, HashSet },
    audio::Volume,
};
use once_cell::sync::Lazy;
use rand::prelude::*;

//standard library
use std::
{   ops::Range,
    f32::consts::{ PI, TAU },
    ops::{ Add, AddAssign },
    cmp::Ordering,
    collections::VecDeque,
};

//import names from other crates in this package
use share::*;

//ゲームロジック
mod schedule;
pub use schedule::Schedule;

//play_game内共通
mod common;
use common::*;

//UIの処理
mod header;
mod ui;
use ui::*;

//pause処理
mod pause;

//マップ、自機、追手の処理
mod map;
use map::GridToPixelOnMap;
mod player;
mod chasers;
mod detection;

//デモ
mod demo;

//End of code.