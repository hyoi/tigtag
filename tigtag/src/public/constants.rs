use super::*;

pub const APP_TITLE: &str = "TigTag";                           //アプリタイトル
pub const CARGO_VER: &str = env!( "CARGO_PKG_VERSION" );        //cargo.ttomlの[package]version

pub const SCREEN_GRIDS_WIDTH : i32 = 25; //21,27,33,43          //ウィンドウ横幅(Grid)
pub const SCREEN_GRIDS_HEIGHT: i32 = 19; //16,20,25,32          //ウインドウ縦幅(Grid)

pub const MAP_GRIDS_WIDTH : i32 = SCREEN_GRIDS_WIDTH;           //マップ横幅(Grid)
pub const MAP_GRIDS_HEIGHT: i32 = SCREEN_GRIDS_HEIGHT - 2;      //マップ縦幅(Grid)

pub static MAP_PIXELS_DISPLAY_OFFSET: Lazy<Pixel> = Lazy::new   //マップ表示の左上隅X,Y座標(Pixel)
(   || Pixel::new( 0.0, -1.0 ) * PIXELS_PER_GRID                //　Pixel::new()がconst fnではないのでLazyに頼った
);

counted_array!
(   pub const DESIGN_GAME_FRAME: [ &str; _ ] = //画面デザイン(枠)
    //   0123456789 123456789 123456789
    [   "                         ", //0----
        "#########################", //1
        "#                       #", //2
        "#                       #", //3
        "#                       #", //4
        "#                       #", //5
        "#                       #", //6
        "#                       #", //7
        "#                       #", //8
        "#                       #", //9
        "#                       #", //10---
        "#                       #", //11
        "#                       #", //12
        "#                       #", //13
        "#                       #", //14
        "#                       #", //15
        "#                       #", //16
        "#########################", //17
        "                         ", //18
    ]  //0123456789 123456789 123456789
);

const SCREEN_SCALING      : f32 = 4.0;
const BASE_PIXELS_PER_GRID: i32 = 8;
pub const PIXELS_PER_GRID : f32 = BASE_PIXELS_PER_GRID as f32 * SCREEN_SCALING;     //1GridあたりのPixel数

pub const SCREEN_PIXELS_WIDTH : f32 = SCREEN_GRIDS_WIDTH  as f32 * PIXELS_PER_GRID; //ウィンドウ横幅(Pixel)
pub const SCREEN_PIXELS_HEIGHT: f32 = SCREEN_GRIDS_HEIGHT as f32 * PIXELS_PER_GRID; //ウィンドウ縦幅(Pixel)

pub const SCREEN_BACKGROUND_COLOR: Color = Color::rgb( 0.13, 0.13, 0.18 );          //ウィンドウ背景色

////////////////////////////////////////////////////////////////////////////////////////////////////

use std::ops::Range;
pub const SCREEN_GRIDS_RANGE_X: Range<i32> = 0..SCREEN_GRIDS_WIDTH;     //ウィンドウ横幅(Grid)
pub const SCREEN_GRIDS_RANGE_Y: Range<i32> = 0..SCREEN_GRIDS_HEIGHT;    //ウィンドウ縦幅(Grid)

pub const MAP_GRIDS_RANGE_X: Range<i32> = 0..MAP_GRIDS_WIDTH;           //マップ横幅(Grid)
pub const MAP_GRIDS_RANGE_Y: Range<i32> = 0..MAP_GRIDS_HEIGHT;          //マップ縦幅(Grid)

////////////////////////////////////////////////////////////////////////////////////////////////////

pub const ASSETS_FONT_ORBITRON_BLACK      : &str = "fonts/Orbitron-Black.ttf";       //フォント
pub const ASSETS_FONT_REGGAEONE_REGULAR   : &str = "fonts/ReggaeOne-Regular.ttf";    //フォント
pub const ASSETS_FONT_PRESSSTART2P_REGULAR: &str = "fonts/PressStart2P-Regular.ttf"; //フォント
pub const ASSETS_FONT_BIZUDPGOTHIC_REGULAR: &str = "fonts/BIZUDPGothic-Regular.ttf"; //フォント
pub const ASSETS_SPRITE_DEBUG_GRID        : &str = "sprites/debug_grid.png";         //スプライト
pub const ASSETS_SPRITE_BRICK_WALL        : &str = "sprites/brick_wall.png";         //スプライト
pub const ASSETS_SPRITE_KANI_DOTOWN       : &str = "sprites/kani_DOTOWN.png";        //スプライト
pub const ASSETS_SOUND_BEEP               : &str = "audio/sounds/beep.ogg";          //サウンド

