// external crates
use bevy::{
    prelude::*,
    ecs::{
        error::{/*GLOBAL_ERROR_HANDLER,*/ warn},
    //     system::SystemParam,
    //     component::Mutable,
    },
    // window::{EnabledButtons, WindowMode},
    // log::LogPlugin,
    // diagnostic::{FrameTimeDiagnosticsPlugin, DiagnosticsStore},
    // color::palettes::*,
    // asset::{LoadedUntypedAsset, LoadState},
    // render::camera::Viewport,
    // input::{
    //     keyboard::NativeKeyCode,
    //     gamepad::{GamepadInput, GamepadAxisChangedEvent},
    // },
    // audio::Volume,
};
// use rand::prelude::*;
// use rustc_hash::{FxHashMap, FxHashSet};

// standard library
// use std::{
//     slice::Iter,
//     ops::{Range, Deref, DerefMut, Add, AddAssign},
//     f32::consts::{TAU, PI},
//     collections::VecDeque,
// };

// proc-macro
// use macros::MyState;
// use macros::{OverlayMessage, Blinking, CountDown};
// use macros::{OverlayMenu, ScalingItem};
// use macros::derive_appctrl_input;

// internal submodules
// mod my_utils; // 共通ライブラリ
// use my_utils::*;

// mod config; // 設定各種
// use config::*;

// mod core_logic; // ゲームロジック
// use core_logic::*;
// use core_logic::overlay_ui::OverlayMessage;

// mod demo_play; // demoロジック

////////////////////////////////////////////////////////////////////////////////

// メイン関数
fn main() -> AppExit
{
    // エラーハンドラ設定
    // GLOBAL_ERROR_HANDLER.set(warn).expect(
    //     "Configure the error handler in main() once, prior to app initialization.",
    // );

    // アプリの生成
    App::new()
        // エラーハンドラ設定
        .set_error_handler(warn)
        // メインスケジュール
        // .add_plugins(core_logic::MainSchedule)
        // アプリ実行
        .run()
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
