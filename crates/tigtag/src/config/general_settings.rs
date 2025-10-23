use super::*;

////////////////////////////////////////////////////////////////////////////////

// 単位Gridの縦横(Pixel)
pub const PIXELS_PER_GRID: f32 = BASE_PIXELS as f32 * BASE_SCALING;
const BASE_PIXELS: i32 = 8;
const BASE_SCALING: f32 = 4.0;

// ウィンドウ縦横(Grid)
pub const SCREEN_GRIDS_WIDTH: i32 = 25; // memo: 25 best 43
pub const SCREEN_GRIDS_HEIGHT: i32 = 19; // memo: 19 best 24

// ウィンドウ縦横(Pixel)
pub const SCREEN_PIXELS_RESO: UVec2 =
    UVec2::new(SCREEN_PIXELS_WIDTH as u32, SCREEN_PIXELS_HEIGHT as u32);
pub const SCREEN_PIXELS_WIDTH: f32 = PIXELS_PER_GRID * SCREEN_GRIDS_WIDTH as f32;
pub const SCREEN_PIXELS_HEIGHT: f32 = PIXELS_PER_GRID * SCREEN_GRIDS_HEIGHT as f32;

// アプリの情報
pub const APP_TITLE: &str = "TigTag"; // env!("CARGO_PKG_NAME");
pub const APP_VER: &str = env!("CARGO_PKG_VERSION");
pub const COPYRIGHT: &str = "hyoi 2021 - 2025";

//ウィンドウの定義
pub static MAIN_WINDOW: LazyLock<Window> = LazyLock::new(|| {
    Window {
        resolution: SCREEN_PIXELS_RESO.into(), // ウィンドウのサイズ
        resizable: false,                      // リサイズ不可
        decorations: true,                     // タイトルバー表示
        title: format!("{APP_TITLE} v{APP_VER}"), // タイトルバーに表示するタイトル
        enabled_buttons: EnabledButtons {
            minimize: false, // 最小化ボタン非表示
            maximize: false, // 最大化ボタン非表示
            close: true,     // クローズボタン表示
        },
        // fit_canvas_to_parent: true, // v0.13で廃止(#11057)、v0.14で復活(#11278)
        ..default()
    }
});

////////////////////////////////////////////////////////////////////////////////

// ログフィルター
pub const LOG_FILTER_DEVELOP: &str = "warn,wgpu_hal=error";
pub const LOG_FILTER_RELEASE: &str = "error";

////////////////////////////////////////////////////////////////////////////////

// ゲームの状態
#[rustfmt::skip]
#[allow(dead_code)]
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Default, States, MyState)]
pub enum MyState
{
    #[default] LoadAssets,
    Initialize,
    TitleDemo, DemoLoop,
    StageStart, MainLoop, StageClear, GameOver,
    Pause,
}

// ゲームの状態の判定
#[rustfmt::skip]
#[allow(dead_code)]
impl MyState
{
    pub fn is_demoplay(&self) -> bool { self.is_titledemo() || self.is_demoloop() }
    pub fn is_playing(&self) -> bool { self.is_stagestart() || self.is_mainloop() || self.is_stageclear() }
}

////////////////////////////////////////////////////////////////////////////////

// アプリ終了のキーとボタンの設定
impl Default for appctrl_input::ExitAppInput
{
    fn default() -> Self
    {
        Self {
            keys: vec![
                (KeyCode::Escape, None),
                (KeyCode::F4, Some(Vec::from(MODIFIERS_ALT))),
            ],
            buttons: vec![
                GamepadButton::Mode, //ps4[PSボタン]
            ],
        }
    }
}

//------------------------------------------------------------------------------

// 全画面切替のキーとボタンの設定
impl Default for appctrl_input::FullScreenToggleInput
{
    fn default() -> Self
    {
        Self {
            keys: vec![
                (KeyCode::Enter, Some(Vec::from(MODIFIERS_ALT))),
                (KeyCode::F11, None),
            ],
            buttons: vec![
                GamepadButton::Select, //ps4[SHARE]
            ],
        }
    }
}

//------------------------------------------------------------------------------