//事前ロード対象のAsset
counted_array!
(   pub const FETCH_ASSETS: [ &str; _ ] =
    [   ASSETS_FONT_ORBITRON_BLACK,
        ASSETS_FONT_REGGAEONE_REGULAR,
        ASSETS_FONT_PRESSSTART2P_REGULAR,
        ASSETS_FONT_BIZUDPGOTHIC_REGULAR,
        ASSETS_SPRITE_DEBUG_GRID,
        ASSETS_SPRITE_BRICK_WALL,
        ASSETS_SPRITE_KANI_DOTOWN,
        ASSETS_SOUND_BEEP,
    ]
);

////////////////////////////////////////////////////////////////////////////////////////////////////

//ゲームパッドのID
pub const GAMEPAD: Gamepad = Gamepad { id: 0 }; //Todo: pad 0番決め打ちでいいいのか？

//パッドの十字ボタンチェック用
use std::collections::HashSet;
pub static CROSS_BUTTON_SET: Lazy<HashSet<GamepadButtonType>> = Lazy::new
(   ||
    HashSet::from
    (   [   GamepadButtonType::DPadRight, GamepadButtonType::DPadLeft,
            GamepadButtonType::DPadDown,  GamepadButtonType::DPadUp,
        ]
    )
);

//PAUSEのキーとパッドボタン
pub const KEY_PAUSE: KeyCode = KeyCode::Escape;
pub const BUTTON_PAUSE: GamepadButtonType = GamepadButtonType::North; //PS4の△ボタン

//FULLSCREENのキーとパッドボタン
pub const _KEY_ALT_RIGHT: KeyCode = KeyCode::RAlt;
pub const _KEY_ALT_LEFT : KeyCode = KeyCode::LAlt;
pub const _KEY_FULLSCREEN: KeyCode = KeyCode::Return;
pub const _BUTTON_FULLSCREEN: GamepadButtonType = GamepadButtonType::West; //PS4の□ボタン

//「HIT SPACE KEY」のキーとパッドボタン
pub const KEY_SPACE: KeyCode = KeyCode::Space;
pub const BUTTON_SPACE: GamepadButtonType = GamepadButtonType::East; //PS4の◯ボタン

////////////////////////////////////////////////////////////////////////////////////////////////////

pub const _DEPTH_SPRITE_DEBUG_GRID : f32 = 200.0; //スプライト重なり順
pub const  DEPTH_SPRITE_KANI_DOTOWN: f32 = 150.0; //スプライト重なり順
pub const  DEPTH_SPRITE_GAME_FRAME : f32 = 100.0; //スプライト重なり順
pub const  DEPTH_SPRITE_CHASER     : f32 = 40.0;  //スプライト重なり順
pub const  DEPTH_SPRITE_PLAYER     : f32 = 30.0;  //スプライト重なり順
pub const  DEPTH_SPRITE_DOT        : f32 = 20.0;  //スプライト重なり順
pub const  DEPTH_SPRITE_BRICK_WALL : f32 = 10.0;  //スプライト重なり順
pub const  DEPTH_SPRITE_TILE       : f32 = 0.0;   //スプライト重なり順
pub const _DEPTH_SPRITE_DEBUG_RECT : f32 = 0.0;   //スプライト重なり順

pub const _COLOR_SPRITE_DEBUG_GRID: Color = Color::rgba( 0.8, 0.8, 0.8, 0.1 );  //スプライト色(透過)
pub const  COLOR_SPRITE_DOT       : Color = Color::rgba( 1.0, 1.0, 0.7, 1.0 );  //スプライト色
pub const  COLOR_SPRITE_PLAYER    : Color = Color::YELLOW;                      //スプライト色
pub const  COLOR_SPRITE_TILE      : Color = Color::YELLOW;                      //スプライト色
pub const _COLOR_SPRITE_DEBUG_RECT: Color = Color::rgba( 0.0, 0.6, 0.0, 0.1 );  //スプライト色(透過)

