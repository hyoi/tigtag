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

// ウィンドウとフルスクリーンを切り替える(トグル動作)
pub fn toggle_fullscreen(
    appctrl: Local<FullScreenToggleInput>, // 初回のみdefault()で初期化
    mut input_device: InputDevicePack,
    mut query_window: Query<&mut Window>,
) -> Result
{
    // 準備
    let mut window = query_window.single_mut()?;

    // ウィンドウ／フルスクリーンの切替キー・ボタンが押下されたなら
    if input_device.is_pressed_with_reset(&*appctrl)
    {
        match window.mode
        {
            WindowMode::Windowed =>
            {
                window.resolution.set_scale_factor(2.0);
                window.mode = WindowMode::Fullscreen(
                    MonitorSelection::Primary,
                    VideoModeSelection::Current,
                );
            }
            _ =>
            {
                window.resolution.set_scale_factor(1.0);
                window.mode = WindowMode::Windowed;
            }
        };

        #[cfg(debug_assertions)]
        dbg!(&window.mode, &window.resolution);
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
