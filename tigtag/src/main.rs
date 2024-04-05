//external crates
use bevy::
{   prelude::*,
    log::LogPlugin,
    input::mouse::{ MouseMotion, MouseWheel },
    asset::{ LoadState, LoadedUntypedAsset },
    diagnostic::{ FrameTimeDiagnosticsPlugin, DiagnosticsStore },
    utils::Duration,
};
use once_cell::sync::Lazy;
use rand::prelude::*;

//standard library
use std::f32::consts::TAU;

//import names from other crates in this package
use share::*;
use tigtag_inside as play_game;

//internal submodules
mod debug;
mod load_assets;
mod init_app;

////////////////////////////////////////////////////////////////////////////////

//メイン関数
fn main()
{   //アプリの生成
    let mut app = App::new();

    //メインウィンドウの設定と簡易テスト
    let primary_window = MAIN_WINDOW.clone();
    let filter = ( if DEBUG() { LOG_LEVEL_DEV } else { LOG_LEVEL_REL } ).into();
    app
    .insert_resource( Msaa::Sample4 ) //アンチエイリアス
    .add_plugins
    (   DefaultPlugins
        .set( WindowPlugin { primary_window, ..default() } ) //主ウィンドウ
        .set( ImagePlugin::default_nearest() ) //ピクセルパーフェクト
        .set( LogPlugin { filter, ..default() } ) //ロギング
    )
    .add_systems
    (   Startup,
        (   //カメラとライトを作る
            (   misc::spawn_camera_2d,
                misc::spawn_camera_3d,
                misc::spawn_3d_light,
            ),

            //テスト用：オブジェクト表示
            (   debug::spawn_2d_sprites,     //2D表示テスト
                debug::spawn_3d_objects,     //3D表示テスト
                debug::spawn_grid_layout_ui, //UI表示テスト
            )
            .run_if( DEBUG )
            .run_if( not( state_exists::<MyState> ) )
        )
        // .chain() //実行順の固定
        //※UIをどのカメラでレンダリングするか制御する場合は実行順の固定が必要。
        //　UIのルートノードにTargetCameraの設定(EntityのID)が必要になるため。
        //　fn debug::spawn_grid_layout_ui() を参照のこと。
        // https://bevyengine.org/news/bevy-0-13/#camera-driven-ui
        // https://docs.rs/bevy/0.13.0/bevy/ui/struct.TargetCamera.html
    )
    .init_resource::<TargetGamepad>() //操作を受付けるゲームパッドのID
    .add_systems
    (   Update,
        (   (   //ゲームパッド検出と切替
                misc::change_gamepad_connection,

                //特殊な操作
                (   bevy::window::close_on_esc, //[ESC]で終了
                    misc::toggle_window_mode,   //フルスクリーン切換
                )
                .run_if( not( WASM ) ),
            ),

            //テスト用：各種の更新処理
            (   //3Dカメラを極座標上で動かす
                (   (   debug::catch_input_keyboard, //キー
                        debug::catch_input_mouse,    //マウス
                        debug::catch_input_gamepad,  //ゲームパッド
                    ),
                    debug::move_orbit_camera //カメラ移動
                )
                .chain(), //実行順の固定

                //Gizmo描画
                debug::update_gizmo,
            )
            .run_if( DEBUG )
            .run_if( not( state_exists::<MyState> ) )
        )
    )
    ;

    //メイン処理
    app
    .init_state::<MyState>() //Stateを初期化する。enumの#[default]で初期値指定
    .add_plugins( load_assets::Schedule ) //assetsの事前ロード
    .add_plugins( init_app::Schedule    ) //事前処理
    .add_plugins( play_game::Schedule   ) //ゲームロジック
    ;

    //アプリの実行
    app.run();
}

////////////////////////////////////////////////////////////////////////////////

//End of code.