pub const MAGNIFY_SPRITE_DOT   : f32 = 0.08;    //スプライト拡縮率
pub const MAGNIFY_SPRITE_PLAYER: f32 = 0.4;     //スプライト拡縮率
pub const MAGNIFY_SPRITE_CHASER: f32 = 0.5;     //スプライト拡縮率
pub const MAGNIFY_SPRITE_KANI  : f32 = 0.9;     //スプライト拡縮率

////////////////////////////////////////////////////////////////////////////////////////////////////

pub const VOLUME_SOUND_BEEP: f32 = 0.1; //SEボリューム

////////////////////////////////////////////////////////////////////////////////////////////////////

//自機の設定値
pub const PLAYER_WAIT: f32 = 0.09;                                  //移動のウェイト
pub const PLAYER_MOVE_COEF: f32 = PIXELS_PER_GRID / PLAYER_WAIT;    //移動の中割係数

//追手の設定値
pub const CHASER_WAIT: f32 = 0.13;                                  //移動のウェイト
pub const CHASER_MOVE_COEF: f32 = PIXELS_PER_GRID / CHASER_WAIT;    //移動の中割係数
pub const CHASER_ACCEL: f32 = 0.4;                                  //スピードアップの割増
pub const CHASER_INIT_POSITION: [ ( i32, i32 ); 4 ] =               //スタート座標(Grid)
[   ( 1    , 1     ),
    ( 1    , MAX_Y ),
    ( MAX_X, 1     ),
    ( MAX_X, MAX_Y ),
];
const MAX_X: i32 = MAP_GRIDS_WIDTH  - 2;
const MAX_Y: i32 = MAP_GRIDS_HEIGHT - 2;

////////////////////////////////////////////////////////////////////////////////////////////////////

//text UIの設定値
#[allow( dead_code )]
pub const NA3  : &str = "###";
pub const NA2_2: &str = "##.##";
pub const NA2  : &str = "##";
pub const NA5  : &str = "#####";
pub const NA2_5: &str = "##-#####";

//中央に表示するtext UI
const TITLE_COLOR1: Color = Color::rgba( 0.6, 1.0, 0.4, 0.75 );
const TITLE_COLOR2: Color = Color::rgba( 0.0, 0.7, 0.5, 0.75 );
counted_array!
(   pub const CENTER_TITLE_TEXT: [ MessageSect; _ ] =
    [   ( APP_TITLE, ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 3.5, TITLE_COLOR1 ),
        ( "\n "    , ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.7, TITLE_COLOR1 ),
        ( "v"      , ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.6, TITLE_COLOR2 ),
        ( CARGO_VER, ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.6, TITLE_COLOR2 ),
        ( "    "   , ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.6, TITLE_COLOR2 ),
    ]
);

counted_array!
(   pub const CENTER_DEMO_TEXT: [ MessageSect; _ ] =
    [   ( " "            , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 1.2, Color::YELLOW ),
        ( "\nD E M O\n\n", ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 1.0, Color::YELLOW ),
        ( "Hit SPACE Key", ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 1.0, Color::CYAN   ),
        ( "\nor A Button", ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.7, Color::CYAN   ),
    ]
);
pub const TEXT_UI_TITLE: TextUiTitle = TextUiTitle( MyState::GameStart, KEY_SPACE, BUTTON_SPACE );

counted_array!
(   pub const CENTER_START_TEXT: [ MessageSect; _ ] =
    [   ( "Game Start\n", ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 4.0, Color::CYAN   ),
        ( "\n"          , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.5, Color::NONE   ),
        ( "Ready...\n"  , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 3.0, Color::YELLOW ),
        ( "\n"          , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.5, Color::NONE   ),
        ( ""            , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 5.0, Color::YELLOW ),
    ]
);
pub const TEXT_UI_START: TextUiStart = TextUiStart ( 3, MyState::MainLoop, 4, cd_string_start );
fn cd_string_start( n: i32 ) -> String { if n == 0 { "Go!!".to_string() } else { n.to_string() } }

