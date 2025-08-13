use super::*;

////////////////////////////////////////////////////////////////////////////////

// ローディングが完了した後の遷移先
#[derive(Resource)]
pub struct NextState(pub MyState);
impl ChangeMyState for NextState
{
    fn state(&self) -> MyState { self.0 }
}

////////////////////////////////////////////////////////////////////////////////

// プラグインの設定
pub struct Schedule;
impl Plugin for Schedule
{
    fn build(&self, appl: &mut App)
    {
        //----------------------------------------------------------------------
        // MyState::LoadAssets
        //----------------------------------------------------------------------

        // ロード中のアニメーション
        appl.add_systems(
            OnEnter(MyState::LoadAssets),
            spawn_sprite_with_camera2d, // アニメ用のカメラとスプライトのspawn
        )
        .add_systems(
            Update,
            move_sprite // スプライトを移動させる
                .run_if(in_state(MyState::LoadAssets)),
        );

        // アセットのローディング
        appl.add_systems(
            OnEnter(MyState::LoadAssets),
            start_loading, // Assetsのロード開始
        )
        .add_systems(
            Update,
            (
                // ローディング完了フラグを立てる
                is_loading_done,
                // ループ脱出
                change_state_by::<NextState>
                    .run_if(resource_exists::<IsLoadingFinished>) //完了フラグが立つこと
                    .run_if(resource_exists::<NextState>), //遷移先がinsert済であること
            )
                .run_if(in_state(MyState::LoadAssets)),
        );

        // 後処理
        appl.add_systems(
            OnExit(MyState::LoadAssets),
            (
                // LoadingAnimeのスプライトの削除
                misc::despawn_component::<SpriteTile>,
                // 専用カメラの削除
                misc::despawn_component::<LoadingAnimeCam2d>,
            ),
        );
    }
}

////////////////////////////////////////////////////////////////////////////////

// ローディングアニメ用2Dカメラのレンダリング順序と位置、マーカー
// ※注意事項：マルチカメラの場合、順序が他とバッティングするとWARNが出力される
const CAMERA_2D_ORDER: isize = 999;

// ※注意事項：第四象限。左上隅が(0,0)で、X軸はプラス方向へ、Y軸はマイナス方向へ伸びる
const CAMERA_2D_POSITION: Vec3 = Vec3::new(
    SCREEN_PIXELS_WIDTH * 0.5,
    SCREEN_PIXELS_HEIGHT * -0.5,
    999.0,
);

#[derive(Component)]
pub struct LoadingAnimeCam2d;

// ローディングメッセージ
static NOWLOADING: LazyLock<LoadingMessage> = LazyLock::new(|| {
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
});

struct LoadingMessage<'a>
{
    width: f32,
    height: f32,
    design: Vec<&'a str>,
}

// ローディングアニメ用スプライトのz-indexとマーカー
const DEPTH_SPRITE_LOADING_MSG: f32 = 999.0;

#[derive(Component)]
struct SpriteTile
{
    goal: (i32, i32),
}

// ローディングアニメ用スプライトを生成する
fn spawn_sprite_with_camera2d(mut cmds: Commands)
{
    // 準備
    let mut rng = rand::rng();
    let color = css::YELLOW.into();
    let custom_size = Some(GRID_CUSTOM_SIZE * 0.9);

    // 専用2Dカメラをspawnする
    cmds.spawn((
        Camera2d,
        Camera {
            order: CAMERA_2D_ORDER,
            ..default()
        },
        Transform::from_translation(CAMERA_2D_POSITION),
        LoadingAnimeCam2d, // マーカー
    ));

    // デザインに従ってスプライトをspawnする
    for (goal_y, line) in NOWLOADING.design.iter().enumerate()
    {
        for (goal_x, char) in line.chars().enumerate()
        {
            // 空白文字は無視
            if char == ' '
            {
                continue;
            }

            // スプライトの初期座標(スタート)
            let rnd_x = rng.random_range(GRIDS_X_RANGE);
            let rnd_y = rng.random_range(GRIDS_Y_RANGE);
            let vec2 = (rnd_x, rnd_y).to_vec2_of_screen();
            let vec3 = vec2.extend(DEPTH_SPRITE_LOADING_MSG);

            // スプライトをspawnする
            cmds.spawn((
                Sprite {
                    color,
                    custom_size,
                    ..default()
                },
                Transform::from_translation(vec3),
                SpriteTile {
                    goal: (goal_x as i32, goal_y as i32),
                },
            ));
        }
    }
}

// スプライトを動かしてローディングアニメを見せる
fn move_sprite(
    mut qry_transform: Query<(&mut Transform, &SpriteTile)>,
    time: Res<Time>,
)
{
    let time_delta = time.delta().as_secs_f32() * 2.0;

    let scaling = SCREEN_PIXELS_WIDTH / NOWLOADING.width; // 横方向に長いのでWidthを使う
    let adjuster_y = (SCREEN_PIXELS_HEIGHT - NOWLOADING.height * scaling) * 0.5;

    qry_transform
        .iter_mut()
        .for_each(|(mut transform, sprite)| {
            // 座標の調整
            let mut goal = sprite.goal.to_vec2_of_screen() * scaling;
            goal.y -= adjuster_y;

            // ゴールへ向けてスプライトを移動
            let now = &mut transform.translation;
            now.x += (goal.x - now.x) * time_delta;
            now.y += (goal.y - now.y) * time_delta;
        });
}

////////////////////////////////////////////////////////////////////////////////

// ロードしたAssetsのハンドルの保存先とローディング完了フラグ
#[derive(Resource)]
struct LoadedAssets
{
    handles: Vec<Handle<LoadedUntypedAsset>>,
}

#[derive(Resource)]
struct IsLoadingFinished;

// Assetsのロードを開始する
fn start_loading(mut cmds: Commands, asset_svr: Res<AssetServer>)
{
    // Assetsのロードを開始
    let mut handles = Vec::new();
    PRELOAD_ASSETS
        .iter()
        .for_each(|fname| handles.push(asset_svr.load_untyped(*fname)));

    // 解放しないようリソースに登録する
    cmds.insert_resource(LoadedAssets { handles });
}

// Assetsのロードは完了したか？
fn is_loading_done(
    assets: Res<LoadedAssets>,
    mut cmds: Commands,
    asset_svr: Res<AssetServer>,
)
{
    // 事前ロードが完了したか？
    for handle in assets.handles.iter()
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
                panic!("Error: Failed loading asset file \"{filename}\"");
            }
            _ => return, // ロード中（スケジュールがUPDATEなので繰り返し実行される）
        }
    }

    // ローディング完了フラグを立てる
    cmds.insert_resource(IsLoadingFinished);
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
