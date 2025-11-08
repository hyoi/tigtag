#![allow(dead_code)]
use super::*;

////////////////////////////////////////////////////////////////////////////////

// 型エイリアス
pub type MainKeyAndModifiers = (KeyCode, Modifiers);
pub type Modifiers = Option<Vec<KeyCode>>;

////////////////////////////////////////////////////////////////////////////////

// 中身をIterで取得するトレイト
pub trait AppCtrl
{
    fn keys_iter(&self) -> Iter<'_, MainKeyAndModifiers>;
    fn buttons_iter(&self) -> Iter<'_, GamepadButton>;
}

////////////////////////////////////////////////////////////////////////////////

// 入力関係のSystemParamを作る
#[derive(SystemParam)]
pub struct InputDevicePack<'w, 's>
{
    input_keycode: ResMut<'w, ButtonInput<KeyCode>>,
    option_target_gamepad: Option<ResMut<'w, handle_input::TargetGamepad>>,
    query_gamepads: Query<'w, 's, &'static mut Gamepad>,
}

pub trait IsPressed<T: AppCtrl>
{
    fn is_pressed(&self, appctrl: &T) -> bool;
    fn is_pressed_with_reset(&mut self, appctrl: &T) -> bool;
}

impl<'w, 's, T> IsPressed<T> for InputDevicePack<'w, 's>
where
    T: AppCtrl,
{
    // キー入力とゲームパッドのボタン入力をチェックする
    fn is_pressed(&self, appctrl_setting: &T) -> bool
    {
        // キーが押下されているか
        let is_keys_pressed = appctrl_setting
            .keys_iter()
            .any(|appctrl_keys| self.input_keycode.is_just_pressed(appctrl_keys));

        // キーの押下がないなら
        let mut is_buttons_pressed = false;
        if !is_keys_pressed
            && let Some(target_gamepad) = &self.option_target_gamepad
            && let Some(gamepad_entity) = target_gamepad.entity()
            && let Ok(gamepad) = self.query_gamepads.get(gamepad_entity)
        {
            // ゲームパッドのボタンが押下されているか
            is_buttons_pressed = appctrl_setting
                .buttons_iter()
                .any(|&appctrl_btn| gamepad.just_pressed(appctrl_btn));
        }

        // 戻り値
        is_keys_pressed || is_buttons_pressed
    }

    // .is_pressed()の拡張版。戻り値は同じ。副作用で押下された入力をキャンセルする
    fn is_pressed_with_reset(&mut self, appctrl_setting: &T) -> bool
    {
        // キーが押下されているか
        let is_keys_pressed = appctrl_setting.keys_iter().any(|appctrl_keys| {
            let check = self.input_keycode.is_just_pressed(appctrl_keys);
            if check
            {
                // 押下された入力（主キーのみ）をリセットする
                let (main_key, _option_vec_mod) = appctrl_keys;
                let input_keycode = &mut self.input_keycode;
                input_keycode.reset(*main_key);
            }
            check
        });

        // ゲームパッドが接続されているなら
        let mut is_buttons_pressed = false;
        if let Some(target_gamepad) = &self.option_target_gamepad
            && let Some(gamepad_entity) = target_gamepad.entity()
            && let Ok(mut gamepad) = self.query_gamepads.get_mut(gamepad_entity)
        {
            // ゲームパッドのボタンが押下されているか
            is_buttons_pressed =
                appctrl_setting.buttons_iter().any(|&appctrl_btn| {
                    let check = gamepad.just_pressed(appctrl_btn);
                    if check
                    {
                        // 押下されたゲームパッドのボタンをリセットする
                        gamepad.digital_mut().reset(appctrl_btn);
                    }
                    check
                });
        }

        // 戻り値
        is_keys_pressed || is_buttons_pressed
    }
}

////////////////////////////////////////////////////////////////////////////////

// 主キー＋装飾キーの押下判定（ButtonInput<KeyCode> にトレイトを追加）
pub trait ButtonInputKeyCodeExt
{
    fn is_just_pressed(&self, keys: &MainKeyAndModifiers) -> bool;
}

impl ButtonInputKeyCodeExt for ButtonInput<KeyCode>
{
    // 主キーと装飾キーの組み合わせが押下されているか
    fn is_just_pressed(&self, appctrl_keys: &MainKeyAndModifiers) -> bool
    {
        let (main_key, option_modifiers) = appctrl_keys;

        // 主キーが押下されているか
        let is_main_key_pressed = self.just_pressed(*main_key);

        // 主キーが押下されているなら
        let mut is_modifiers_pressed = false;
        if is_main_key_pressed
        {
            // 修飾キー指定があれば押下状態を調べる
            is_modifiers_pressed = match option_modifiers
            {
                Some(keys) => keys.iter().any(|&key| self.pressed(key)),
                None => true, // 修飾キー指定がない場合
            };
        }

        // 戻り値
        is_main_key_pressed && is_modifiers_pressed
    }
}

////////////////////////////////////////////////////////////////////////////////

// アプリ終了のキーとボタンの設定
#[derive_appctrl_input]
pub struct ExitAppInput;

