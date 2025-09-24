// external crates
use bevy::{
    prelude::*,
    ecs::{
        error::{GLOBAL_ERROR_HANDLER, warn},
        system::SystemParam,
        component::Mutable,
    },
    window::{EnabledButtons, WindowMode},
    log::LogPlugin,
    diagnostic::{FrameTimeDiagnosticsPlugin, DiagnosticsStore},
    color::palettes::*,
    asset::{LoadedUntypedAsset, LoadState},
    render::camera::Viewport,
    input::{
        keyboard::NativeKeyCode,
        gamepad::{GamepadInput, GamepadAxisChangedEvent},
    },
    audio::Volume,
};
use rand::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};

// standard library
use std::{
    slice::Iter,
    ops::{Range, Deref, DerefMut, Add, AddAssign},
    f32::consts::{TAU, PI},
    collections::VecDeque,
    //     sync::LazyLock,
};

// my proc-macro
use macros::MyState;
use macros::{OverlayMessage, Blinking, CountDown};
use macros::{OverlayMenu, ScalingItem};
use macros::derive_appctrl_input;

// my internal submodules
mod my_utils; // 共通ライブラリ
use my_utils::*;

mod config; // 各種設定
use config::*;

mod core_logic; // ゲームアプリの中核
use core_logic::*;
use core_logic::overlay_ui::OverlayMessage;

mod demo_play; // DEMO

////////////////////////////////////////////////////////////////////////////////

