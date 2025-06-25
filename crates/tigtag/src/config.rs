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

////////////////////////////////////////////////////////////////////////////////

// 事前ロード対象
pub const PRELOAD_ASSETS: &[&str] = &[
    // ASSETS_FONT_PRESSSTART2P_REGULAR,
    // ASSETS_FONT_ORBITRON_BLACK,
    // ASSETS_SPRITE_KANI_DOTOWN,
    // ASSETS_SPRITE_BRICK_WALL,
];

// assets（フォント）
// pub const ASSETS_FONT_PRESSSTART2P_REGULAR: &str = "font/PressStart2P-Regular.ttf";
// pub const ASSETS_FONT_ORBITRON_BLACK: &str = "font/Orbitron-Black.ttf";

// assets（スプライト）
// pub const ASSETS_SPRITE_KANI_DOTOWN: &str = "image/sprite/kani_DOTOWN.png";
// pub const ASSETS_SPRITE_BRICK_WALL: &str = "image/sprite/brick_wall.png";

////////////////////////////////////////////////////////////////////////////////

// カメラのマーカーComponent
#[derive(Component, Clone)]
pub struct SimpleCamera2d;
// #[derive(Component, Clone)]
// pub struct SimpleCamera3dOrbit;

pub static CAMERA_SETTINGS: LazyLock<Vec<simple_camera::Setting>> =
    LazyLock::new(|| {
        vec![
            simple_camera::Setting::from((
                1,              // カメラのレンダリング優先度（0が最後）
                Color::BLACK,    // レンダリング時の背景色（NONEは透明）
                SimpleCamera2d, // マーカーComponent
                Camera2d,       // カメラ種類Component
                Transform::from_translation(CAMERA2D_POSITION), // カメラの位置
            )),
            // simple_camera::Setting::from((
            //     0,
            //     Color::BLACK,
            //     SimpleCamera3dOrbit,
            //     Camera3d::default(),
            //     Transform::from_translation(CAMERA3D_POSITION_ORBIT.vec3())
            //         .looking_at(Vec3::ZERO, Vec3::Y),
            // )),
        ]
    });

// 2Dカメラの位置
// ※第四象限を利用する。左上隅が(0,0)で、X軸はプラス方向へ、Y軸はマイナス方向へ伸びる
pub const CAMERA2D_POSITION: Vec3 = Vec3::new(
    SCREEN_PIXELS_WIDTH * 0.5,
    SCREEN_PIXELS_HEIGHT * -0.5,
    999.0, /* 0.0だとスプライトの子のText2dがZ軸1.0(Vec3::Z)で表示されない不具合が発生(v0.14) */
);

// 3Dカメラの位置（極座標）
// pub const CAMERA3D_POSITION_ORBIT: Orbit = Orbit {
//     r: 8.0,
//     theta: PI * 0.6, // 1.0:天頂、0.5:真横、0.0:真下
//     phi: TAU * 0.9,  // 時計の6時方向が0.0で反時計回り
// };

////////////////////////////////////////////////////////////////////////////////

// コンパイル オプションの定数
pub const SPRITE_OFF: fn() -> bool = || cfg!(feature = "sprite_off");
pub const ATTACH_VIEWPORT: fn() -> bool = || cfg!(feature = "attach_viewport");

////////////////////////////////////////////////////////////////////////////////

// 3Dライトの設定
// pub const SIMPLE_LIGHT3D_BRIGHTNESS: f32 = 3000.0; // 明るさ
// pub const SIMPLE_LIGHT3D_POSITION: Vec3 = Vec3::new(1.0, 3.0, 2.0); // 位置

////////////////////////////////////////////////////////////////////////////////

// スプライト重なり
// pub const DEPTH_SPRITE_DEBUG_GRID: f32 = 999.0; // 重なりの最大値
// pub const DEPTH_SPRITE_KANI_DOTOWN: f32 = 900.0; // フッターの蟹アイコン

////////////////////////////////////////////////////////////////////////////////

