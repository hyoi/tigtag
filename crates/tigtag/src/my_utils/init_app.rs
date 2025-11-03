use super::*;

////////////////////////////////////////////////////////////////////////////////

// プラグインの設定
#[rustfmt::skip]
pub struct Schedule { pub next: MyState }
#[rustfmt::skip]
impl Schedule { fn next_state(&self) -> MyState { self.next } }

impl Plugin for Schedule
{
    fn build(&self, application: &mut App)
    {
        // アプリの初期化
        application
            // エラーハンドラ設定
            .set_error_handler(warn)
            // first party plugins
            .add_plugins((
                DefaultPlugins
                    .set(WindowPlugin::initialize()) // 主ウィンドウ初期化
                    .set(LogPlugin::initialize()) // ログレベル初期化
                    .set(ImagePlugin::default_nearest()), // ピクセルパーフェクト
                FrameTimeDiagnosticsPlugin::default(), // FPS Plugin
            ))
            // Stateの初期化
            .init_state::<MyState>()
            // 汎用的な処理の登録
            .init_resource::<appctrl_input::ScaleFactor>() // 全画面表示のスケールファクター
            .add_systems(
                Update, // without MyState
                (
                    // gamepadの接続を検出して必要なら切り替える
                    handle_input::check_gamepad_connections,
                    (
                        // アプリの終了
                        appctrl_input::send_exit_app_message,
                        // 全画面切替（window.mode変更）
                        appctrl_input::toggle_fullscreen,
                        // スケールファクターを再計算して設定
                        appctrl_input::update_scale_factor
                            .after(appctrl_input::toggle_fullscreen)
                            .run_if(any_match_filter::<Changed<Window>>),
                        // ヘッダー／フッターの位置ずれを調整する
                        header_footer::adjust_header_footer_layout
                            .after(appctrl_input::toggle_fullscreen)
                            .run_if(any_match_filter::<Changed<Window>>),
                    )
                        .in_set(execution_order::Before::HitAnyKey)
                        .run_if(not(misc::WASM)), // WASMでは実行しない
                    // UI outline表示
                    appctrl_input::toggle_ui_outline_gizmo
                        .in_set(execution_order::Before::HitAnyKey)
                        .run_if(misc::DEBUG),
                ),
            )
            // システムセットの実行順の制御
            .configure_sets(
                Update,
                (
                    // HitAnyKey
                    execution_order::Before::HitAnyKey,
                    execution_order::Target::HitAnyKey,
                    execution_order::After::HitAnyKey,
                )
                    .chain(),
            );

        // ローディングアニメを表示しながらアセットをロードする
        application
            // 前処理
            .add_systems(
                OnEnter(MyState::LoadAssets),
                (
                    // スプライト（とカメラ）のspawn
                    spawn_sprite_with_camera2d,
                    // Assetsのロード開始
                    start_loading,
                ),
            )
            // ループ処理
            .add_message::<AssetsAllLoaded>() // ロード完了メッセージの登録
            .add_systems(
                Update, // within MyState::LoadAssets
                (
                    // スプライトを移動させる
                    move_sprite,
                    // ローディング完了を検知してフラグを立てる
                    check_loading_done,
                    // ループ脱出
                    misc::set_next_state(self.next_state())
                        .run_if(on_message::<AssetsAllLoaded>), // 完了メッセージ受信
                )
                    .run_if(in_state(MyState::LoadAssets)),
            )
            // 後処理
            .add_systems(
                OnExit(MyState::LoadAssets),
                (
                    // スプライトとカメラ（あれば）の削除
                    misc::despawn_component::<SpriteTile>,
                    misc::despawn_component::<LoadingAnimeCam2d>,
                ),
            );
    }
}

////////////////////////////////////////////////////////////////////////////////

// WindowPluginの初期化
#[rustfmt::skip]
pub trait InitWindowPlugin { fn initialize() -> Self; }
impl InitWindowPlugin for WindowPlugin
{
    fn initialize() -> Self
    {
        Self {
            primary_window: Some(MAIN_WINDOW.clone()),
            ..default()
        }
    }
}

