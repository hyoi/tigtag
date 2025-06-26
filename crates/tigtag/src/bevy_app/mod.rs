use super::*;

// internal submodules
// mod test_2d; // 2Dのテスト
// mod test_3d; // 3Dのテスト
// mod test_ui; // UIのテスト

////////////////////////////////////////////////////////////////////////////////

// プラグインの設定
pub struct Schedule;
impl Plugin for Schedule
{
    fn build(&self, appl: &mut App)
    {
        // アプリの初期化
        appl.add_plugins(init_app::Schedule);

        // assetsの事前ロード
        appl.add_plugins(load_assets::Schedule)
            .insert_resource(load_assets::NextState(MyState::InitGame)); // 完了後のState遷移先

        // アプリ終了キーをフックして処理を挿入
        // appl.add_systems(
        //     Update,
        //     hook_exit_app_key // Pause等の雛型
        //         .before(misc::app_close_on_key),
        // );

        // カメラ（Camera2dとCamera3d）をspawnする
        appl.insert_resource(simple_camera::Settings(CAMERA_SETTINGS.clone()))
            .add_systems(
                OnEnter(MyState::InitGame),
                simple_camera::spawn::<simple_camera::Settings>,
            );

        // 2D表示簡易テスト
        // appl.add_systems(
        //     OnEnter(MyState::InitGame),
        //     test_2d::spawn_sprites, // 格子状にスプライトをspawnする
        // )
        // .add_systems(
        //     Update,
        //     test_2d::draw_gizmos_2d // gizmoを描画
        //         .run_if(in_state(MyState::InitGame)),
        // );

        // 3D表示簡易テスト
        // appl.add_systems(
        //     OnEnter(MyState::InitGame),
        //     (
        //         test_3d::spawn_simple_light3d, // 3Dライトをspawnする
        //         test_3d::spawn_meshes3d,       // 3Dメッシュをspawnする
        //         test_3d::show_light_gizmo,     // ライトのgizmoを描画
        //     ),
        // )
        // .add_systems(
        //     Update,
        //     test_3d::draw_gizmos_3d // gizmoを描画
        //         .run_if(in_state(MyState::InitGame)),
        // );

        // UI簡易テスト
        // appl.add_systems(
        //     OnEnter(MyState::InitGame),
        //     (
        //         test_ui::select_ui_camera,   // UIを描画するカメラを選ぶ
        //         test_ui::spawn_grid_text_ui, // UIをspawnする
        //     )
        //         .chain() // 実行順固定
        //         .after(simple_camera::spawn::<simple_camera::Settings>) // カメラのspawn
        //         .run_if(any_with_component::<Camera>), // カメラが存在すれば実行する
        // )
        // .add_systems(
        //     Update,
        //     test_ui::toggle_ui_outline_gizmo // TextUIのアウトライン表示
        //         .run_if(in_state(MyState::InitGame)),
        // );

        // 入力（キー、マウス、ゲームパッド）の簡易テスト（極座標カメラの操作）
        // appl.insert_resource(ORBIT_CAMERA_DEFAULT.clone())
        //     .insert_resource(orbit_camera::KeyMap(FxHashMap::from_iter(KEY_MAP)))
        //     .insert_resource(orbit_camera::PadMap(FxHashMap::from_iter(PAD_MAP)))
        //     .add_systems(
        //         Update,
        //         (
        //             // ゲームパッドの接続状態を検出する
        //             misc::detect_gamepad_connection,
        //             (
        //                 // Resourceに保存した極座標値を更新する
        //                 orbit_camera::input_from_keyboard, // キー
        //                 orbit_camera::input_from_gamepad,  // ゲームパッド
        //                 orbit_camera::input_from_mouse,    // マウス
        //             ),
        //             // カメラの座標を変更する
        //             orbit_camera::move_orbit_camera::<SimpleCamera3dOrbit>,
        //         )
        //             .chain() // 実行順固定
        //             .run_if(in_state(MyState::InitGame)),
        //     );

        // ヘッダー／フッターの準備
        appl.insert_resource(header_footer::Settings(HEADER_FOOTER)) // UIの情報（Resource）
            .add_systems(
                OnEnter(MyState::InitGame),
                (
                    misc::select_ui_camera // UIを描画するカメラを選ぶ
                        .after( simple_camera::spawn::<simple_camera::Settings> ),
                    header_footer::spawn_header_footer, // UIをspawnする
                )
                    .chain(),
            );

        // FPS表示
        let bottomleft = header_footer::Position::BottomLeft;
        let index = 1;
        appl.add_plugins(FrameTimeDiagnosticsPlugin::default()) // FPS Plugin
            .insert_resource(DisplayInfoFps(bottomleft, index)) // 表示位置の指定（Resource）
            .add_systems(Update, update_fps::<DisplayInfoFps>); // FPS表示の更新
    }
}

////////////////////////////////////////////////////////////////////////////////

// Pause等の処理の雛形
// fn hook_exit_app_key(
//     mut input_keycode: ResMut<ButtonInput<KeyCode>>,
//     mut state: ResMut<State<MyState>>,
//     mut back_to: Local<MyState>,
// )
// {
//     // キーが押下されているか
//     if input_keycode.just_pressed(EXIT_APP_KEY)
//     {
//         // キー押下をリセットする（misc::app_close_on_keyが実行されないように）
//         input_keycode.reset(EXIT_APP_KEY);

//         // Pauseのトグル処理
//         if state.get().is_pause()
//         {
//             // OnEnter／OnExitを実行せす遷移する
//             *state = State::new(*back_to);
//         }
//         else
//         {
//             // 遷移元のStateをローカルに保存する
//             *back_to = *state.get();

//             // OnEnter／OnExitを実行せす遷移する
//             *state = State::new(MyState::Pause);
//         }

//         #[cfg(debug_assertions)]
//         dbg!(state);
//     }
// }

////////////////////////////////////////////////////////////////////////////////

// 更新対象の位置を指定するためのResource
#[derive(Resource)]
struct DisplayInfoFps(header_footer::Position, usize);

trait DisplayInfoFpsTrait
{
    fn position(&self) -> header_footer::Position;
    fn index(&self) -> usize;
}
impl DisplayInfoFpsTrait for DisplayInfoFps
{
    fn position(&self) -> header_footer::Position { self.0 }
    fn index(&self) -> usize { self.1 }
}

// FPSの表示を更新する
fn update_fps<T: Resource + DisplayInfoFpsTrait>(
    opt_display_info: Option<Res<T>>,
    qry_text_block: Query<(Entity, &header_footer::Position)>,
    mut text_writer: TextUiWriter,
    diag_store: Res<DiagnosticsStore>,
) -> Result
{
    // 準備
    let display_info = opt_display_info.ok_or("DisplayInfoFps not found.")?;
    let root_entity = qry_text_block
        .iter()
        .filter(|(_, position)| **position == display_info.position())
        .collect::<Vec<(Entity, &header_footer::Position)>>();

    // 書き換え
    if !root_entity.is_empty()
    {
        let entity = root_entity[0].0;
        let mut fps_text = text_writer
            .get_text(entity, display_info.index())
            .ok_or(format!(
                "no entity with a matching index: {}",
                display_info.index()
            ))?;

        *fps_text = diag_store.get(&FrameTimeDiagnosticsPlugin::FPS).map_or(
            NA3_2.to_string(),
            |fps| {
                fps.average()
                    .map_or(NA3_2.to_string(), |avg| format!("{avg:06.2}"))
            },
        );
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