counted_array!
(   pub const CENTER_CLEAR_TEXT: [ MessageSect; _ ] =
    [   ( "C L E A R !!\n" , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 4.0, Color::CYAN   ),
        ( "\n"             , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.5, Color::NONE   ),
        ( "Next stage...\n", ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 3.0, Color::YELLOW ),
        ( "\n"             , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.5, Color::NONE   ),
        ( ""               , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 5.0, Color::YELLOW ),
    ]
);
pub const TEXT_UI_CLEAR: TextUiClear = TextUiClear ( 1, MyState::StageStart, 4, cd_string_clear );
fn cd_string_clear( n: i32 ) -> String { ( n + 4 ).to_string() }

counted_array!
(   pub const CENTER_OVER_TEXT: [ MessageSect; _ ] =
    [   ( "Game Over\n"    , ASSETS_FONT_REGGAEONE_REGULAR   , PIXELS_PER_GRID * 6.0, Color::RED    ),
        ( "\n"             , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.5, Color::NONE   ),
        ( "REPLAY?\n\n"    , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 1.0, Color::CYAN   ),
        ( "Hit SPACE Key"  , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 1.0, Color::CYAN   ),
        ( "\nor A Button\n", ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.7, Color::CYAN   ),
        ( ""               , ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 4.0, Color::YELLOW ),
    ]
);
pub const TEXT_UI_OVER: TextUiOver
    = TextUiOver( 10, MyState::TitleDemo, 5, cd_string_over, MyState::GameStart, KEY_SPACE, BUTTON_SPACE );
fn cd_string_over( n: i32 ) -> String { n.to_string() }

counted_array!
(   pub const CENTER_PAUSE_TEXT: [ MessageSect; _ ] =
    [   ( "P A U S E", ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 4.0, Color::SILVER ),
    ]
);

//ヘッダーに表示するtext UI
counted_array!
(   pub const HEADER_LEFT_TEXT: [ MessageSect; _ ] =
    [   ( " STAGE ", ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.7, Color::GOLD  ),
        ( NA2      , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.7, Color::WHITE ),
    ]
);
counted_array!
(   pub const HEADER_CENTER_TEXT: [ MessageSect; _ ] =
    [   ( " SCORE ", ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.7, Color::GOLD   ),
        ( NA5      , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.7, Color::WHITE  ),
        ( ""       , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.5, Color::SILVER ),  //placeholder for debug
    ]
);
counted_array!
(   pub const HEADER_RIGHT_TEXT: [ MessageSect; _ ] =
    [   ( " Hi-SCORE ", ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.7, Color::GOLD  ),
        ( NA5         , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.7, Color::WHITE ),
    ]
);

//フッターに表示するtext UI
counted_array!
(   pub const FOOTER_LEFT_TEXT: [ MessageSect; _ ] =
    [   ( " FPS " , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID       * 0.6, Color::TEAL   ),
        ( NA2_2   , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.4, Color::SILVER ),
        ( " demo ", ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID       * 0.6, Color::TEAL   ),
        ( NA2_5   , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.4, Color::SILVER ),
    ]
);
counted_array!
(   pub const FOOTER_CENTER_TEXT: [ MessageSect; _ ] =
    [   ( "hyoi 2021 - 2023", ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.6, Color::TEAL ),
    ]
);
counted_array!
(   pub const FOOTER_RIGHT_TEXT: [ MessageSect; _ ] =
    [   ( "Powered by RUST & BEVY ", ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.6, Color::TEAL ),
    ]
);

//debug用数字タイル
#[cfg( debug_assertions )]
counted_array!
(   pub const NUM_TILE_TEXT: [ MessageSect; _ ] =
    [   ( "", ASSETS_FONT_BIZUDPGOTHIC_REGULAR, PIXELS_PER_GRID * 0.3, Color::rgba( 1.0, 1.0, 1.0, 0.3 ) ),
    ]
);

//End of code.