// LogPluginの初期化
#[rustfmt::skip]
pub trait InitLogPlugin { fn initialize() -> Self; }
impl InitLogPlugin for LogPlugin
{
    fn initialize() -> Self
    {
        Self {
            filter: match misc::DEBUG()
            {
                true => LOG_FILTER_DEVELOP.into(),
                false => LOG_FILTER_RELEASE.into(),
            },
            ..default()
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

// ローディングアニメ用2DカメラのComponentとレンダリング順序、およびカメラの位置
#[derive(Component)]
pub struct LoadingAnimeCam2d;
const CAMERA_2D_ORDER: isize = 999; // マルチカメラの場合、順序が他とバッティングするとWARNが出力される
const CAMERA_2D_POSITION: Vec3 = Vec3::new(
    // 第四象限。左上隅が(0,0)で、X軸はプラス方向へ、Y軸はマイナス方向へ伸びる
    SCREEN_PIXELS_WIDTH * 0.5,
    SCREEN_PIXELS_HEIGHT * -0.5,
    999.0,
);

// ローディングメッセージのデザイン
struct LoadingMessage<'a>
{
    width: f32,
    height: f32,
    design: Vec<&'a str>,
}

impl<'a> Default for LoadingMessage<'a>
{
    fn default() -> Self
    {
        let design = vec![
            " ##  #           #                            ", // 0
            " ##  # ### #   # #    ###  #  ##  # #  #  ##  ", // 1
            " # # # # # # # # #    # # # # # #   ## # #    ", // 2
            " # # # # # # # # #    # # # # # # # #### # ## ", // 3
            " #  ## # #  # #  #    # # ### # # # # ## #  # ", // 4
            " #  ## ###  # #  #### ### # # ##  # #  #  ##  ", // 5
            "",                                               // 6
            "",                                               // 7
            " ###                      #   #           # # ", // 8
            " #  # #   ###  #  ### ### # # #  #  # ### # # ", // 9
            " #  # #   #   # # #   #   # # # # #    #  # # ", // 10
            " ###  #   ### # # ### ### # # # # # #  #  # # ", // 11
            " #    #   #   ###   # #    # #  ### #  #      ", // 12
            " #    ### ### # # ### ###  # #  # # #  #  # # ", // 13
        ]; // 123456789_123456789_123456789_123456789_12345

        LoadingMessage {
            width: design[0].len() as f32 * PIXELS_PER_GRID,
            height: design.len() as f32 * PIXELS_PER_GRID,
            design,
        }
    }
}

// ローディングアニメ用スプライトのz-indexとComponent
const DEPTH_SPRITE_LOADING_MSG: f32 = 999.0;

#[derive(Component)]
struct SpriteTile
{
    goal_cell: (i32, i32),
}

// スプライト（とカメラ）を生成する
fn spawn_sprite_with_camera2d(
    query_camera2d: Query<&Camera2d>,
    mut cmds: Commands,
) -> Result
{
    // カメラ2Dが存在しないなら
    if query_camera2d.is_empty()
    {
        // 専用2Dカメラをspawnする
        cmds.spawn((
            LoadingAnimeCam2d, // マーカーComponent（despawnに使う）
            Camera2d,
            Camera {
                order: CAMERA_2D_ORDER,
                ..default()
            },
            Transform::from_translation(CAMERA_2D_POSITION),
        ));
    }

    // デザインに従ってスプライトをspawnする
    let message = LoadingMessage::default();
    let mut rng = rand::rng();
    (0..).zip(message.design.iter()).for_each(|(y, line)| {
        (0..).zip(line.chars()).for_each(|(x, char)| {
            // 空白文字でないなら
            if char != ' '
            {
                // スプライトの初期座標(スタート)はランダム
                let rnd_x = rng.random_range(GRIDS_X_RANGE);
                let rnd_y = rng.random_range(GRIDS_Y_RANGE);
                let translation = (rnd_x, rnd_y)
                    .to_screen_pixels()
                    .extend(DEPTH_SPRITE_LOADING_MSG);

                // スプライトをspawnする
                cmds.spawn((
                    Sprite {
                        color: COLOR_YELLOW,
                        custom_size: Some(CELL_CUSTOM_SIZE * 0.9),
                        ..default()
                    },
                    Transform::from_translation(translation),
                    SpriteTile { goal_cell: (x, y) },
                ));
            }
        });
    });

    Ok(())
}

// スプライトを動かす（ローディングアニメーション）
fn move_sprite(
    mut query_transform: Query<(&mut Transform, &SpriteTile)>,
    time: Res<Time>,
    setting: Local<LoadingMessage>, // 初回のみdefault()で初期化
) -> Result
{
    // 準備
    let time_delta = time.delta().as_secs_f32() * 2.0;
    let scaling = SCREEN_PIXELS_WIDTH / setting.width; // 横方向に長いのでWidthを使う
    let adjuster_y = (SCREEN_PIXELS_HEIGHT - setting.height * scaling) * 0.5;

    // スプライトの移動
    query_transform
        .iter_mut()
        .for_each(|(mut transform, sprite)| {
            // 座標の調整
            let mut goal = sprite.goal_cell.to_screen_pixels() * scaling;
            goal.y -= adjuster_y;

            // ゴールへ向かってまっすぐ、速度を落としながら移動
            let now = &mut transform.translation;
            now.x += (goal.x - now.x) * time_delta;
            now.y += (goal.y - now.y) * time_delta;
        });

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// ロードしたAssetsのハンドルの保存先
#[derive(Resource, Deref)]
struct LoadedAssets(Vec<Handle<LoadedUntypedAsset>>);

// Assetsのロードを開始する
fn start_loading(mut cmds: Commands, asset_svr: Res<AssetServer>) -> Result
{
    // Assetsのロードを開始
    let mut handles = Vec::new();
    PRELOAD_ASSETS
        .iter()
        .for_each(|fname| handles.push(asset_svr.load_untyped(*fname)));

    // 解放しないようリソースに登録する
    cmds.insert_resource(LoadedAssets(handles));

    Ok(())
}

// ローディング完了メッセージ
#[derive(Message)]
struct AssetsAllLoaded;

// Assetsのロードは完了したか？
fn check_loading_done(
    assets: Res<LoadedAssets>,
    asset_svr: Res<AssetServer>,
    mut message_assets_all_loaded: MessageWriter<AssetsAllLoaded>,
) -> Result
{
    // 事前ロードが完了したか？
    for handle in assets.iter()
    {
        match asset_svr.get_load_state(handle)
        {
            Some(LoadState::Loaded) => (), // ロード完了
            Some(LoadState::Failed(err)) =>
            {
                // ロード失敗⇒パニック
                dbg!(err); // for debug
                let mut filename = "Unknown".to_string();
                if let Some(asset_path) = handle.path()
                    && let Some(s) = asset_path.path().to_str()
                {
                    filename = s.to_string();
                }
                panic!("Failed loading asset file \"{filename}\"");
            }
            _ => return Ok(()), // スケジュールがUPDATEなので繰り返し実行
        }
    }

    // ローディング完了を通知
    message_assets_all_loaded.write(AssetsAllLoaded);

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
