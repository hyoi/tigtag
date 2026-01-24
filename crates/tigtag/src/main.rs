// external crates
use bevy::{
    prelude::*,
    window::{WindowMode, EnabledButtons},
    render::{
        RenderPlugin,
        settings::{Backends, RenderCreation, WgpuSettings},
    },
    log::{LogPlugin, Level},
    diagnostic::{FrameTimeDiagnosticsPlugin /*, DiagnosticsStore*/},
    ecs::{error::warn /*, system::SystemParam, component::Mutable*/},
    audio::Volume,
    // color::palettes::*,
    // asset::{LoadedUntypedAsset, LoadState},
    // camera::Viewport,
    // input::{
    //     keyboard::NativeKeyCode,
    //     gamepad::GamepadInput,
    //     mouse::{MouseMotion, MouseWheel},
    // },
    // platform::collections::{HashMap, HashSet},
};
use const_format::formatcp;
// use rand::prelude::*;

// standard library
// use std::{
//     // slice::Iter,
//     // ops::{Range, Deref, DerefMut, Add, AddAssign},
//     // f32::consts::{TAU, PI},
//     // collections::VecDeque,
//     // sync::LazyLock,
// };

// internal submodules
mod config; // 設定ファイル
use config::common::*;

mod my_plugins; // 自作のプラグイン
use my_plugins::common::*;

mod core_logic; // アプリ本体
use core_logic::common::*;

// mod demo_play; // demoロジック

// mod my_utils; // 共通ライブラリ
// use my_utils::common::*;

// proc-macro
use macros::MyState;

// use macros::derive_appctrl_input;
// use macros::{OverlayMessage, Blinking, CountDown};
// use macros::{OverlayMenu, ScalingItem};

////////////////////////////////////////////////////////////////////////////////

// アプリの情報
const APP_TITLE: &str = "TigTag"; // env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_COPYRIGHT: &str = "hyoi 2021-2026";

////////////////////////////////////////////////////////////////////////////////

// メイン関数
fn main() -> AppExit
{
    // アプリの生成
    let mut application = App::new();

    // 準備
    application
        // first party plugins
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin::initialize()) // 主ウィンドウ初期化
                .set(RenderPlugin::initialize(Backends::DX12)) // バックエンド切替
                .set(ImagePlugin::default_nearest()) // ピクセルパーフェクト
                .set(LogPlugin::initialize()), // ログ出力制御
            FrameTimeDiagnosticsPlugin::default(), // FPS Plugin
        ))
        // エラーハンドラを登録
        .set_error_handler(warn);

    // 自作のプラグインの登録
    application.add_plugins((
        // ゲームパッド接続状態の管理
        controller::GamepadConnectionCheck,
        // フルスクリーン処理
        #[cfg(not(target_arch = "wasm32"))]
        fullscreen::PluginConfig {
            base_resolution: WINDOW_BASE_RESOLUTION.as_vec2(),
            toggle_trigger: TRIGGER_FULLSCREEN,
        },
        // 各種雑多な処理
        #[allow(clippy::needless_update)]
        misc::PluginConfig {
            app_exit_trigger: Some(TRIGGER_APP_EXIT),
            ui_outline_trigger: Some(TRIGGER_UI_OUTLINE),
            ..default()
        },
    ));

    // メインスケジュール
    application.add_plugins(core_logic::Schedule);

    // アプリの実行
    application.run()
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
