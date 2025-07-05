// external crates
use bevy::{
    prelude::*,
    ecs::{
        error::{GLOBAL_ERROR_HANDLER, warn},
        component::Mutable,
    },
    log::LogPlugin,
    window::{EnabledButtons, WindowMode /* Monitor */},
    color::palettes::*,
    asset::{LoadedUntypedAsset, LoadState},
    render::camera::Viewport,
    diagnostic::{FrameTimeDiagnosticsPlugin, DiagnosticsStore},
    input::{
        gamepad::{
            GamepadInput, GamepadButton::*, GamepadAxis::*, GamepadAxisChangedEvent,
        },
        //     mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    },
    // ecs::query::QueryFilter,
};
use rand::prelude::*;
use rustc_hash::FxHashMap;
// use chrono::prelude::Local as time_local; //「Local」がbevyとバッティングするのでaliasを使う

// standard library
use std::{
    sync::LazyLock,
    ops::{Range, Deref, DerefMut, Add, AddAssign},
    f32::consts::{PI, TAU},
    cmp::Ordering,
    // time::Duration,
};

// proc-macro
use macros::MyState;

// internal submodules
mod config; // 設定
use config::*;

mod bevy_app; // アプリ本体
use bevy_app::*;

mod template; // 共通ライブラリ
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
