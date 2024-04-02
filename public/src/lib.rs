//external crates
use bevy::
{   prelude::*,
    window::WindowMode,
    ecs::query::QueryFilter,
};
use once_cell::sync::Lazy;

//standard library
use std::
{   ops::Range,
    f32::consts::{ PI, TAU },
};

//proc-macro crates
use macros::MyState;

//アプリの設定
mod config;
pub use config::*;

//型定義
mod types;
pub use types::*;

//ユーティリティ
pub mod misc;
pub use misc::constants::*; //定数は名前を公開

//End of code.