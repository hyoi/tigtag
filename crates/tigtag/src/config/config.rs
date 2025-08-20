use super::*;

////////////////////////////////////////////////////////////////////////////////

// ウィンドウの定義
pub static MAIN_WINDOW: LazyLock<Option<Window>> = LazyLock::new(|| {
    let window = Window {
        resolution: SCREEN_PIXELS_RESO.into(), // ウィンドウのサイズ
        resizable: false,                      // リサイズ不可
        decorations: true,                     // タイトルバー表示
        title: format!("{APP_TITLE} v{APP_VER}"), // ウィンドウタイトル
        enabled_buttons: EnabledButtons {
            minimize: false, // 最小化ボタン非表示
            maximize: false, // 最大化ボタン非表示
            close: true,     // クローズボタン表示
        },
        fit_canvas_to_parent: true, // v0.13で廃止(#11057)、v0.14で復活(#11278)
        ..default()
    };
    Some(window)
});

// ウィンドウ縦横(Pixel)
pub const SCREEN_PIXELS_RESO: Vec2 =
    Vec2::new(SCREEN_PIXELS_WIDTH, SCREEN_PIXELS_HEIGHT);
pub const SCREEN_PIXELS_WIDTH: f32 = PIXELS_PER_GRID * SCREEN_GRIDS_WIDTH as f32;
pub const SCREEN_PIXELS_HEIGHT: f32 = PIXELS_PER_GRID * SCREEN_GRIDS_HEIGHT as f32;

// 単位Gridの縦横(Pixel)
pub const PIXELS_PER_GRID: f32 = BASE_PIXELS as f32 * BASE_SCALING;
const BASE_PIXELS: i32 = 8;
const BASE_SCALING: f32 = 4.0;

// ウィンドウ縦横(Grid)
pub const SCREEN_GRIDS_WIDTH: i32 = 25; // memo: 25 best 43
pub const SCREEN_GRIDS_HEIGHT: i32 = 19; // memo: 19 best 24

// アプリの情報
pub const APP_TITLE: &str = "TigTag"; // env!("CARGO_PKG_NAME");
pub const APP_VER: &str = env!("CARGO_PKG_VERSION");
pub const COPYRIGHT: &str = "hyoi 2021 - 2025";

////////////////////////////////////////////////////////////////////////////////

// ログレベル
pub const LOG_LV_DEV: &str = "warn,wgpu_hal=error"; // 開発中
pub const LOG_LV_REL: &str = "error"; // リリース

////////////////////////////////////////////////////////////////////////////////

// コンパイル オプションの定数
pub const SPRITE_OFF: fn() -> bool = || cfg!(feature = "sprite_off");
pub const ATTACH_VIEWPORT: fn() -> bool = || cfg!(feature = "attach_viewport");

////////////////////////////////////////////////////////////////////////////////

// アプリ終了キー
pub const EXIT_APP_KEY: KeyCode = KeyCode::Escape;

// フルスクリーンのキー
pub const FULL_SCREEN_KEY: KeyCode = KeyCode::Enter;
pub const FULL_SCREEN_MODIFIER_KEY: &[KeyCode] =
    &[KeyCode::AltRight, KeyCode::AltLeft];

// フルスクリーンのゲームパッドボタン
// pub const FULL_SCREEN_BUTTON: GamepadButton = GamepadButton::Start; //ps4[OPTIONS]

////////////////////////////////////////////////////////////////////////////////

// ゲームの状態
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Default, States, MyState)]
pub enum MyState
{
    #[default]
    LoadAssets,
    InitGame,
    TitleDemo,
    DemoLoop,
    StageStart,
    MainLoop,
    StageClear,
    GameOver,
    Pause,
}

// ゲームの状態の判定
#[allow(dead_code)]
impl MyState
{
    pub fn is_demoplay(&self) -> bool { self.is_titledemo() || self.is_demoloop() }
    pub fn is_playing(&self) -> bool
    {
        self.is_stagestart() || self.is_mainloop() || self.is_stageclear()
    }
}

////////////////////////////////////////////////////////////////////////////////

// 事前ロード対象
pub const PRELOAD_ASSETS: &[&str] = &[
    ASSETS_FONT_PRESSSTART2P_REGULAR,
    ASSETS_FONT_ORBITRON_BLACK,
    ASSETS_SPRITE_KANI_DOTOWN,
    //============================
    ASSETS_SPRITE_BRICK_WALL,
    ASSETS_SPRITE_SHEET_PLAYER,
    ASSETS_SPRITE_SHEET_CHASER_RED,
    ASSETS_SPRITE_SHEET_CHASER_GREEN,
    ASSETS_SPRITE_SHEET_CHASER_BLUE,
    ASSETS_SPRITE_SHEET_CHASER_PINK,
    ASSETS_FONT_REGGAEONE_REGULAR,
    // ASSETS_SOUND_BEEP,
    //============================
];

// assets（フォント）
pub const ASSETS_FONT_PRESSSTART2P_REGULAR: &str = "font/PressStart2P-Regular.ttf";
pub const ASSETS_FONT_ORBITRON_BLACK: &str = "font/Orbitron-Black.ttf";
//==============================================================================
pub const ASSETS_FONT_REGGAEONE_REGULAR: &str = "font/ReggaeOne-Regular.ttf";
//==============================================================================

