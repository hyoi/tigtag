// external crates
use bevy::{
    prelude::*,
    ecs::error::{GLOBAL_ERROR_HANDLER, warn},
    log::LogPlugin,
    window::{EnabledButtons, WindowMode /* Monitor */},
    color::palettes::*,
    asset::{LoadedUntypedAsset, LoadState},
    render::camera::Viewport,
    // input::{
    //     gamepad::{
    //         GamepadInput, GamepadButton::*, GamepadAxis::*, GamepadAxisChangedEvent,
    //     },
    //     mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    // },
    // diagnostic::{FrameTimeDiagnosticsPlugin, DiagnosticsStore},
    // ecs::query::QueryFilter,
};
use rand::prelude::*;
// use rustc_hash::FxHashMap;
// use chrono::prelude::Local as time_local; //「Local」がbevyとバッティングするのでaliasを使う

// standard library
use std::{
    sync::LazyLock,
    ops::Range,
    ops::{Deref, DerefMut},
    // f32::consts::{PI, TAU},
    // time::Duration,
};

// proc-macro
use macros::MyState;

// internal submodules
mod config; // アプリの設定
use config::*;

mod bevy_app; // アプリ本体

mod template; // 共通
use template::*;

////////////////////////////////////////////////////////////////////////////////

// メイン関数
fn main() -> AppExit
{
    // エラーハンドラ変更（Note: App生成前に設定すること）
    GLOBAL_ERROR_HANDLER.set(warn).expect(
        "Error handler should be set only once in main() before app-initialization.",
    );

    // アプリを実行
    App::new().add_plugins(bevy_app::Schedule).run()
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
