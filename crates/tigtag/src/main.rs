// external crates
use bevy::{
    prelude::*,
    ecs::{
        error::{GLOBAL_ERROR_HANDLER, warn},
        component::Mutable,
    },
    log::LogPlugin,
    window::{EnabledButtons, WindowMode},
    color::palettes::*,
    asset::{LoadedUntypedAsset, LoadState},
    render::camera::Viewport,
    diagnostic::{FrameTimeDiagnosticsPlugin, DiagnosticsStore},
    input::{
        gamepad::{
            GamepadInput, GamepadButton::*, GamepadAxis::*, GamepadAxisChangedEvent,
        },
    },
    // ecs::query::QueryFilter,
};
use rand::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};

// standard library
use std::{
    sync::LazyLock,
    ops::{Range, Deref, DerefMut, Add, AddAssign},
    f32::consts::{PI, TAU},
};

//------------------------------------------------------------------------------

// proc-macro
use macros::MyState;

// internal submodules
mod config; // 設定
use config::*;

mod my_utils; // 共通ライブラリ
use my_utils::*;

mod core_logic; // ゲームアプリの中核
use core_logic::*;

mod popup_messages; //ポップアップメッセージ関連

////////////////////////////////////////////////////////////////////////////////

// メイン関数
fn main() -> AppExit
{
    // エラーハンドラ変更（Note: App生成前に設定すること）
    GLOBAL_ERROR_HANDLER.set(warn).expect(
        "Error handler should be set only once in main() before app-initialization.",
    );

    // アプリを生成
    let mut appl = App::new();

    // Resourceを登録
    appl.init_resource::<Record>()   // ゲームの成績
        .init_resource::<map::Map>() // ステージのマップ
        .init_resource::<player::PlayerInput>() // プレイヤーの入力
        .insert_resource(player::KeyMap(FxHashMap::from_iter(KEY_MAP))) //マッピング（キー）
        .insert_resource(player::PadMap(FxHashMap::from_iter(PAD_MAP))) //マッピング（ゲームパッド）
        ;

    // Eventを登録
    appl.add_event::<EventStageClear>()  //ステージクリアの伝達
        .add_event::<EventGameOver>()    //ゲームオーバーの伝達
    // .add_event::<EventTimerPlayer>()  //プレイヤー移動タイマーのfinishedの伝達
    // .add_event::<EventEatDot>() //スコアリングの伝達
    // .add_event::<EventTimerChasers>() //敵キャラ移動タイマーのfinishedの伝達
        ;

    // アプリを初期化しassetsをロードする
    appl.add_plugins(init_app::Schedule)
        .add_plugins(load_assets::Schedule)
        .insert_resource(load_assets::NextState(MyState::InitGame)); // 完了後の遷移先

    //----------------------------------------------------------------------
    // Updateスケジュール（without State）
    //----------------------------------------------------------------------

    // ゲームパッドの接続状態を検出する
    appl.add_systems(Update, misc::detect_gamepad_connection);

    //ヘッダーの表示を更新する
    appl.insert_resource(header_info::PlaceHolder(PLACE_HOLDER)) // 表示位置
        .add_systems(Update, header_info::update::<header_info::PlaceHolder>); // 表示の更新

    //フッターの表示を更新する
    appl.insert_resource(PlaceHolderFps(header_footer::BottomLeft, 1)) // 表示位置
        .add_systems(Update, update_fps::<PlaceHolderFps>) // 表示の更新
        .add_plugins(FrameTimeDiagnosticsPlugin::default()); // FPS Plugin

    // スプライトアニメーション
    appl.add_systems(
        Update,
        (
            //スプライトアニメーション
            animate_sprites::<player::Player>, // プレイヤー
            animate_sprites::<chaser::Chaser>, // チェイサー
            // スプライト表示OFFの場合
            chaser::rotate_chaser_shape.run_if(SPRITE_OFF), // チェイサー回転
        ),
    );

    // アプリ終了キーをフックして処理を挿入
    // appl.add_systems(
    //     Update,
    //     hook_exit_app_key // Pause等の雛型
    //         .before(misc::app_close_on_key),
    // );

    // UI Nodeのアウトラインの表示／非表示を切替える
    appl.add_systems(Update, misc::toggle_ui_outline_gizmo.run_if(DEBUG));

    //----------------------------------------------------------------------
    // MyState::InitGameスケジュール
    //----------------------------------------------------------------------

    // カメラをspawnする
    appl.insert_resource(simple_camera::Settings(CAMERA_SETTINGS.clone()))
        .add_systems(
            OnEnter(MyState::InitGame),
            (
                simple_camera::spawn::<simple_camera::Settings>,
                misc::select_ui_camera, // UIを描画するカメラを選ぶ
            )
                .chain(), // 実行順の固定
        );

    // 無条件遷移
    appl.add_systems(OnEnter(MyState::InitGame), set_next_state::<StageStart>);
    // appl.add_systems(OnEnter(MyState::InitGame), set_next_state::<TitleDemo>);

    // ヘッダー／フッターの準備
    appl.insert_resource(header_footer::Settings(HEADER_FOOTER)) // UIの情報（Resource）
        .add_systems(
            OnExit(MyState::InitGame),
            header_footer::spawn_header_footer, // UIをspawnする
        );

    // ポップアップメッセージの準備
    appl.init_resource::<PopupMessages>()
        .add_systems(OnExit(MyState::InitGame), popup_messages::spawn::<PopupMessages>);

    //----------------------------------------------------------------------
    // MyState::StageStartスケジュール
    //----------------------------------------------------------------------

    // ステージ初期化
    appl.add_systems(
        OnEnter(MyState::StageStart),
        (
            (
                //マップデータ生成
                map::make_new_stage_data,
                //スプライトのspawn
                (
                    map::spawn_sprite,
                    player::spawn_sprite,
                    chaser::spawn_sprite,
                ),
                // 無条件遷移
                set_next_state::<MainLoop>, //★★★DEBUG後に削除すること★★★
            )
                .chain(), //実行順の固定
            //TextUIの可視化
            (
                // effect::init_count::<stage_start::CountDown>, //カウント初期化
                misc::show_component::<popup::StageSatrt>,
            )
                .chain(), //実行順の固定
        ),
        /*
        // )
        // .add_systems
        // (   Update,
        //     (   //TextUIの演出
        //         effect::count_down::<stage_start::CountDown>, //カウントダウン
        //     )
        //     .run_if( in_state( MyState::StageStart ) )
        // )
        // .add_systems
        // (   OnExit ( MyState::StageStart ),
        //     (   //TextUIの不可視化
        //         misc::hide_component::<stage_start::Message>,
        //     )
         */
    );

    //----------------------------------------------------------------------
    // MyState::MainLoopスケジュール
    //----------------------------------------------------------------------

    // メインループ
    appl.add_systems(
        Update,
        (
            // スコアリング＆クリア判定
            detecting_change::scoring_and_stage_clear,
            // 衝突判定
            detecting_change::collisions_and_gameover
                .run_if(not(on_event::<EventStageClear>)), // ステージクリアならチェックしない
            // Stateの条件付き遷移
            set_next_state::<StageClear>.run_if(on_event::<EventStageClear>),
            set_next_state::<GameOver>.run_if(on_event::<EventGameOver>),
            // スプライトの位置を更新する
            (
                // プレイヤーの移動
                (
                    // 入力に従ってResourceを更新する
                    (
                        player::input_from_keyboard, // キー
                        player::input_from_gamepad,  // ゲームパッド
                    ),
                    player::move_sprite,
                )
                    .chain(), // 実行順の固定
                // チェイサーの移動
                chaser::move_sprite,
            ),
        )
            .chain() // 実行順の固定
            .run_if(in_state(MyState::MainLoop)),
    );

    //----------------------------------------------------------------------
    // MyState::StageClearスケジュール
    //----------------------------------------------------------------------

    // ステージクリアの処理
    appl.add_systems(
        OnEnter(MyState::StageClear),
        // 無条件遷移
        set_next_state::<StageStart>, //★★★DEBUG後に削除すること★★★
    );
    // appl.add_systems
    // (   OnEnter ( MyState::StageClear ),
    //     (   //TextUIの可視化
    //         effect::init_count::<stage_clear::CountDown>, //カウント初期化
    //         misc::show_component::<stage_clear::Message>,
    //     )
    //     .chain(), //実行順の固定
    // )
    // .add_systems
    // (   Update,
    //     (   //TextUIの演出
    //         effect::count_down::<stage_clear::CountDown>, //カウントダウン
    //     )
    //     .run_if( in_state( MyState::StageClear ) )
    // )
    // .add_systems
    // (   OnExit ( MyState::StageClear ),
    //     (   //TextUIの不可視化
    //         misc::hide_component::<stage_clear::Message>,
    //     )
    // );

    //----------------------------------------------------------------------
    // MyState::GameOverスケジュール
    //----------------------------------------------------------------------

    // ゲームオーバーの処理
    appl.add_systems(
        OnEnter(MyState::GameOver),
        // 無条件遷移
        set_next_state::<StageStart>, //★★★DEBUG後に削除すること★★★
    );
    // appl.add_systems
    // (   OnEnter ( MyState::GameOver ),
    //     (   //TextUIの可視化
    //         effect::init_count::<game_over::CountDown>,//カウント初期化
    //         misc::show_component::<game_over::Message>,
    //     )
    //     .chain() //実行順の固定
    // )
    // .add_systems
    // (   Update,
    //     (   //TextUIの演出＆入力待ち
    //         effect::count_down::<game_over::CountDown>, //カウントダウン
    //         effect::blinking_text::<game_over::TextREPLAY>, //Replay? の明滅
    //         effect::hit_any_key::<StageStart>, //Hit ANY Key
    //     )
    //     .run_if( in_state( MyState::GameOver ) )
    // )

    //scoreとstageをゼロクリアする
    appl.add_systems(OnExit(MyState::GameOver), initialize_record_except_hi_score);
    //     (   //TextUIの不可視化
    //         misc::hide_component::<game_over::Message>,

    //----------------------------------------------------------------------
    // MyState::TitleDemoスケジュール
    //----------------------------------------------------------------------

    // タイトル画面
    // appl.add_systems
    // (   OnEnter ( MyState::TitleDemo ),
    //     (   //TextUIの可視化
    //         misc::show_component::<title_demo::Message>,
    //     )
    // )
    // .add_systems
    // (   Update,
    //     (   //TextUIの演出＆入力待ち
    //         effect::blinking_text::<title_demo::TextDEMO>, //Demo の明滅
    //         effect::hit_any_key::<StageStart>, //Hit ANY Key
    //     )
    //     .run_if( in_state( MyState::TitleDemo ) )
    // )
    // .add_systems
    // (   OnExit ( MyState::TitleDemo ),
    //     (   //TextUIの不可視化
    //         misc::hide_component::<title_demo::Message>,

    //         //scoreとstageをゼロクリアする(DEMOでステージクリアの時はしない)
    //         initialize_record_except_hi_score,
    //     )
    // );

    //----------------------------------------------------------------------

    // アプリを実行
    appl.run()
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
