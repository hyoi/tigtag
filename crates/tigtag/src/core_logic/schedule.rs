use super::*;

////////////////////////////////////////////////////////////////////////////////

// プラグインの設定
pub struct Schedule;
impl Plugin for Schedule
{
    fn build(&self, application: &mut App)
    {
        //--------------------------------------------------------------------------
        // 各種登録
        application
            // スケジュールの追加
            .add_plugins(init_app::Schedule { next: MyState::Initialize } ) // アプリ初期化とアセットロード
            .add_plugins(demo_play::Schedule)                               // デモプレイ
            .add_plugins(overlay_ui::pause_menu::Schedule)                  // Pauseメニュー

            // Resourceの登録
            .init_resource::<CameraSettings>()                  // カメラの設定を登録
            .init_resource::<Record>()                          // ゲームの成績
            .init_resource::<map::Map>()                        // ステージのマップ
            .init_resource::<misc::MaskHitAnyKeyInput>()        // 「Hit Any Key」の入力マスク
            .insert_resource(handle_input::MappingKeyboard::from(KEYBOARD_MAP)) // マッピング
            .insert_resource(handle_input::MappingGamepad::from(GAMEPAD_MAP))   // マッピング

            // Messageの登録
            .add_message::<misc::AnyButtonPressed>() //「Hit Any Key」の入力通知
            .add_message::<SkipOverlayMessage>()     // 全画面メッセージ表示のスキップに使用
            .add_message::<CountDownEnded>()         // カウントダウンの終了通知
            .add_message::<handle_input::MessUserAction>() // デバイスからの入力
            .add_message::<DotsAllEaten >()          // ステージクリアの伝達用
            .add_message::<DotEaten>()               // スコアリングの伝達用
            .add_message::<PlayerCaught>()           // ゲームオーバーの伝達用
            .add_message::<PlayerPositionAdjusted>() // プレイヤーの位置補正の伝達用
            .add_message::<ChaserPositionAdjusted>() // チェイサーの位置補正の伝達用
            ;

        //--------------------------------------------------------------------------
        // 初期化（MyState::Initialize）
        application
            // 前処理
            .add_systems(
                OnEnter(MyState::Initialize),
                (
                    // カメラのspawn
                    simple_camera::spawn::<CameraSettings>,
                    // UIを描画するカメラの選択
                    misc::set_ui_camera::<SimpleCamera2d>
                        .after(simple_camera::spawn::<CameraSettings>),
                    // TextUIのspawn
                    header_footer::spawn, // ヘッダー／フッター
                    overlay_ui::messages::spawn, //全画面メッセージ
                    // 無条件遷移
                    misc::set_next_state(MyState::TitleDemo),
                ),
            );

        //--------------------------------------------------------------------------
        // 常に実行する処理（Update without MyState）
        application
            // ループ処理
            .add_systems(
                Update, // without MyState
                (
                    // ヘッダーとフッターの表示情報を更新する
                    information::update_header_footer,
                    // スプライトアニメーション
                    animate_sprites::<player::Player>, // プレイヤー
                    animate_sprites::<chaser::Chaser>, // チェイサー
                    // スプライト表示OFFの場合のアニメーション
                    chaser::rotate_chaser_shape.run_if(SPRITE_OFF), // チェイサー回転
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
                    misc::check_hit_any_key
                        .in_set(misc::execution_order::Target::HitAnyKey),
                    (
                        // scoreとstageをゼロクリアする(demoの情報消去)
                        detecting_change::initialize_score_stage,
                        // Stateを変更
                        misc::set_next_state(MyState::StageStart),
                    )
                        .in_set(misc::execution_order::After::HitAnyKey)
                        .run_if(on_message::<misc::AnyButtonPressed>),
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
            );

        //--------------------------------------------------------------------------
        // ゲーム開始処理（MyState::StageStart）
        application
            // 前処理
            .add_systems(
                OnEnter(MyState::StageStart),
                (
                    // 全画面メッセージの表示スキップ指示があるなら即メインループへ
                    misc::set_next_state(MyState::MainLoop)
                        .run_if(on_message::<SkipOverlayMessage>),
                    // 全画面メッセージ（ステージ開始）表示
                    (
                        OverlayStageStart::init(),
                        misc::show_component::<OverlayStageStart>
                            .after(OverlayStageStart::init()),
                    )
                        .run_if(not(on_message::<SkipOverlayMessage>)),
                    // ステージ初期化
                    map::make_new_stage_data, // マップデータ
                    (
                        map::spawn_sprite,    // マップスプライト
                        player::spawn_sprite, // プレーヤースプライト
                        chaser::spawn_sprite, // チェイサースプライト
                    )
                        .after(map::make_new_stage_data),
                ),
            )
            // ループ処理
            .add_systems(
                Update, // within MyState::StageStart
                (
                    // カウントダウン完了後にState遷移
                    overlay_ui::effect::countdown::<OverlayStageStart>,
                    misc::set_next_state(MyState::MainLoop)
                        .run_if(on_message::<CountDownEnded>)
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

        // --------------------------------------------------------------------------
        // メインループ処理（MyState::MainLoop）
        application
            // ループ処理
            .add_systems(
                Update, // within MyState::MainLoop
                (
                    (
                        // スプライトの位置を更新する
                        handle_input::check_keyboard, // キー
                        handle_input::check_gamepad,  // ゲームパッド
                        player::move_sprite
                            .after(handle_input::check_keyboard)
                            .after(handle_input::check_gamepad)
                            .run_if(on_message::<handle_input::MessUserAction>),
                        chaser::move_sprite,
                    ),
                    // スコアリング＆クリア判定
                    detecting_change::scoring_and_stage_clear,
                    misc::set_next_state(MyState::StageClear)
                        .run_if(on_message::<DotsAllEaten>),
                    // 衝突判定
                    detecting_change::collisions_and_gameover
                        // DotsAllEaten ➡ スキップ
                        .run_if(not(on_message::<DotsAllEaten>)),
                    misc::set_next_state(MyState::GameOver)
                        .run_if(on_message::<PlayerCaught>),
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
                    misc::set_next_state(MyState::StageStart)
                        .run_if(on_message::<CountDownEnded>)
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
                    misc::set_message::<SkipOverlayMessage>,
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
            // ループ処理
            .add_systems(
                Update, // within MyState::GameOver
                (
                    // カウントダウン完了後にState遷移
                    overlay_ui::effect::countdown::<OverlayGameOver>,
                    misc::set_next_state(MyState::TitleDemo)
                        .run_if(on_message::<CountDownEnded>)
                        .after(overlay_ui::effect::countdown::<OverlayGameOver>),
                    // Replay? の明滅
                    overlay_ui::effect::blinking_text::<OverlayGameOver>,
                    // Hit ANY Key に反応あればState遷移
                    misc::check_hit_any_key
                        .in_set(misc::execution_order::Target::HitAnyKey),
                    misc::set_next_state(MyState::StageStart)
                        .in_set(misc::execution_order::After::HitAnyKey)
                        .run_if(on_message::<misc::AnyButtonPressed>),
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
    }
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