// アプリ終了
pub fn send_exit_app_message(
    appctrl: Local<ExitAppInput>, // 初回のみdefault()で初期化
    mut input_device: InputDevicePack,
    mut message_exit_app: MessageWriter<AppExit>,
) -> Result
{
    // アプリ終了キー・ボタンが押下されたなら
    if input_device.is_pressed_with_reset(&*appctrl)
    {
        message_exit_app.write(AppExit::Success); // アプリ終了メッセージを送信
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// 全画面切替のキーとボタンの設定
#[derive_appctrl_input]
pub struct FullScreenToggleInput;

// ウィンドウと全画面を切り替える(トグル動作)
pub fn toggle_fullscreen(
    appctrl: Local<FullScreenToggleInput>, // 初回のみdefault()で初期化
    mut input_device: InputDevicePack,
    mut window: Single<&mut Window>,
) -> Result
{
    // 切替キー・ボタンが押下されたなら
    if input_device.is_pressed_with_reset(&*appctrl)
    {
        use WindowMode::*;
        window.mode = match window.mode
        {
            // ウィンドウ => 全画面
            Windowed => BorderlessFullscreen(MonitorSelection::Current),
            // 全画面 => ウィンドウ
            _ => Windowed,
        };
    }

    Ok(())
}

//------------------------------------------------------------------------------

// 全画面時にCurrentモニターのスケールファクター他を保存するResource
#[derive(Resource, Default, Debug, PartialEq, Clone, Copy)]
pub struct ScaleFactor(pub Option<(f32, Vec2, Vec2)>); // default: ScaleFactor(None) -> Windowed

// 解像度変更を検知してスケールファクター他を計算し、変更とResource更新を行う
pub fn update_scale_factor(
    mut window: Single<&mut Window>,
    option_camera: Option<Single<&Camera, With<IsDefaultUiCamera>>>,
    option_scale_factor: Option<ResMut<ScaleFactor>>,
    mut local_resolution: Local<(u32, u32)>,
    mut local_logical_viewport: Local<Rect>,
) -> Result
{
    // 準備
    let mut res_scale_factor = option_scale_factor.ok_or("Resource not found.")?;

    // モニター解像度を取得する
    let reso_width = window.resolution.physical_width();
    let reso_height = window.resolution.physical_height();

    // モニター解像度が変化したなら（おそらくwindow.mode変更から1フレーム遅れる）
    if (reso_width, reso_height) != *local_resolution
    {
        // モニター解像度の変化検出用に現在の値を記録する
        *local_resolution = (reso_width, reso_height);

        // window.modeが全画面なら
        if matches!(window.mode, WindowMode::BorderlessFullscreen(_))
        {
            // モニター解像度とウィンドウサイズからスケールファクターを決める
            let scale_width = reso_width as f32 / SCREEN_PIXELS_WIDTH;
            let scale_height = reso_height as f32 / SCREEN_PIXELS_HEIGHT;
            let scale_facter = if scale_width < scale_height
            {
                scale_width
            }
            else
            {
                scale_height
            };

            // viewportの位置ずれ調整用の値を記録する
            let viewport_adjuster = Vec2::new(
                reso_width as f32 - SCREEN_PIXELS_WIDTH * scale_facter,
                reso_height as f32 - SCREEN_PIXELS_HEIGHT * scale_facter,
            ) * 0.5;

            // スケールファクターを設定しResourceを更新
            window.resolution.set_scale_factor(scale_facter);
            *res_scale_factor =
                ScaleFactor(Some((scale_facter, Vec2::ZERO, viewport_adjuster)));
        }
        else
        {
            // スケールファクターを解除しResourceを更新
            window.resolution.set_scale_factor(1.0);
            *res_scale_factor = ScaleFactor(None);
        }
    }

    // カメラのviewportが変化したなら（おそらくスケールファクター変更から1フレーム遅れる）
    if let Some(camera) = option_camera
        && let Some(logical_viewport_rect) = camera.logical_viewport_rect()
        && logical_viewport_rect != *local_logical_viewport
    {
        // viewportのサイズの変化検出用に現在の値を記録する
        *local_logical_viewport = logical_viewport_rect;

        // Resourceを更新
        if let ScaleFactor(Some((_, ui_adjuster, _))) = &mut *res_scale_factor
        {
            let x = (logical_viewport_rect.max.x - SCREEN_PIXELS_WIDTH) * 0.5;
            let y = (logical_viewport_rect.max.y - SCREEN_PIXELS_HEIGHT) * 0.5;
            *ui_adjuster = Vec2::new(x, y);
        }
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// UI outline表示切替のキーとボタンの設定
#[derive_appctrl_input]
pub struct UiOutlineToggleInput;

// UI outlineの表示／非表示を切り替える(トグル動作)
pub fn toggle_ui_outline_gizmo(
    appctrl: Local<UiOutlineToggleInput>, // 初回のみdefault()で初期化
    mut input_device: InputDevicePack,
    option_ui_debug_options: Option<ResMut<UiDebugOptions>>,
) -> Result
{
    // 準備
    let mut ui_debug_options =
        option_ui_debug_options.ok_or("ResMut<UiDebugOptions> not found.")?;

    // UI outline表示／非表示の切替キー・ボタンが押下されたなら
    if input_device.is_pressed_with_reset(&*appctrl)
    {
        ui_debug_options.toggle();
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