// 極座標カメラを移動する場合の初期位置
// ※positionとlook_atがspawn済みCamera3dと同じ状態になる初期値を与えること
// pub static ORBIT_CAMERA_DEFAULT: LazyLock<orbit_camera::OrbitCamera> =
//     LazyLock::new(|| {
//         orbit_camera::OrbitCamera {
//             position: CAMERA3D_POSITION_ORBIT, // 極座標上のカメラの位置
//             look_at: (Vec3::ZERO, Vec3::Y),    // 注視点と視線を軸にしたロール
//             is_active: true,                   // カメラが有効か
//             clamp_r: ORBIT_R_MIN_MAX,          // 極座標のRのminとmax
//             clamp_theta: ORBIT_THETA_MIN_MAX,  // 極座標のθのminとmax
//             ..default()
//         }
//     });

// 極座標カメラを移動する場合の制限
// pub const ORBIT_R_MIN_MAX: (f32, f32) = (1.0, 30.0); // min, max
// pub const ORBIT_THETA_MIN_MAX: (f32, f32) = (PI * 0.51, PI * 0.99); // min, max

////////////////////////////////////////////////////////////////////////////////

// キーコードとコールバック関数の対応
// pub const KEY_MAP: [(KeyCode, orbit_camera::CallBack); 12] = [
//     // WASD
//     (KeyCode::KeyW, orbit_camera::move_up),
//     (KeyCode::KeyS, orbit_camera::move_down),
//     (KeyCode::KeyA, orbit_camera::move_right),
//     (KeyCode::KeyD, orbit_camera::move_left),
//     (KeyCode::KeyQ, orbit_camera::zoom_in),
//     (KeyCode::KeyE, orbit_camera::zoom_out),
//     // カーソルキー
//     (KeyCode::ArrowUp, orbit_camera::move_up),
//     (KeyCode::ArrowDown, orbit_camera::move_down),
//     (KeyCode::ArrowLeft, orbit_camera::move_right),
//     (KeyCode::ArrowRight, orbit_camera::move_left),
//     (KeyCode::PageUp, orbit_camera::zoom_in),
//     (KeyCode::PageDown, orbit_camera::zoom_out),
// ];

// ゲームパッドのボタン／スティックとコールバック関数の対応
// pub const PAD_MAP: [(GamepadInput, orbit_camera::CallBack); 10] = [
//     // 十字ボタン
//     (GamepadInput::Button(DPadUp), orbit_camera::move_up),
//     (GamepadInput::Button(DPadDown), orbit_camera::move_down),
//     (GamepadInput::Button(DPadLeft), orbit_camera::move_right),
//     (GamepadInput::Button(DPadRight), orbit_camera::move_left),
//     // トリガー
//     (GamepadInput::Button(LeftTrigger), orbit_camera::zoom_in),
//     (GamepadInput::Button(RightTrigger), orbit_camera::zoom_out),
//     (GamepadInput::Button(LeftTrigger2), orbit_camera::zoom_in),
//     (GamepadInput::Button(RightTrigger2), orbit_camera::zoom_out),
//     // 左スティック
//     (GamepadInput::Axis(LeftStickX), orbit_camera::axis_x_reverse),
//     (GamepadInput::Axis(LeftStickY), orbit_camera::axis_y_normal),
// ];

////////////////////////////////////////////////////////////////////////////////

// シンプル ヘッダー／フッター用情報
// pub const HEADER_FOOTER: &[header_footer::TextBlock] = &[
//     HEADER_DAY_TIME,     // 日時表示
//     HEADER_TITLE,        // タイトル
//     HEADER_ELAPSED_TIME, // 経過時間表示
//     FOOTER_FPS,          // FPS表示
//     FOOTER_AUTHER,       // auther
//     FOOTER_POWERED_BY,   // Powered by
// ];

// pub const HEADER_DAY_TIME: header_footer::TextBlock = header_footer::TextBlock {
//     position: header_footer::Position::TopLeft,
//     align_self: AlignSelf::Start,     // セル内の上寄せ
//     justify_self: JustifySelf::Start, // セル内の左寄せ
//     bg_color: css::BLUE,
//     spans: &[(
//         "",
//         ASSETS_FONT_PRESSSTART2P_REGULAR,
//         PIXELS_PER_GRID * 0.4,
//         css::SILVER,
//     )],
// };

