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

        // 端末がサポートしている解像度を一覧表示する
        // appl.add_systems(Startup, show_monitor_info.run_if(DEBUG));

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

        // ヘッダー／フッターを表示する
        // appl.insert_resource(header_footer::Settings(HEADER_FOOTER))
        //     .add_systems(
        //         OnEnter(MyState::InitGame),
        //         header_footer::spawn_header_footer,
        //     );

        // FPS表示
        // let bottomleft = header_footer::Position::BottomLeft;
        // let index = 1;
        // appl.add_plugins(FrameTimeDiagnosticsPlugin::default()) // FPS Plugin
        //     .insert_resource(DisplayInfoFps(bottomleft, index)) // 表示位置指定Resource
        //     .add_systems(Update, update_fps); // FPS表示の更新

        // 現在日時表示
        // let topleft = header_footer::Position::TopLeft;
        // appl.insert_resource(DisplayInfoDaytime(topleft)) // 表示位置指定Resource
        //     .add_systems(Update, uapdate_daytime); // リアルタイム（日時）の表示更新

        // 経過時間表示
        // let topright = header_footer::Position::TopRight;
        // appl.insert_resource(DisplayInfoElapsedTime(topright)) // 表示位置指定Resource
        //     .add_systems(Startup, set_elapsed_time_wrap_period) // timeのwrap変更(3600s->60s)
        //     .add_systems(Update, uapdate_elapsed_time); // アプリ実行経過時間の表示更新
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

// 端末がサポートしている解像度の一覧を表示する
// fn show_monitor_info(qry_monitor: Query<&Monitor>) -> Result
// {
//     // モニタの情報
//     let monitor = qry_monitor.single()?;
//     let gcd = misc::gcd_u32(monitor.physical_width, monitor.physical_height);
//     let w = monitor.physical_width / gcd;
//     let h = monitor.physical_height / gcd;
//     println!("{}:{}\n{:#?}", w, h, monitor);

//     // サポートしている解像度の一覧
//     for info in monitor.video_modes.iter()
//     {
//         let UVec2 { x, y } = info.physical_size;
//         let gcd = misc::gcd_u32(x, y);
//         let w = x / gcd;
//         let h = y / gcd;

//         println!("{}:{}\t{:?}", w, h, info);
//     }

//     Ok(())
// }

////////////////////////////////////////////////////////////////////////////////

// 更新対象の位置を指定するためのResource
// #[derive(Resource)]
// struct DisplayInfoFps(header_footer::Position, usize);
// #[derive(Resource)]
// struct DisplayInfoDaytime(header_footer::Position);
// #[derive(Resource)]
// struct DisplayInfoElapsedTime(header_footer::Position);

// FPSの表示を更新する
// fn update_fps(
//     opt_display_info: Option<Res<DisplayInfoFps>>,
//     qry_text_block: Query<(Entity, &header_footer::Position)>,
//     mut text_writer: TextUiWriter,
//     diag_store: Res<DiagnosticsStore>,
// ) -> Result
// {
//     // 準備
//     let display_info = opt_display_info.ok_or("DisplayInfoFps not found.")?;
//     let root_entity = qry_text_block
//         .iter()
//         .filter(|(_, position)| **position == display_info.0)
//         .collect::<Vec<(Entity, &header_footer::Position)>>();

//     // 書き換え
//     if !root_entity.is_empty()
//     {
//         let entity = root_entity[0].0;
//         let mut fps_text = text_writer.get_text(entity, display_info.1).ok_or(
//             format!("no entity with a matching index: {}", display_info.1),
//         )?;
//         const NA3_2: &str = "###.##";

//         *fps_text = diag_store.get(&FrameTimeDiagnosticsPlugin::FPS).map_or(
//             NA3_2.to_string(),
//             |fps| {
//                 fps.average()
//                     .map_or(NA3_2.to_string(), |avg| format!("{avg:06.2}"))
//             },
//         );
//     }

//     Ok(())
// }

// 日時表示を更新する
// fn uapdate_daytime(
//     opt_display_info: Option<Res<DisplayInfoDaytime>>,
//     mut qry_text_block: Query<(&mut Text, &header_footer::Position)>, /* 要素が先頭ならText */
// ) -> Result
// {
//     // 準備
//     let display_info = opt_display_info.ok_or("DisplayInfoDaytime not found.")?;

//     // 書き変え対象をfilterの条件で探す
//     qry_text_block
//         .iter_mut()
//         .filter(|(_, position)| **position == display_info.0)
//         .for_each(|(mut text, _)| {
//             // 表示書き換え
//             text.0 = time_local::now().format("%m/%d %H:%M:%S").to_string();
//         });

//     Ok(())
// }

// 経過時間を記録する準備
// fn set_elapsed_time_wrap_period(mut time: ResMut<Time<Real>>, mut cmd: Commands)
// {
//     // 経過秒数の周回を60秒へ変更
//     time.set_wrap_period(Duration::new(60, 0));

//     // 経過時間を記録するResourceを登録
//     cmd.insert_resource(ElapsedTime {
//         s: time.elapsed_secs_wrapped(),
//         ..default()
//     });
// }

// 経過時間を記録するResource
// #[derive(Resource, Default)]
// struct ElapsedTime
// {
//     d: i32,
//     h: i32,
//     m: i32,
//     s: f32,
// }

// 経過時間を更新する
// fn uapdate_elapsed_time(
//     opt_display_info: Option<Res<DisplayInfoElapsedTime>>,
//     opt_clock: Option<ResMut<ElapsedTime>>,
//     mut qry_text_block: Query<(&mut Text, &header_footer::Position)>,
//     time: Res<Time<Real>>,
// ) -> Result
// {
//     // 準備
//     let display_info =
//         opt_display_info.ok_or("DisplayInfoElapsedTime not found.")?;
//     let mut clock = opt_clock.ok_or("ElapsedTime not found.")?;

//     // 算出
//     let new_s = time.elapsed_secs_wrapped(); // 60秒で周回する経過秒数
//     if new_s < clock.s
//     {
//         // 秒が周回したら
//         clock.m += 1; // 分を＋１する
//         if clock.m >= 60
//         {
//             clock.h += clock.m / 60;
//             clock.m %= 60;
//         }
//         if clock.h >= 24
//         {
//             clock.d += clock.h / 24;
//             clock.h %= 24;
//         }
//     }
//     clock.s = new_s;

//     // 書き変え対象をfilterの条件で探す
//     qry_text_block
//         .iter_mut()
//         .filter(|(_, position)| **position == display_info.0)
//         .for_each(|(mut text, _)| {
//             // 表示書き換え
//             text.0 = format!(
//                 "{:02}:{:02}:{:02}:{:02.0}",
//                 clock.d,
//                 clock.h,
//                 clock.m,
//                 clock.s.floor(),// 秒数は、小数点以下切り捨て(format!が切り上げ表示する対策)
//             );
//         });

//     Ok(())
// }

////////////////////////////////////////////////////////////////////////////////

// End of code.