// assets（スプライト）
pub const ASSETS_SPRITE_KANI_DOTOWN: &str = "image/sprite/kani_DOTOWN.png";
//==============================================================================
pub const ASSETS_SPRITE_BRICK_WALL: &str = "image/sprite/brick_wall.png";

// assets（スプライトシート）
pub const ASSETS_SPRITE_SHEET_PLAYER: &str = "image/sprite_sheet/player.png";
pub const ASSETS_SPRITE_SHEET_CHASER_RED: &str = "image/sprite_sheet/chaser_red.png";
pub const ASSETS_SPRITE_SHEET_CHASER_GREEN: &str =
    "image/sprite_sheet/chaser_green.png";
pub const ASSETS_SPRITE_SHEET_CHASER_BLUE: &str =
    "image/sprite_sheet/chaser_blue.png";
pub const ASSETS_SPRITE_SHEET_CHASER_PINK: &str =
    "image/sprite_sheet/chaser_pink.png";

// assets（サウンド）
// pub const ASSETS_SOUND_BEEP: &str = "audio/sound/beep.ogg";
//==============================================================================

////////////////////////////////////////////////////////////////////////////////

// カメラのマーカーComponent
#[derive(Component, Clone)]
pub struct SimpleCamera2d;

pub static CAMERA_SETTINGS: LazyLock<Vec<simple_camera::Setting>> =
    LazyLock::new(|| {
        vec![simple_camera::Setting::from((
            1,              // カメラのレンダリング優先度（0が最後）
            Color::BLACK,   // レンダリング時の背景色（NONEは透明）
            SimpleCamera2d, // マーカーComponent
            Camera2d,       // カメラ種類Component
            Transform::from_translation(CAMERA2D_POSITION), // カメラの位置
        ))]
    });

// 2Dカメラの位置
// ※第四象限を利用する。左上隅が(0,0)で、X軸はプラス方向へ、Y軸はマイナス方向へ伸びる
pub const CAMERA2D_POSITION: Vec3 = Vec3::new(
    SCREEN_PIXELS_WIDTH * 0.5,
    SCREEN_PIXELS_HEIGHT * -0.5,
    999.0, /* 0.0だとスプライトの子のText2dがZ軸1.0(Vec3::Z)で表示されない不具合が発生(v0.14) */
);

////////////////////////////////////////////////////////////////////////////////

// スプライト重なり
pub const DEPTH_SPRITE_KANI_DOTOWN: f32 = 900.0; // フッターの蟹アイコン

//==============================================================================

pub const DEPTH_SPRITE_CHASER: f32 = 700.0; // チェイサーのスプライト
pub const DEPTH_SPRITE_PLAYER: f32 = 600.0; // プレイヤーのスプライト
pub const DEPTH_SPRITE_DOT: f32 = 500.0; // ドットスプライト
pub const DEPTH_SPRITE_BRICK_WALL: f32 = 400.0; // 壁スプライト

//==============================================================================

////////////////////////////////////////////////////////////////////////////////

// おまけ(蟹)
pub const SPRITE_KANI_GRID_X: i32 = SCREEN_GRIDS_WIDTH - 4;
pub const SPRITE_KANI_GRID_Y: i32 = SCREEN_GRIDS_HEIGHT - 1;
pub const SPRITE_KANI_MAGNIFY: f32 = 0.9;
pub const SPRITE_KANI_COLOR: Color = Color::srgba(1.0, 1.0, 1.0, 0.6);

//==============================================================================

// ドットのスプライトの情報
pub const SPRITE_DOT_RADIUS: f32 = PIXELS_PER_GRID * 0.08;
pub const SPRITE_DOT_COLOR: Color = Color::srgb(1.0, 1.0, 0.7);

//==============================================================================

////////////////////////////////////////////////////////////////////////////////

// キーコードとコールバック関数の対応
#[rustfmt::skip]
pub const KEY_MAP: [(KeyCode, player::CallBack); 8] = [
    // WASD
    ( KeyCode::KeyW, player::move_up    ),
    ( KeyCode::KeyS, player::move_down  ),
    ( KeyCode::KeyA, player::move_left  ),
    ( KeyCode::KeyD, player::move_right ),
    // カーソルキー
    ( KeyCode::ArrowUp   , player::move_up    ),
    ( KeyCode::ArrowDown , player::move_down  ),
    ( KeyCode::ArrowLeft , player::move_left  ),
    ( KeyCode::ArrowRight, player::move_right ),
];

// ゲームパッドのボタン／スティックとコールバック関数の対応
#[rustfmt::skip]
pub const PAD_MAP: [(GamepadInput, player::CallBack); 6] = [
    // 十字ボタン
    ( GamepadInput::Button( GamepadButton::DPadUp   ), player::move_up    ),
    ( GamepadInput::Button( GamepadButton::DPadDown ), player::move_down  ),
    ( GamepadInput::Button( GamepadButton::DPadLeft ), player::move_left  ),
    ( GamepadInput::Button( GamepadButton::DPadRight), player::move_right ),
    // 左スティック
    ( GamepadInput::Axis( GamepadAxis::LeftStickX ), player::axis_x_normal ),
    ( GamepadInput::Axis( GamepadAxis::LeftStickY ), player::axis_y_normal ),
];

////////////////////////////////////////////////////////////////////////////////

// GIZMOの表示／非表示を切替えるキー
pub const SHOW_HIDE_GIZMO_TOGGLE_KEY: KeyCode = KeyCode::Tab;

////////////////////////////////////////////////////////////////////////////////

// End of code.
