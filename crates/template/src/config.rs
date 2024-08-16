use super::*;

////////////////////////////////////////////////////////////////////////////////

//アプリの情報
pub const APP_TITLE : &str = "TigTag";       //アプリタイトル
pub const APP_VER   : &str = CARGO_TOML_VER; //アプリのバージョン
const CARGO_TOML_VER: &str = env!( "CARGO_PKG_VERSION" );

//単位Gridの縦横(Pixel)
const BASE_PIXELS : i32 = 8;
const BASE_SCALING: f32 = 4.0;
pub const PIXELS_PER_GRID: f32 = BASE_PIXELS as f32 * BASE_SCALING;

//ウィンドウ縦横(Grid)
pub const SCREEN_GRIDS_WIDTH : i32 = 25; //memo: 25 best 43
pub const SCREEN_GRIDS_HEIGHT: i32 = 19; //memo: 19 best 24

//ウィンドウ縦横(Pixel)
pub const SCREEN_PIXELS_WIDTH : f32 = PIXELS_PER_GRID * SCREEN_GRIDS_WIDTH  as f32;
pub const SCREEN_PIXELS_HEIGHT: f32 = PIXELS_PER_GRID * SCREEN_GRIDS_HEIGHT as f32;

//ウィンドウの定義
pub static MAIN_WINDOW: LazyLock<Option<Window>> = LazyLock::new
(   ||
    {   let window = Window
        {   title: format!( "{APP_TITLE} v{APP_VER}" ),
            resolution: ( SCREEN_PIXELS_WIDTH, SCREEN_PIXELS_HEIGHT ).into(),
            resizable: false,
            enabled_buttons: bevy::window::EnabledButtons
            {   minimize: false,
                maximize: false,
                close   : true,
            },
            // fit_canvas_to_parent: true, //v0.13で廃止(#11057)、v0.14で復活(#11278)
            ..default()
        };
        Some ( window )
    }
);

////////////////////////////////////////////////////////////////////////////////

//ログレベル
pub const LOG_LEVEL_DEV: &str = "warn,wgpu_hal=error"; //開発
pub const LOG_LEVEL_REL: &str = "error"; //リリース

////////////////////////////////////////////////////////////////////////////////

//カメラのレンダリングの重なり
pub const CAMERA_ORDER_DEFAULT_2D: isize = 1; //2D デフォルトカメラ
pub const CAMERA_ORDER_DEFAULT_3D: isize = 0; //3D デフォルトカメラが最下

//カメラの背景色
const CAMERA_BG_THROUGH: ClearColorConfig = ClearColorConfig::None;
const CAMERA_BG_COLOR  : ClearColorConfig = ClearColorConfig::Custom( Color::BLACK );

pub const CAMERA_BGCOLOR_2D: ClearColorConfig = CAMERA_BG_THROUGH;
pub const CAMERA_BGCOLOR_3D: ClearColorConfig = CAMERA_BG_COLOR;

//3Dライトの設定
pub const LIGHT_3D_BRIGHTNESS : f32  = 3000.0; //明るさ
pub const LIGHT_3D_TRANSLATION: Vec3 = Vec3::new( 30.0, 100.0, 40.0 ); //位置

//デフォルト2Dカメラの位置
//第四象限。左上隅が(0,0)で、X軸はプラス方向へ、Y軸はマイナス方向へ伸びる
pub const CAMERA_POSITION_DEFAULT_2D: Vec3 = Vec3::new
(   SCREEN_PIXELS_WIDTH  *  0.5,
    SCREEN_PIXELS_HEIGHT * -0.5,
    0.0
);

//極座標カメラの設定
pub const CAMERA_ORBIT_INIT_R    : f32 = 6.0;       //初期値
pub const CAMERA_ORBIT_INIT_THETA: f32 = PI  * 0.6; //初期値(ラジアン) 1.0:天頂、0.5:真横、0.0:真下
pub const CAMERA_ORBIT_INIT_PHI  : f32 = TAU * 0.9; //初期値(ラジアン) 6時方向が0.0で反時計回り

pub const CAMERA_ORBIT_MAX_R    : f32 = 50.0;      //rの最大値
pub const CAMERA_ORBIT_MIN_R    : f32 = 1.0;       //rの最小値
pub const CAMERA_ORBIT_MAX_THETA: f32 = PI * 0.99; //Θの最大値(ラジアン)
pub const CAMERA_ORBIT_MIN_THETA: f32 = PI * 0.51; //Θの最小値(ラジアン)

