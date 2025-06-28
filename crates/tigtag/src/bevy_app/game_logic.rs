use super::*;

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
        appl.insert_resource(load_assets::NextState(MyState::InitGame)) // 完了後のState遷移先
            .add_plugins(load_assets::Schedule);

        //----------------------------------------------------------------------

        // Resource
        appl.init_resource::<Record>() //ゲームの成績
            .init_resource::<Map>()    //ステージのマップ
        //     .init_resource::<player::InputDirection>(); //プレイヤーの入力(十字方向)
        ;

        // Event
        // appl.add_event::<EventClear>()  //ステージクリアの伝達
        //     .add_event::<EventOver>()   //ゲームオーバーの伝達
        //     .add_event::<EventEatDot>() //スコアリングの伝達
        //     .add_event::<EventTimerPlayer>()  //自キャラ移動タイマーのfinishedの伝達
        //     .add_event::<EventTimerChasers>(); //敵キャラ移動タイマーのfinishedの伝達

        // plugin
        // appl.add_plugins( header::Schedule ) //ヘッダー更新(Stage、Score、HiScore)
        //     .add_plugins( demo::Schedule   ) //タイトル画面のデモプレイ
        //     .add_plugins( pause::Schedule  ); //Pause処理

        //----------------------------------------------------------------------
        // Update
        //----------------------------------------------------------------------

        // ゲームパッドの接続状態を検出する
        appl.add_systems(Update, misc::detect_gamepad_connection);

        // FPS表示
        let bottomleft = header_footer::Position::BottomLeft;
        let index = 1;
        appl.add_plugins(FrameTimeDiagnosticsPlugin::default()) // FPS Plugin
            .insert_resource(DisplayInfoFps(bottomleft, index)) // 表示位置の指定（Resource）
            .add_systems(Update, update_fps::<DisplayInfoFps>); // FPS表示の更新

        // アニメーション（ゲーム中もPAUSE中も）
        // appl.add_systems
        //     (   Update,
        //         (   //スプライトシートアニメーション
        //             animating_sprites::<player::Player>,
        //             animating_sprites::<chasers::Chaser>,

        //             //チェイサーの回転(スプライトシートがOFFの場合)
        //             chasers::rotate_chaser_shape.run_if( SPRITE_OFF ),
        //         )
        //     );

        // アプリ終了キーをフックして処理を挿入
        // appl.add_systems(
        //     Update,
        //     hook_exit_app_key // Pause等の雛型
        //         .before(misc::app_close_on_key),
        // );

        //----------------------------------------------------------------------
        // MyState::InitGame
        //----------------------------------------------------------------------

        // カメラ（Camera2dとCamera3d）をspawnする
        appl.insert_resource(simple_camera::Settings(CAMERA_SETTINGS.clone()))
            .add_systems(
                OnEnter(MyState::InitGame),
                simple_camera::spawn::<simple_camera::Settings>,
            );

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

        // ゲーム初期化
        // appl.add_systems
        //     (   OnEnter ( MyState::InitGame ),
        //         (   //TextUIの準備
        //             title_demo ::spawn_text,
        //             stage_start::spawn_text,
        //             stage_clear::spawn_text,
        //             game_over  ::spawn_text,

        //             //無条件遷移
        //             change_state_to::<TitleDemo>,
        //         )
        //     );

        // 無条件遷移
        appl.add_systems(OnEnter(MyState::InitGame), change_state_to::<StageStart>);

        //----------------------------------------------------------------------
        // MyState::StageStart
        //----------------------------------------------------------------------

        // ステージ初期化
        appl.add_systems
        (   OnEnter ( MyState::StageStart ),
            (   (   //マップデータ生成
                    map::make_new_stage_data,

                    //スプライトのspawn
                    (   map::spawn_sprite,
        //                 player::spawn_sprite,
        //                 chasers::spawn_sprite,
                    ),
                )
                .chain(), //実行順の固定
/*
        //         //TextUIの可視化
        //         (   effect::init_count::<stage_start::CountDown>, //カウント初期化
        //             misc::show_component::<stage_start::Message>,
        //         )
        //         .chain(), //実行順の固定
*/
            )
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
        // MyState::MainLoop
        //----------------------------------------------------------------------

        // メインループ
        // appl.add_systems
        // (   Update,
        //     (   //ループ脱出条件
        //         detection::scoring_and_stage_clear, //スコアリング＆クリア判定
        //         change_state_to::<StageClear>.run_if( on_event::<EventClear>() ),

        //         detection::collisions_and_gameover, //衝突判定
        //         change_state_to::<GameOver>.run_if( on_event::<EventOver>() ),

        //         //スプライトの移動
        //         (   //自キャラ
        //             (   player::catch_input_direction,
        //                 player::move_sprite,
        //             )
        //             .chain(), //実行順の固定

        //             //敵キャラ
        //             chasers::move_sprite,
        //         )
        //     )
        //     .chain() //実行順の固定
        //     .run_if( in_state( MyState::MainLoop ) )
        // );

        //----------------------------------------------------------------------
        // MyState::StageClear
        //----------------------------------------------------------------------

        // ステージクリアの処理
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
        // MyState::GameOver
        //----------------------------------------------------------------------

        // ゲームオーバーの処理
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
        // .add_systems
        // (   OnExit ( MyState::GameOver ),
        //     (   //TextUIの不可視化
        //         misc::hide_component::<game_over::Message>,

        //         //scoreとstageをゼロクリアする
        //         initialize_record_except_hi_score
        //     )
        // );

        //----------------------------------------------------------------------
        // MyState::TitleDemo
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
    }
}

////////////////////////////////////////////////////////////////////////////////

// キャラクターをアニメーションさせる
// fn animating_sprites<T: Component + CharacterAnimation>
// (   mut qry_target: Query<( &mut TextureAtlas, &mut T )>,
//     time: Res<Time>,
// )
// {   for ( mut sprite, mut character ) in &mut qry_target
//     {   if character.anime_timer_mut().tick( time.delta() ).just_finished()
//         {   sprite.index += 1;
//             let offset = character.sprite_sheet_offset( character.direction() );
//             let frame  = character.sprite_sheet_frame();
//             if sprite.index as u32 >= offset + frame { sprite.index = offset as usize }
//         }
//     }
// }

////////////////////////////////////////////////////////////////////////////////

// ScoreとStageの初期化
// pub fn initialize_record_except_hi_score
// (   opt_record: Option<ResMut<Record>>,
// )
// {   let Some ( mut record ) = opt_record else { return };

//     //クリアフラグが立っていた場合
//     if record.is_clear()
//     {   *record.is_clear_mut() = false;
//         return;
//     }

//     //scoreとstageをゼロクリア
//     *record.score_mut() = 0;
//     *record.stage_mut() = 0;
// }

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
