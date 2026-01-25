// external crates
use bevy::{
    prelude::*,
    window::WindowMode,
    camera::{Viewport, ScalingMode},
};

// external modules
use crate::my_utils::plugins::controller::{
    InputDeviceConfig, //
    InputDevicePack,   //
};

////////////////////////////////////////////////////////////////////////////////

// フルスクリーン切替処理のスケジュール
pub struct PluginConfig
{
    pub base_resolution: Vec2,
    pub toggle_trigger: InputDeviceConfig,
}

impl Plugin for PluginConfig
{
    fn build(&self, application: &mut App)
    {
        application
            // コントローラーからの入力の処理
            .insert_resource(InputTargets(self.toggle_trigger))
            .add_systems(
                Update,
                catch_input_targets, //.in_set(execution_order::Before::HitAnyKey)
            )
            // window.modeを切替えるオブザーバーを登録
            .add_observer(toggle_fullscreen::<ToggleEvent>)
            // viewportをセットする（アスペクト比を変えずにフルスクリーン表示するため）
            .add_systems(Update, fit_viewport_to_fullscreen)
            .insert_resource(AppBaseResolution(self.base_resolution))
            .insert_resource(BackupResolution(self.base_resolution))
            .insert_resource(ScaleFactor(1.0))
            // 正射影カメラ(Camera2d)のスケーリングモードをFixedに設定する
            .add_systems(Update, fix_camera2d_projection_mode)
            // フルスクリーン中に生成されたカメラにviewportをセットする
            .add_systems(Update, fit_new_camera_viewport_to_fullscreen)
            .insert_resource(FullscreenState(false));
    }
}

//------------------------------------------------------------------------------

// フルスクリーン切替処理のトリガーEvent
#[derive(Event)]
pub struct ToggleEvent;

// フルスクリーン切替処理の対象カメラのマーカーComponent
#[derive(Component, Default, Deref, Clone)]
pub struct Target(pub Option<Viewport>);

//------------------------------------------------------------------------------

// 設計上の解像度を保存するResource
#[derive(Resource, Deref)]
struct AppBaseResolution(Vec2);

// 解像度の変化を検知する目的で現在値を保存するResource
#[derive(Resource)]
struct BackupResolution(Vec2);

// 設計上の解像度を1.0とした場合の比を保存するResource
#[derive(Resource)]
struct ScaleFactor(f32);

// 現状がフルスクリーンか(true)、ウィンドウか(false)を保存するResource
#[derive(Resource)]
struct FullscreenState(bool);

////////////////////////////////////////////////////////////////////////////////

// 入力の定を登録するためのResource
#[derive(Resource)]
struct InputTargets(InputDeviceConfig);

// コントローラーの入力を処理する
fn catch_input_targets(
    targets: Res<InputTargets>,
    mut input_devices: InputDevicePack,
    mut cmds: Commands,
)
{
    // キー／ボタンが押下されたなら
    if input_devices.is_pressed_with_reset(&targets.0)
    {
        // フルスクリーンの状態を切り替える
        cmds.trigger(ToggleEvent);
    }
}

////////////////////////////////////////////////////////////////////////////////