// pub const HEADER_TITLE: header_footer::TextBlock = header_footer::TextBlock {
//     position: header_footer::Position::TopCenter,
//     align_self: AlignSelf::Start,      // セル内の上寄せ
//     justify_self: JustifySelf::Center, // セル内の中央寄せ
//     bg_color: css::BLUE,
//     spans: &[(
//         APP_TITLE,
//         ASSETS_FONT_ORBITRON_BLACK,
//         PIXELS_PER_GRID * 0.5,
//         css::TEAL,
//     )],
// };

// pub const HEADER_ELAPSED_TIME: header_footer::TextBlock = header_footer::TextBlock {
//     position: header_footer::Position::TopRight,
//     align_self: AlignSelf::Start,   // セル内の上寄せ
//     justify_self: JustifySelf::End, // セル内の右寄せ
//     bg_color: css::BLUE,
//     spans: &[(
//         "",
//         ASSETS_FONT_PRESSSTART2P_REGULAR,
//         PIXELS_PER_GRID * 0.4,
//         css::SILVER,
//     )],
// };

// pub const FOOTER_FPS: header_footer::TextBlock = header_footer::TextBlock {
//     position: header_footer::Position::BottomLeft,
//     align_self: AlignSelf::End,       // セル内の下寄せ
//     justify_self: JustifySelf::Start, // セル内の左寄せ
//     bg_color: Srgba::NONE,
//     spans: &[
//         (
//             " FPS ",
//             ASSETS_FONT_ORBITRON_BLACK,
//             PIXELS_PER_GRID * 0.5,
//             css::TEAL,
//         ),
//         (
//             "",
//             ASSETS_FONT_PRESSSTART2P_REGULAR,
//             PIXELS_PER_GRID * 0.4,
//             css::SILVER,
//         ),
//     ],
// };

// pub const FOOTER_AUTHER: header_footer::TextBlock = header_footer::TextBlock {
//     position: header_footer::Position::BottomCenter,
//     align_self: AlignSelf::End,        // セル内の下寄せ
//     justify_self: JustifySelf::Center, // セル内の中央寄せ
//     bg_color: Srgba::NONE,
//     spans: &[(
//         COPYRIGHT,
//         ASSETS_FONT_ORBITRON_BLACK,
//         PIXELS_PER_GRID * 0.5,
//         css::TEAL,
//     )],
// };

// pub const FOOTER_POWERED_BY: header_footer::TextBlock = header_footer::TextBlock {
//     position: header_footer::Position::BottomRight,
//     align_self: AlignSelf::End,     // セル内の下寄せ
//     justify_self: JustifySelf::End, // セル内の右寄せ
//     bg_color: Srgba::NONE,
//     spans: &[
//         (
//             "Powered by ",
//             ASSETS_FONT_ORBITRON_BLACK,
//             PIXELS_PER_GRID * 0.5,
//             css::TEAL,
//         ),
//         (
//             "RUST",
//             ASSETS_FONT_ORBITRON_BLACK,
//             PIXELS_PER_GRID * 0.5,
//             css::SILVER,
//         ),
//         (
//             " & ",
//             ASSETS_FONT_ORBITRON_BLACK,
//             PIXELS_PER_GRID * 0.5,
//             css::TEAL,
//         ),
//         (
//             "BEVY ",
//             ASSETS_FONT_ORBITRON_BLACK,
//             PIXELS_PER_GRID * 0.5,
//             css::SILVER,
//         ),
//     ],
// };

////////////////////////////////////////////////////////////////////////////////

// おまけ(蟹)
// pub const GRID_X_KANI: i32 = SCREEN_GRIDS_WIDTH - 4;
// pub const GRID_Y_KANI: i32 = SCREEN_GRIDS_HEIGHT - 1;
// pub const MAGNIFY_SPRITE_KANI: f32 = 0.9;
// pub const COLOR_SPRITE_KANI: Color = Color::srgba(1.0, 1.0, 1.0, 0.6);

////////////////////////////////////////////////////////////////////////////////

// End of code.
