// external crates
use bevy::{
    prelude::*,
    ecs::{
        error::warn,
        system::SystemParam,
        // component::Mutable,
    },
    window::{EnabledButtons, WindowMode},
    log::LogPlugin,
    diagnostic::{FrameTimeDiagnosticsPlugin, DiagnosticsStore},
    color::palettes::*,
    asset::{LoadedUntypedAsset, LoadState},
    camera::Viewport,
    // input::{
    //     keyboard::NativeKeyCode,
    //     gamepad::{GamepadInput, GamepadAxisChangedEvent},
    // },
    // audio::Volume,
};
use rand::prelude::*;
// use rustc_hash::{FxHashMap, FxHashSet};

// standard library
use std::{
    slice::Iter,
    ops::{Range, Deref, DerefMut, /*Add, AddAssign*/},
    // f32::consts::{TAU, PI},
    // collections::VecDeque,
};

// internal submodules
mod core_logic; // ゲームロジック
// use core_logic::*;
// use core_logic::overlay_ui::OverlayMessage;

mod my_utils; // 共通ライブラリ
use my_utils::*;

mod config; // 設定各種
use config::*;

// mod demo_play; // demoロジック

// proc-macro
use macros::MyState;
use macros::derive_appctrl_input;
// use macros::{OverlayMessage, Blinking, CountDown};
// use macros::{OverlayMenu, ScalingItem};

////////////////////////////////////////////////////////////////////////////////

// メイン関数
fn main() -> AppExit
{
    // アプリの生成
    App::new()
        // メインスケジュール
        .add_plugins(core_logic::schedule::Schedule)
        // アプリ実行
        .run()
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
