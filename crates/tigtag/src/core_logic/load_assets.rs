use super::*;

////////////////////////////////////////////////////////////////////////////////

// プラグインの設定
pub struct Schedule
{
    pub in_state: MyState,   // 事前ロードを実行するState
    pub next_state: MyState, // 事前ロード完了後の遷移先State
    pub target_assets: &'static [&'static str], // ロード対象のリスト
}

impl Plugin for Schedule
{
    fn build(&self, application: &mut App)
    {
        // ローディングアニメを表示しながらアセットをロードする
        application
            // 前処理
            .add_systems(
                OnEnter(self.in_state),
                (
                    // スプライト（とカメラ）のspawn
                    spawn_sprite_with_camera2d,
                    // Assetsのロード開始
                    start_loading(self.target_assets),
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
                    set_next_state(self.next_state)
                        .run_if(on_message::<AssetsAllLoaded>), // 完了メッセージ受信
                )
                    .run_if(in_state(self.in_state)),
            )
            // 後処理
            .add_systems(
                OnExit(self.in_state),
                (
                    // スプライトとカメラ（あれば）の削除
                    misc::despawn_component::<SpriteTile>,
                    misc::despawn_component::<LoadingAnimeCam2d>,
                ),
            );
    }
}

////////////////////////////////////////////////////////////////////////////////

// ローディングアニメ用2DカメラのComponent
#[derive(Component)]
struct LoadingAnimeCam2d;

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
            width: design[0].len() as f32 * PIXELS_PER_CELL,
            height: design.len() as f32 * PIXELS_PER_CELL,
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
    query_camera2d: Query<&Camera2d>, //
    mut cmds: Commands, //
)
{
    // カメラ2Dが存在しないなら
    if query_camera2d.is_empty()
    {
        // 専用2Dカメラをspawnする
        let _id = cmds
            .spawn((
                LoadingAnimeCam2d, // マーカーComponent
                Camera2d,
                Camera {
                    order: CAMERA_ORDER_OVERLAY,
                    clear_color: CAMERA_BG_COLOR_OVERLAY.into(),
                    ..default()
                },
                Transform::from_translation(POSITION_CAMERA_2D),
            ))
            .id();

        // WASMでないならカメラをフルスクリーン処理の対象にする
        #[cfg(not(target_arch = "wasm32"))]
        cmds.entity(_id).insert(fullscreen::Target::default());
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
                let rnd_x = rng.random_range(0..WINDOW_CELLS_WIDTH as i32);
                let rnd_y = rng.random_range(0..WINDOW_CELLS_HEIGHT as i32);
                let translation = (rnd_x, rnd_y)
                    .to_screen_pixels()
                    .extend(DEPTH_SPRITE_LOADING_MSG);

                // スプライトをspawnする
                cmds.spawn((
                    SpriteTile { goal_cell: (x, y) }, // マーカーComponent
                    Sprite {
                        color: Color::Srgba(css::YELLOW),
                        custom_size: Some(Vec2::splat(PIXELS_PER_CELL) * 0.9),
                        ..default()
                    },
                    Transform::from_translation(translation),
                ));
            }
        });
    });
}

// スプライトを動かす（ローディングアニメーション）
fn move_sprite(
    mut query_transform: Query<(&mut Transform, &SpriteTile)>,
    time: Res<Time>,
    setting: Local<LoadingMessage>, // 初回のみdefault()で初期化
)
{
    // 準備
    let time_delta = time.delta().as_secs_f32() * 2.0;
    let scaling = WINDOW_PIXELS_WIDTH / setting.width; // 横方向に長いのでWidthを使う
    let adjuster_y = (WINDOW_PIXELS_HEIGHT - setting.height * scaling) * 0.5;

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
}

////////////////////////////////////////////////////////////////////////////////

// ロードしたAssetsのハンドルの保存先
#[derive(Resource, Deref)]
struct LoadedAssets(Vec<Handle<LoadedUntypedAsset>>);

// Assetsのロードを開始する
fn start_loading(
    target_assets: &'static [&'static str], // 引数でロード対象のリストをもらう
) -> impl FnMut(
    Commands,         // (a)
    Res<AssetServer>, // (b)
)
{
    // クロージャを返す
    move |
        mut cmds: Commands, // (a)
        asset_svr: Res<AssetServer>, // (b)
    |
    {
        // Assetsのロードを開始
        let mut handles = Vec::new();
        target_assets
            .iter()
            .for_each(|fname| handles.push(asset_svr.load_untyped(*fname)));

        // 解放しないようリソースに登録する
        cmds.insert_resource(LoadedAssets(handles));
    }
}

// ローディング完了メッセージ
#[derive(Message)]
struct AssetsAllLoaded;

// Assetsのロードは完了したか？
fn check_loading_done(
    assets: Res<LoadedAssets>,
    asset_svr: Res<AssetServer>,
    mut message_assets_all_loaded: MessageWriter<AssetsAllLoaded>,
)
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
            _ => return, // スケジュールがUPDATEなので繰り返し実行
        }
    }

    // ローディング完了を通知
    message_assets_all_loaded.write(AssetsAllLoaded);
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