// メイン関数
fn main() -> AppExit
{
    // エラーハンドラ設定
    GLOBAL_ERROR_HANDLER.set(warn).expect(
        "Configure the error handler in main() once, prior to app initialization.",
    );

    // アプリを生成
    let mut application = App::new();

    // 汎用的なアプリ初期化とアセットのロード
    application
        .add_plugins(init_app::Schedule)
        .insert_resource(init_app::ChangeTo(MyState::Initialize)) // 処理が完了したらState変更
        ;

    //--------------------------------------------------------------------------
    // 各種登録
    application
        // Eventの登録
        .add_event::<misc::AnyButtonPressed>() //「Hit Any Key」の入力通知
        .add_event::<CountDownFinished>()      // カウントダウンの終了通知
        .add_event::<EventPlayerInputNews>()   // プレイヤーキャラクターの操作入力通知
        .add_event::<DotEaten>()               // スコアリングの伝達用
        .add_event::<DotsAllEaten >()          // ステージクリアの伝達用
        .add_event::<PlayerCaught>()           // ゲームオーバーの伝達用
        .add_event::<SkipOverlayMessage>()     // 全画面メッセージ表示のスキップに使用

        // Resourceの登録
        .init_resource::<CameraSettings>()                  // カメラの設定を登録
        .init_resource::<Record>()                          // ゲームの成績
        .init_resource::<map::Map>()                        // ステージのマップ
        .init_resource::<misc::MaskHitAnyKeyInput>()        // 「Hit Any Key」の入力マスク
        .insert_resource(player::KeyMap::from( KEY_MAP ))   // マッピング（キー）
        .insert_resource(player::GamepadMap::from(PAD_MAP)) // マッピング（ゲームパッド）
        ;

    //--------------------------------------------------------------------------
    // 常に実行する処理（Update without MyState）
    application
        .add_systems(
            Update, // without MyState
            (
                //スプライトアニメーション
                animate_sprites::<player::Player>, // プレイヤー
                animate_sprites::<chaser::Chaser>, // チェイサー
                // スプライト表示OFFの場合のアニメーション
                chaser::rotate_chaser_shape.run_if(SPRITE_OFF), // チェイサー回転
                //ヘッダーとフッターの表示情報を更新する
                information::update_header_footer,
            ),
        )
        // システムセット間の実行順序を管理する
        .configure_sets(
            Update,
            (
                // BeforeHitAnyKey は HitAnyKey の前に実行
                MyLabel::BeforeHitAnyKey.before(MyLabel::HitAnyKey),
                // AfterHitAnyKey は HitAnyKey の後に実行
                MyLabel::AfterHitAnyKey.after(MyLabel::HitAnyKey),
            ),
        )
        // Pauseメニューのスケジュールを追加
        .add_plugins(overlay_ui::pause_menu::Schedule);

    //--------------------------------------------------------------------------
    // 初期化（MyState::InitGame）
    application
        // 前処理
        .add_systems(
            OnEnter(MyState::Initialize),
            (
                simple_camera::spawn::<CameraSettings> // カメラのspawn
                    .before(misc::select_ui_camera),
                misc::select_ui_camera, // UIを描画するカメラの選択
            ),
        )
        // ループ処理
        .add_systems(
            Update, // within MyState::Initialize
            (
                set_next_state::<TitleDemo>
                // 無条件遷移
            )
                .run_if(in_state(MyState::Initialize)),
        )
        // 後処理
        .add_systems(
            OnExit(MyState::Initialize),
            (
                header_footer::spawn,       // ヘッダー／フッターのspawn
                overlay_ui::spawn_messages, //全画面メッセージのspawn
            ),
        );

    //--------------------------------------------------------------------------
    // タイトル画面の処理（MyState::TitleDemo）
    application
        // 前処理
        .add_systems(
            OnEnter(MyState::TitleDemo),
            (
                // 全画面メッセージ（タイトル）表示
                OverlayTitleDemo::init(),
                misc::show_component::<OverlayTitleDemo>
                    .after(OverlayTitleDemo::init()),
            ),
        )
        // ループ処理
        .add_systems(
            Update, // within MyState::TitleDemo
            (
                // Hit ANY Key に反応あればState遷移
                misc::check_hit_any_key.in_set(MyLabel::HitAnyKey),
                (
                    // scoreとstageをゼロクリアする(demoの情報消去)
                    detecting_change::initialize_score_stage,
                    set_next_state::<StageStart>,
                )
                    .in_set(MyLabel::AfterHitAnyKey)
                    .run_if(on_event::<misc::AnyButtonPressed>),
                // DEMO の明滅
                overlay_ui::effect::blinking_text::<OverlayTitleDemo>,
            )
                .run_if(in_state(MyState::TitleDemo)),
        )
        // 後処理
        .add_systems(
            OnExit(MyState::TitleDemo),
            (
                // 全画面メッセージ（タイトル）非表示
                misc::hide_component::<OverlayTitleDemo>,
            ),
        )
        // デモプレイplugin
        .add_plugins(demo_play::Schedule);

    //--------------------------------------------------------------------------
    // ゲーム開始処理（MyState::StageStart）
    application
        // 前処理
        .add_systems(
            OnEnter(MyState::StageStart),
            (
                //ステージ初期化
                map::make_new_stage_data, // マップデータ
                (
                    map::spawn_sprite,    // マップスプライト
                    player::spawn_sprite, // プレーヤースプライト
                    chaser::spawn_sprite, // チェイサースプライト
                )
                    .after(map::make_new_stage_data),
                // 全画面メッセージの表示スキップ指示があるなら即メインループへ
                set_next_state::<MainLoop>.run_if(on_event::<SkipOverlayMessage>),
                // 全画面メッセージ（ステージ開始）表示
                (
                    OverlayStageStart::init(),
                    misc::show_component::<OverlayStageStart>
                        .after(OverlayStageStart::init()),
                )
                    .run_if(not(on_event::<SkipOverlayMessage>)),
            ),
        )
        // ループ処理
        .add_systems(
            Update, // within MyState::StageStart
            (
                // カウントダウン完了後にState遷移
                overlay_ui::effect::countdown::<OverlayStageStart>,
                set_next_state::<MainLoop>
                    .run_if(on_event::<CountDownFinished>)
                    .after(overlay_ui::effect::countdown::<OverlayStageStart>),
            )
                .run_if(in_state(MyState::StageStart)),
        )
        // 後処理
        .add_systems(
            OnExit(MyState::StageStart),
            (
                // 全画面メッセージ（ステージ開始）非表示
                misc::hide_component::<OverlayStageStart>,
            ),
        );

    //--------------------------------------------------------------------------
    // メインループ処理（MyState::MainLoop）
    application
        // ループ処理
        .add_systems(
            Update, // within MyState::MainLoop
            (
                (
                    // スプライトの位置を更新する
                    player::input_from_keyboard, // キー
                    player::input_from_gamepad,  // ゲームパッド
                    player::move_sprite
                        .after(player::input_from_keyboard)
                        .after(player::input_from_gamepad),
                    chaser::move_sprite,
                ),
                // スコアリング＆クリア判定
                detecting_change::scoring_and_stage_clear,
                set_next_state::<StageClear>.run_if(on_event::<DotsAllEaten>),
                // 衝突判定
                detecting_change::collisions_and_gameover
                    .run_if(not(on_event::<DotsAllEaten>)), // DotsAllEaten ➡ スキップ
                set_next_state::<GameOver>.run_if(on_event::<PlayerCaught>),
            )
                .chain()
                .run_if(in_state(MyState::MainLoop)),
        );

    //--------------------------------------------------------------------------
    // ステージクリアの処理（MyState::StageClear）
    application
        // 前処理
        .add_systems(
            OnEnter(MyState::StageClear),
            (
                // 全画面メッセージ（ステージクリア）表示
                OverlayStageClear::init(),
                misc::show_component::<OverlayStageClear>
                    .after(OverlayStageClear::init()),
            ),
        )
        // ループ処理
        .add_systems(
            Update, // within MyState::StageClear
            (
                // カウントダウン完了後にState遷移
                overlay_ui::effect::countdown::<OverlayStageClear>,
                set_next_state::<StageStart>
                    .run_if(on_event::<CountDownFinished>)
                    .after(overlay_ui::effect::countdown::<OverlayStageClear>),
            )
                .run_if(in_state(MyState::StageClear)),
        )
        // 後処理
        .add_systems(
            OnExit(MyState::StageClear),
            (
                // 全画面メッセージ（ステージクリア）非表示
                misc::hide_component::<OverlayStageClear>,
                // 後続の MyState::StageStart で全画面メッセージを表示しない
                misc::set_event::<SkipOverlayMessage>,
            ),
        );

    //--------------------------------------------------------------------------
    // ゲームオーバーの処理（MyState::GameOver）
    application
        // 前処理
        .add_systems(
            OnEnter(MyState::GameOver),
            (
                // 全画面メッセージ（ステージクリア）表示
                OverlayGameOver::init(),
                misc::show_component::<OverlayGameOver>
                    .after(OverlayGameOver::init()),
            ),
        )
        .add_systems(
            // ループ処理
            Update, // within MyState::GameOver
            (
                // カウントダウン完了後にState遷移
                overlay_ui::effect::countdown::<OverlayGameOver>,
                set_next_state::<TitleDemo>
                    .run_if(on_event::<CountDownFinished>)
                    .after(overlay_ui::effect::countdown::<OverlayGameOver>),
                // Replay? の明滅
                overlay_ui::effect::blinking_text::<OverlayGameOver>,
                // Hit ANY Key に反応あればState遷移
                misc::check_hit_any_key.in_set(MyLabel::HitAnyKey),
                set_next_state::<StageStart>
                    .in_set(MyLabel::AfterHitAnyKey)
                    .run_if(on_event::<misc::AnyButtonPressed>),
            )
                .run_if(in_state(MyState::GameOver)),
        )
        // 後処理
        .add_systems(
            OnExit(MyState::GameOver),
            (
                // 全画面メッセージ（ステージクリア）非表示
                misc::hide_component::<OverlayGameOver>,
                // scoreとstageをゼロクリアする
                detecting_change::initialize_score_stage,
            ),
        );

    // アプリを実行
    application.run()
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