////////////////////////////////////////////////////////////////////////////////

//Gridに関連する定数
pub const GRID_CUSTOM_SIZE: Vec2 = Vec2::new( PIXELS_PER_GRID, PIXELS_PER_GRID );
pub const GRIDS_X_RANGE: Range<i32> = 0..SCREEN_GRIDS_WIDTH;
pub const GRIDS_Y_RANGE: Range<i32> = 0..SCREEN_GRIDS_HEIGHT;

////////////////////////////////////////////////////////////////////////////////

//スプライト重なり
pub const DEPTH_SPRITE_DEBUG_GRID : f32 = 999.0; //重なりの最大値
pub const DEPTH_SPRITE_LOADING_MSG: f32 = 950.0; //Now Loadingアニメのスプライト
pub const DEPTH_SPRITE_KANI_DOTOWN: f32 = 900.0; //フッターの蟹アイコン
//==============================================================================
pub const DEPTH_SPRITE_CHASER     : f32 = 700.0; //ゲームの敵機スプライト
pub const DEPTH_SPRITE_PLAYER     : f32 = 600.0; //ゲームの自機スプライト
pub const DEPTH_SPRITE_DOT        : f32 = 500.0; //ゲームのドットスプライト
pub const DEPTH_SPRITE_BRICK_WALL : f32 = 400.0; //ゲームの壁スプライト
//==============================================================================

////////////////////////////////////////////////////////////////////////////////

//assets（スプライト）
pub const ASSETS_SPRITE_KANI_DOTOWN: &str = "image/sprite/kani_DOTOWN.png";
pub const ASSETS_SPRITE_BRICK_WALL : &str = "image/sprite/brick_wall.png";

//==============================================================================
//assets（スプライトシート）
pub const ASSETS_SPRITE_SHEET_PLAYER      : &str = "image/sprite_sheet/player.png";
pub const ASSETS_SPRITE_SHEET_CHASER_RED  : &str = "image/sprite_sheet/chaser_red.png";
pub const ASSETS_SPRITE_SHEET_CHASER_GREEN: &str = "image/sprite_sheet/chaser_green.png";
pub const ASSETS_SPRITE_SHEET_CHASER_BLUE : &str = "image/sprite_sheet/chaser_blue.png";
pub const ASSETS_SPRITE_SHEET_CHASER_PINK : &str = "image/sprite_sheet/chaser_pink.png";
//==============================================================================

//assets（フォント）
pub const ASSETS_FONT_ORBITRON_BLACK      : &str = "font/Orbitron-Black.ttf";
pub const ASSETS_FONT_PRESSSTART2P_REGULAR: &str = "font/PressStart2P-Regular.ttf";
//==============================================================================
pub const ASSETS_FONT_REGGAEONE_REGULAR   : &str = "font/ReggaeOne-Regular.ttf";

//assets（サウンド）
pub const ASSETS_SOUND_BEEP: &str = "audio/sound/beep.ogg";
//==============================================================================

//事前ロード対象
pub const PRELOAD_ASSETS: &[ &str ] =
&[  ASSETS_SPRITE_KANI_DOTOWN,
    ASSETS_SPRITE_BRICK_WALL,
    ASSETS_FONT_ORBITRON_BLACK,
    ASSETS_FONT_PRESSSTART2P_REGULAR,
    //=================================
    ASSETS_SPRITE_SHEET_PLAYER,
    ASSETS_SPRITE_SHEET_CHASER_RED,
    ASSETS_SPRITE_SHEET_CHASER_GREEN,
    ASSETS_SPRITE_SHEET_CHASER_BLUE,
    ASSETS_SPRITE_SHEET_CHASER_PINK,
    ASSETS_FONT_REGGAEONE_REGULAR,
    ASSETS_SOUND_BEEP,
    //=================================
];

////////////////////////////////////////////////////////////////////////////////

//フルスクリーンのキー
pub const FULL_SCREEN_KEY: KeyCode = KeyCode::Enter;
pub const FULL_SCREEN_KEY_MODIFIER: &[ KeyCode ] = &[ KeyCode::AltRight, KeyCode::AltLeft ];

//フルスクリーンのゲームパッドボタン
pub const FULL_SCREEN_BUTTON: GamepadButtonType = GamepadButtonType::Start; //ps4[OPTIONS]

////////////////////////////////////////////////////////////////////////////////

//End of code.