// UI outlineの表示／非表示を切替えるキー
impl Default for appctrl_input::UiOutlineToggleInput
{
    fn default() -> Self
    {
        Self {
            keys: vec![(KeyCode::Tab, Some(Vec::from(MODIFIERS_CTRL)))],
            buttons: vec![],
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

// 事前ロード対象
pub const PRELOAD_ASSETS: &[&str] = &[
    ASSETS_FONT_ORBITRON_BLACK,
    ASSETS_FONT_PRESSSTART2P_REGULAR,
    ASSETS_FONT_REGGAEONE_REGULAR,
    ASSETS_SPRITE_KANI_DOTOWN,
    ASSETS_SPRITE_BRICK_WALL,
    ASSETS_SPRITESHEET_PLAYER,
    ASSETS_SPRITESHEET_CHASER_RED,
    ASSETS_SPRITESHEET_CHASER_GREEN,
    ASSETS_SPRITESHEET_CHASER_BLUE,
    ASSETS_SPRITESHEET_CHASER_PINK,
    ASSETS_SOUND_BEEP,
];

// assets（フォント）
pub const ASSETS_FONT_ORBITRON_BLACK: &str = "font/Orbitron-Black.ttf";
pub const ASSETS_FONT_PRESSSTART2P_REGULAR: &str = "font/PressStart2P-Regular.ttf";
pub const ASSETS_FONT_REGGAEONE_REGULAR: &str = "font/ReggaeOne-Regular.ttf";

// assets（スプライト）
pub const ASSETS_SPRITE_KANI_DOTOWN: &str = "sprite/kani_DOTOWN.png";
pub const ASSETS_SPRITE_BRICK_WALL: &str = "sprite/brick_wall.png";

// assets（スプライトシート）
pub const ASSETS_SPRITESHEET_PLAYER: &str = "spritesheet/player.png";
pub const ASSETS_SPRITESHEET_CHASER_RED: &str = "spritesheet/chaser_red.png";
pub const ASSETS_SPRITESHEET_CHASER_GREEN: &str = "spritesheet/chaser_green.png";
pub const ASSETS_SPRITESHEET_CHASER_BLUE: &str = "spritesheet/chaser_blue.png";
pub const ASSETS_SPRITESHEET_CHASER_PINK: &str = "spritesheet/chaser_pink.png";

// assets（サウンド）
pub const ASSETS_SOUND_BEEP: &str = "audio/sound/beep.ogg";
pub const VOLUME_SOUND_BEEP: Volume = Volume::Linear(0.1); //SEボリューム

////////////////////////////////////////////////////////////////////////////////

// カメラの情報を格納するResourceの定義
#[derive(Resource, Deref, DerefMut)]
pub struct CameraSettings(pub Vec<simple_camera::Setting>);

// カメラのComponent
#[derive(Component, Clone)]
pub struct SimpleCamera2d;

// 2Dカメラの位置
// 第四象限を利用する。左上隅が(0,0)で、X軸はプラス方向へ、Y軸はマイナス方向へ伸びる
pub const CAMERA2D_POSITION: Vec3 = Vec3::new(
    SCREEN_PIXELS_WIDTH * 0.5,
    SCREEN_PIXELS_HEIGHT * -0.5,
    999.0, // 0.0だとスプライトの子のText2dがZ軸1.0(Vec3::Z)で表示されない不具合が発生(v0.14)
);

// カメラ情報の初期化
impl Default for CameraSettings
{
    fn default() -> Self
    {
        Self(vec![simple_camera::Setting::from((
            1,              // カメラのレンダリング優先度（0が最後）
            Color::BLACK,   // レンダリング時の背景色（NONEは透明）
            SimpleCamera2d, // マーカー（Component）
            Camera2d,       // カメラ種類（Component）
            Transform::from_translation(CAMERA2D_POSITION), // カメラの位置
        ))])
    }
}

////////////////////////////////////////////////////////////////////////////////

// コンパイル オプションの定数
pub const ATTACH_VIEWPORT: fn() -> bool = || cfg!(feature = "attach_viewport");
pub const SPRITE_OFF: fn() -> bool = || cfg!(feature = "sprite_off");

////////////////////////////////////////////////////////////////////////////////

// スプライト重なり
pub const DEPTH_SPRITE_KANI_DOTOWN: f32 = 900.0; // フッターの蟹アイコン
pub const DEPTH_SPRITE_CHASER: f32 = 700.0; // チェイサーのスプライト
pub const DEPTH_SPRITE_PLAYER: f32 = 600.0; // プレイヤーのスプライト
pub const DEPTH_SPRITE_DOT: f32 = 500.0; // ドットスプライト
pub const DEPTH_SPRITE_BRICK_WALL: f32 = 400.0; // 壁スプライト

////////////////////////////////////////////////////////////////////////////////

// Hit ANY Keyの処理で無視するキーとボタン
#[rustfmt::skip]
pub const IGNORE_KEYS_HITANYKEY: &[KeyCode] = &[
    KeyCode::AltLeft    , KeyCode::AltRight,
    KeyCode::ControlLeft, KeyCode::ControlRight,
    KeyCode::ShiftLeft  , KeyCode::ShiftRight,
    KeyCode::SuperLeft  , KeyCode::SuperRight,
    KeyCode::ArrowUp    , KeyCode::ArrowDown,
    KeyCode::ArrowRight , KeyCode::ArrowLeft,
    KeyCode::Fn,
    KeyCode::Unidentified(NativeKeyCode::Windows(57443)), //ThinkPad [Fn]
];
#[rustfmt::skip]
pub const IGNORE_BUTTONS_HITANYKEY: &[GamepadButton] = &[
    GamepadButton::Select, //ps4[SHARE]
    GamepadButton::Start,  //ps4[OPTIONS]
    GamepadButton::Mode,   //ps4[PSボタン]
];

// .init_resource()用default
impl Default for misc::MaskHitAnyKeyInput
{
    fn default() -> Self
    {
        Self {
            keys: HashSet::from_iter(IGNORE_KEYS_HITANYKEY.iter()),
            buttons: HashSet::from_iter(IGNORE_BUTTONS_HITANYKEY.iter()),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

// ドットのスプライトの情報
pub const SPRITE_DOT_RADIUS: f32 = PIXELS_PER_GRID * 0.08;
pub const SPRITE_DOT_COLOR: Color = Color::srgb(1.0, 1.0, 0.7);

////////////////////////////////////////////////////////////////////////////////

// キーコードとアクションの対応
#[rustfmt::skip]
pub const KEYBOARD_MAP: handle_input::ConnfigKeyboard = &[
    // WASD
    ( KeyCode::KeyW, handle_input::UserAction::MoveUp    ),
    ( KeyCode::KeyS, handle_input::UserAction::MoveDown  ),
    ( KeyCode::KeyA, handle_input::UserAction::MoveLeft  ),
    ( KeyCode::KeyD, handle_input::UserAction::MoveRight ),
    // カーソルキー
    ( KeyCode::ArrowUp   , handle_input::UserAction::MoveUp    ),
    ( KeyCode::ArrowDown , handle_input::UserAction::MoveDown  ),
    ( KeyCode::ArrowLeft , handle_input::UserAction::MoveLeft  ),
    ( KeyCode::ArrowRight, handle_input::UserAction::MoveRight ),
];

// ゲームパッドのボタン／スティックとアクションの対応
#[rustfmt::skip]
pub const GAMEPAD_MAP: handle_input::ConnfigGamepad = &[
    // 十字ボタン
    ( GAMEPAD_UP   , handle_input::UserAction::MoveUp    ),
    ( GAMEPAD_DOWN , handle_input::UserAction::MoveDown  ),
    ( GAMEPAD_LEFT , handle_input::UserAction::MoveLeft  ),
    ( GAMEPAD_RIGHT, handle_input::UserAction::MoveRight ),
    // 左スティック
    ( GAMEPAD_STICK_LEFT_Y, handle_input::UserAction::AxisVertNormal  ( 1.0 ) ),
    ( GAMEPAD_STICK_LEFT_X, handle_input::UserAction::AxisHorizNormal ( 1.0 ) ),
];

////////////////////////////////////////////////////////////////////////////////

// アジャスタ（マップ座標から画面座標への変換調整値）
pub const ADJUST_MAP_ON_SCREEN: IVec2 = IVec2::new(0, 1);

////////////////////////////////////////////////////////////////////////////////

// End of code.