// ウィンドウ／フルスクリーンを切替える(トグル動作)
fn toggle_fullscreen<T: Event>(
    _trigger: On<T>, // Observerのトリガー
    mut window: Single<&mut Window>,
)
{
    info!("-----");
    info!("ToggleEvent fired");

    // window.modeを更新する
    use WindowMode::*;
    match window.mode
    {
        Windowed =>
        {
            // ウィンドウ => フルスクリーン
            window.mode = BorderlessFullscreen(MonitorSelection::Current);
        }
        _ =>
        {
            // フルスクリーン => ウィンドウ
            window.mode = Windowed;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

// viewportをセットする（アスペクト比を変えずにフルスクリーン表示するため）
fn fit_viewport_to_fullscreen(
    window: Single<&Window>,
    mut backup_resolution: ResMut<BackupResolution>,
    mut scale_factor: ResMut<ScaleFactor>,
    app_base_resolution: Res<AppBaseResolution>,
    mut ui_scale: ResMut<UiScale>,
    mut query_target_camera: Query<(&mut Camera, &mut Target)>,
    mut fullscreen_state: ResMut<FullscreenState>,
)
{
    // 現在の解像度を取得
    let window_width = window.resolution.physical_width() as f32;
    let window_height = window.resolution.physical_height() as f32;
    let now_resolution = (window_width, window_height).into();

    // 解像度に変化がないなら
    if now_resolution == backup_resolution.0
    {
        return;
    }
    info!("-----＜解像度の変化を検知＞-----");
    info!("backup_resolution: {}", backup_resolution.0);
    info!("window.mode: {:?}", window.mode);
    info!("now_resolution: {}", now_resolution);

    // 新たな解像度を保存する
    backup_resolution.0 = now_resolution;

    // window.modeがフルスクリーンなら（ウィンドウからフルスクリーンへ）
    if matches!(window.mode, WindowMode::BorderlessFullscreen(_))
    {
        // 拡大係数を算出する
        scale_factor.0 = (window_width / app_base_resolution.x)
            .min(window_height / app_base_resolution.y);

        info!("scale_factor.0: {}", scale_factor.0);

        // 現在の解像度を考慮しアスペクト比を変えないようにカメラにviewportを設定する
        set_fullscreen_viewport(
            query_target_camera.iter_mut().collect(),
            now_resolution,
            app_base_resolution.0,
            scale_factor.0,
        );

        // UIの拡大係数をセットする
        *ui_scale = UiScale(scale_factor.0);

        // フルスクリーンフラグを立てる
        fullscreen_state.0 = true;
    }
    else
    {
        // 対象のカメラのviewportを元に戻す。
        clear_viewport_override(query_target_camera.iter_mut().collect());

        // UIの拡大係数をリセットする
        *ui_scale = UiScale(1.0);

        // フルスクリーンフラグを倒す
        fullscreen_state.0 = false;
    }
}

////////////////////////////////////////////////////////////////////////////////

// 現在の解像度を考慮してアスペクト比を変えないようにカメラにviewportを設定する
fn set_fullscreen_viewport(
    target_cameras: Vec<(Mut<Camera>, Mut<Target>)>,
    now_resolution: Vec2,
    app_base_resolution: Vec2,
    scale_factor: f32,
)
{
    // 黒帯によるviewport左上隅の位置ずれを算出する
    let monitor_offset = (now_resolution - app_base_resolution * scale_factor) * 0.5;

    info!("monitor_offset: {}", monitor_offset);

    // 対象のカメラを全て処理する
    for (mut target_camera, mut backup_viewport) in target_cameras
    {
        info!("A: backup: {:?}", backup_viewport.0);
        info!("A: target: {:?}", target_camera.viewport);

        // 現在カメラにセットされているviewportを保存する
        backup_viewport.0 = target_camera.viewport.clone();

        // 現在カメラにセットされているviewportからオフセットとサイズを取り出す
        let (mut viewport_offset, mut viewport_size) = match &target_camera.viewport
        {
            Some(viewport_rect) => (
                viewport_rect.physical_position.as_vec2(),
                viewport_rect.physical_size.as_vec2(),
            ),
            None => (Vec2::ZERO, app_base_resolution),
        };

        // viewportを加工する
        viewport_offset *= scale_factor;
        viewport_size *= scale_factor;

        // viewportを変更する
        target_camera.viewport = Some(Viewport {
            physical_position: (viewport_offset + monitor_offset).as_uvec2(),
            physical_size: viewport_size.as_uvec2(),
            ..default()
        });

        info!("A: viewport: {:?}", target_camera.viewport);
    }
}

//------------------------------------------------------------------------------

// ウィンドウへ切り替える場合は、保存しておいたviewportを設定する
fn clear_viewport_override(target_cameras: Vec<(Mut<Camera>, Mut<Target>)>)
{
    for (mut target_camera, backup_viewport) in target_cameras
    {
        info!("B: backup: {:?}", backup_viewport.0);
        info!("B: target: {:?}", target_camera.viewport);

        // viewportを変更する
        target_camera.viewport = backup_viewport.0.clone();

        info!("B: viewport: {:?}", target_camera.viewport);
    }
}

////////////////////////////////////////////////////////////////////////////////

// 正射影カメラ(Camera2d)のスケーリングモードをFixedに設定する
#[allow(clippy::type_complexity)]
fn fix_camera2d_projection_mode(
    mut query_projection: Query<
        &mut Projection,
        (
            Added<Projection>,          // 新たに追加された直後のProjectionが対象
            With<Camera2d>,             // Camera2dが対象
            Without<IsDefaultUiCamera>, // Bevyが管理するUI用カメラは対象外
        ),
    >,
    app_base_resolution: Res<AppBaseResolution>,
)
{
    // 対象のProjectionを全て処理する
    for mut projection in query_projection.iter_mut()
    {
        // 正射影カメラ(Orthographic)なら
        if let Projection::Orthographic(orthographic_projection) = &mut *projection
        {
            orthographic_projection.scaling_mode = ScalingMode::Fixed {
                width: app_base_resolution.x,
                height: app_base_resolution.y,
            };
            info!("-----＜正射影カメラの追加を検知＞-----");
            info!(
                "orthographic_projection.scaling_mode: {:?}",
                orthographic_projection.scaling_mode
            );
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

// フルスクリーン中に生成されたカメラにviewportをセットする
#[allow(clippy::type_complexity)]
fn fit_new_camera_viewport_to_fullscreen(
    mut query_target_camera: Query<
        (&mut Camera, &mut Target),
        (
            Added<Camera>, // spawnされたばかりのCameraが対象
            With<Target>,  // フルスクリーン処理の対象であること
        ),
    >,
    window: Single<&Window>,
    app_base_resolution: Res<AppBaseResolution>,
    scale_factor: Res<ScaleFactor>,
    fullscreen_state: Res<FullscreenState>,
)
{
    // 追加されたカメラが存在し、且つ現在フルスクリーンなら
    if !query_target_camera.is_empty() && fullscreen_state.0
    {
        info!("-----＜フルスクリーン中のカメラ追加を検知＞-----");

        // 現在の解像度を取得
        let window_width = window.resolution.physical_width() as f32;
        let window_height = window.resolution.physical_height() as f32;
        let now_resolution = Vec2::new(window_width, window_height);

        // 解像度を考慮してviewportを設定する
        set_fullscreen_viewport(
            query_target_camera.iter_mut().collect(),
            now_resolution,
            app_base_resolution.0,
            scale_factor.0,
        );
    }
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
