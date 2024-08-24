use super::*;

////////////////////////////////////////////////////////////////////////////////

//プラグインの設定
pub struct Schedule;
impl Plugin for Schedule
{   fn build( &self, app: &mut App )
    {   //メインウィンドウの設定と簡易の表示テスト
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
        .add_plugins( ui_debug_overlay::DebugUiPlugin ) //UI Node Outline Gizmos

        .add_systems
        (   Startup,
            (   //カメラとライトを作る
                (   misc::spawn_camera_2d,
                    misc::spawn_camera_3d,
                    misc::spawn_3d_light,
                ),

                //spawnしたカメラのうちUIの描画に使うカメラのEntity IDをResourceに保存する
                misc::insert_res_ui_render_camera_id,

                //テスト用：オブジェクト表示
                (   debug::spawn_2d_sprites,     //2D表示テスト
                    debug::spawn_3d_objects,     //3D表示テスト
                    debug::spawn_grid_layout_ui, //UI表示テスト ※
                    debug::show_light_gizmo,     //Light Gizmo
                )
                .run_if( DEBUG )
                .run_if( not( state_exists::<MyState> ) )
            )
            .chain() //実行順の固定 ※
            //※UIをどのカメラでレンダリングするか制御する場合は実行順を固定する。
            //　UIのルートノードにカメラのEntity IDをセットする必要があるため。
            //　fn debug::spawn_grid_layout_ui() を参照のこと。
            // https://bevyengine.org/news/bevy-0-13/#camera-driven-ui
            // https://docs.rs/bevy/0.13.0/bevy/ui/struct.TargetCamera.html
        )

        .init_resource::<TargetGamepad>() //操作を受付けるゲームパッドのID
        .add_systems
        (   Update,
            (   //特殊な操作
                (   misc::close_on_esc, //[ESC]で終了
                    misc::toggle_window_mode, //フルスクリーン切換
                )
                .run_if( not( WASM ) ),

                //ゲームパッド検出と切替
                misc::change_gamepad_connection,

                //テスト用：各種の更新処理
                (   //3Dカメラを極座標上で動かす
                    (   (   debug::catch_input_keyboard, //キー
                            debug::catch_input_mouse,    //マウス
                            debug::catch_input_gamepad,  //ゲームパッド
                        ),
                        debug::move_orbit_camera //カメラ移動
                    )
                    .chain(), //実行順の固定

                    //テスト用：Gizmo表示
                    debug::update_gizmo,
                    debug::toggle_ui_node_gizmo, //UI Node Outline Gizmos
                )
                .run_if( DEBUG )
                .run_if( not( state_exists::<MyState> ) )
            )
        )
        ;
    }
}

////////////////////////////////////////////////////////////////////////////////

//End of code.