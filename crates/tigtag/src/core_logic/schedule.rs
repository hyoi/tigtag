use super::*;

////////////////////////////////////////////////////////////////////////////////

// プラグインの設定
pub struct MainSchedule;
impl Plugin for MainSchedule
{
    fn build(&self, application: &mut App)
    {
        //--------------------------------------------------------------------------
        // 各種登録
        application
            // スケジュールの追加
            .add_plugins(init_app::Schedule)                          // アプリ初期化とアセットロード
            .insert_resource(init_app::ChangeTo(MyState::Initialize)) // 処理が完了したらState変更
            .add_plugins(overlay_ui::pause_menu::Schedule)            // Pauseメニュー
            .add_plugins(demo_play::Schedule)                         // デモプレイ

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
            // ループ処理
            .add_systems(
                Update, // without MyState
                (
                    // スプライトアニメーション
                    animate_sprites::<player::Player>, // プレイヤー
                    animate_sprites::<chaser::Chaser>, // チェイサー
                    // スプライト表示OFFの場合のアニメーション
                    chaser::rotate_chaser_shape.run_if(SPRITE_OFF), // チェイサー回転
                    // ヘッダーとフッターの表示情報を更新する
                    information::update_header_footer,
                ),
            )
            // システムセット間の実行順序を制御する
            .configure_sets(
                Update,
                (
                    // BeforeHitAnyKey は HitAnyKey の前に実行
                    MyLabel::BeforeHitAnyKey.before(MyLabel::HitAnyKey),
                    // AfterHitAnyKey は HitAnyKey の後に実行
                    MyLabel::AfterHitAnyKey.after(MyLabel::HitAnyKey),
                ),
            );

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
                    // 無条件遷移
                    set_next_state::<TitleDemo>,
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
            );

        //--------------------------------------------------------------------------
        // ゲーム開始処理（MyState::StageStart）
        application
            // 前処理
            .add_systems(
                OnEnter(MyState::StageStart),
                (
                    // ステージ初期化
                    map::make_new_stage_data, // マップデータ
                    (
                        map::spawn_sprite,    // マップスプライト
                        player::spawn_sprite, // プレーヤースプライト
                        chaser::spawn_sprite, // チェイサースプライト
                    )
                        .after(map::make_new_stage_data),
                    // 全画面メッセージの表示スキップ指示があるなら即メインループへ
                    set_next_state::<MainLoop>
                        .run_if(on_event::<SkipOverlayMessage>),
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
            // ループ処理
            .add_systems(
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
    }
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
