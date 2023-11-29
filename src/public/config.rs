use super::*;

////////////////////////////////////////////////////////////////////////////////

//.run_if( condition )用
pub const DEBUG: fn() -> bool = || cfg!( debug_assertions );
pub const WASM : fn() -> bool = || cfg!( target_arch = "wasm32" );

////////////////////////////////////////////////////////////////////////////////

//ウィンドウの定義
pub static MAIN_WINDOW: Lazy<Option<Window>> = Lazy::new
(   ||
    {   let title = format!( "{APP_TITLE} v{APP_VER}" );
        let window = Window
        {   title,
            resolution: ( SCREEN_PIXELS_WIDTH, SCREEN_PIXELS_HEIGHT ).into(),
            resizable: false,
            // fit_canvas_to_parent: true, //不具合が発生した場合コメントアウトする
            ..default()
        };

        Some ( window )
    }
);

////////////////////////////////////////////////////////////////////////////////

//アプリの情報
const _CARGO_TOML_NAME: &str = env!( "CARGO_PKG_NAME"    );
const _CARGO_TOML_VER : &str = env!( "CARGO_PKG_VERSION" );

const APP_TITLE: &str = _CARGO_TOML_NAME; //アプリタイトル
const APP_VER  : &str = _CARGO_TOML_VER;  //アプリのバージョン

////////////////////////////////////////////////////////////////////////////////

//ウィンドウ縦横幅(Pixel)
pub const SCREEN_PIXELS_WIDTH : f32 = PIXELS_PER_GRID * SCREEN_GRIDS_WIDTH  as f32;
pub const SCREEN_PIXELS_HEIGHT: f32 = PIXELS_PER_GRID * SCREEN_GRIDS_HEIGHT as f32;

//ウィンドウ背景色
pub const SCREEN_BACKGROUND_COLOR: Color = Color::rgb( 0.13, 0.13, 0.18 );

////////////////////////////////////////////////////////////////////////////////

//単位Gridの縦横幅(Pixel)
const BASE_PIXELS: i32 = 8;
const SCALING: f32 = 4.0;
pub const PIXELS_PER_GRID: f32 = BASE_PIXELS as f32 * SCALING;

//GridのSize(縦横Pixel)
pub const SIZE_GRID: Vec2 = Vec2::new( PIXELS_PER_GRID, PIXELS_PER_GRID );

////////////////////////////////////////////////////////////////////////////////

//ウィンドウ縦横幅(Grid)
pub const SCREEN_GRIDS_WIDTH : i32 = 25; //memo: best 43
pub const SCREEN_GRIDS_HEIGHT: i32 = 19; //memo: best 24

pub const SCREEN_GRIDS_X_RANGE: Range<i32> = 0..SCREEN_GRIDS_WIDTH;
pub const SCREEN_GRIDS_Y_RANGE: Range<i32> = 0..SCREEN_GRIDS_HEIGHT;

////////////////////////////////////////////////////////////////////////////////

//ログレベル
pub const LOG_LEVEL_DEV: &str = "warn,wgpu_hal=error"; //開発
pub const LOG_LEVEL_REL: &str = "error"; //リリース

////////////////////////////////////////////////////////////////////////////////

//assets（スプライト）
pub const ASSETS_SPRITE_DEBUG_GRID : &str = "sprites/debug_grid.png";
pub const ASSETS_SPRITE_BRICK_WALL : &str = "sprites/brick_wall.png";
pub const ASSETS_SPRITE_KANI_DOTOWN: &str = "sprites/kani_DOTOWN.png";

pub const ASSETS_SPRITE_SHEET_PLAYER_NORTH: &str = "sprites/player/sheet_player_north.png";
pub const ASSETS_SPRITE_SHEET_PLAYER_EAST : &str = "sprites/player/sheet_player_east.png";
pub const ASSETS_SPRITE_SHEET_PLAYER_WEST : &str = "sprites/player/sheet_player_west.png";
pub const ASSETS_SPRITE_SHEET_PLAYER_SOUTH: &str = "sprites/player/sheet_player_south.png";

pub const ASSETS_SPRITE_SHEET_CHASER_1_NORTH: &str = "sprites/chasers/sheet_chaser_red_north.png";
pub const ASSETS_SPRITE_SHEET_CHASER_1_EAST : &str = "sprites/chasers/sheet_chaser_red_east.png";
pub const ASSETS_SPRITE_SHEET_CHASER_1_WEST : &str = "sprites/chasers/sheet_chaser_red_west.png";
pub const ASSETS_SPRITE_SHEET_CHASER_1_SOUTH: &str = "sprites/chasers/sheet_chaser_red_south.png";

pub const ASSETS_SPRITE_SHEET_CHASER_2_NORTH: &str = "sprites/chasers/sheet_chaser_blue_north.png";
pub const ASSETS_SPRITE_SHEET_CHASER_2_EAST : &str = "sprites/chasers/sheet_chaser_blue_east.png";
pub const ASSETS_SPRITE_SHEET_CHASER_2_WEST : &str = "sprites/chasers/sheet_chaser_blue_west.png";
pub const ASSETS_SPRITE_SHEET_CHASER_2_SOUTH: &str = "sprites/chasers/sheet_chaser_blue_south.png";

pub const ASSETS_SPRITE_SHEET_CHASER_3_NORTH: &str = "sprites/chasers/sheet_chaser_green_north.png";
pub const ASSETS_SPRITE_SHEET_CHASER_3_EAST : &str = "sprites/chasers/sheet_chaser_green_east.png";
pub const ASSETS_SPRITE_SHEET_CHASER_3_WEST : &str = "sprites/chasers/sheet_chaser_green_west.png";
pub const ASSETS_SPRITE_SHEET_CHASER_3_SOUTH: &str = "sprites/chasers/sheet_chaser_green_south.png";

pub const ASSETS_SPRITE_SHEET_CHASER_4_NORTH: &str = "sprites/chasers/sheet_chaser_pink_north.png";
pub const ASSETS_SPRITE_SHEET_CHASER_4_EAST : &str = "sprites/chasers/sheet_chaser_pink_east.png";
pub const ASSETS_SPRITE_SHEET_CHASER_4_WEST : &str = "sprites/chasers/sheet_chaser_pink_west.png";
pub const ASSETS_SPRITE_SHEET_CHASER_4_SOUTH: &str = "sprites/chasers/sheet_chaser_pink_south.png";

//assets（フォント）
pub const ASSETS_FONT_ORBITRON_BLACK      : &str = "fonts/Orbitron-Black.ttf";
pub const ASSETS_FONT_PRESSSTART2P_REGULAR: &str = "fonts/PressStart2P-Regular.ttf";
pub const ASSETS_FONT_REGGAEONE_REGULAR   : &str = "fonts/ReggaeOne-Regular.ttf";

//assets（サウンド）
pub const ASSETS_SOUND_BEEP: &str = "audio/sounds/beep.ogg";

//事前ロード対象
counted_array!
(   pub const PRELOAD_ASSETS: [ &str; _ ] =
    [   ASSETS_SPRITE_DEBUG_GRID,
        ASSETS_SPRITE_BRICK_WALL,
        ASSETS_SPRITE_KANI_DOTOWN,

        ASSETS_SPRITE_SHEET_PLAYER_NORTH,
        ASSETS_SPRITE_SHEET_PLAYER_EAST,
        ASSETS_SPRITE_SHEET_PLAYER_WEST,
        ASSETS_SPRITE_SHEET_PLAYER_SOUTH,

        ASSETS_SPRITE_SHEET_CHASER_1_NORTH,
        ASSETS_SPRITE_SHEET_CHASER_1_EAST ,
        ASSETS_SPRITE_SHEET_CHASER_1_WEST ,
        ASSETS_SPRITE_SHEET_CHASER_1_SOUTH,

        ASSETS_SPRITE_SHEET_CHASER_2_NORTH,
        ASSETS_SPRITE_SHEET_CHASER_2_EAST ,
        ASSETS_SPRITE_SHEET_CHASER_2_WEST ,
        ASSETS_SPRITE_SHEET_CHASER_2_SOUTH,

        ASSETS_SPRITE_SHEET_CHASER_3_NORTH,
        ASSETS_SPRITE_SHEET_CHASER_3_EAST ,
        ASSETS_SPRITE_SHEET_CHASER_3_WEST ,
        ASSETS_SPRITE_SHEET_CHASER_3_SOUTH,

        ASSETS_SPRITE_SHEET_CHASER_4_NORTH,
        ASSETS_SPRITE_SHEET_CHASER_4_EAST ,
        ASSETS_SPRITE_SHEET_CHASER_4_WEST ,
        ASSETS_SPRITE_SHEET_CHASER_4_SOUTH,

        ASSETS_FONT_ORBITRON_BLACK,
        ASSETS_FONT_PRESSSTART2P_REGULAR,
        ASSETS_FONT_REGGAEONE_REGULAR,
        ASSETS_SOUND_BEEP,
    ]
);

////////////////////////////////////////////////////////////////////////////////

//アニメーションするスプライトの情報（player）
pub const ANIME_PLAYER_ASSETS: &[ ( News, &str ) ] =
&[  ( News::North, ASSETS_SPRITE_SHEET_PLAYER_NORTH ),
    ( News::East , ASSETS_SPRITE_SHEET_PLAYER_EAST  ),
    ( News::West , ASSETS_SPRITE_SHEET_PLAYER_WEST  ),
    ( News::South, ASSETS_SPRITE_SHEET_PLAYER_SOUTH ),
];
pub const ANIME_PLAYER_SIZE : Vec2  = Vec2::new( 8.0, 8.0 );
pub const ANIME_PLAYER_COLS : usize = 4;
pub const ANIME_PLAYER_ROWS : usize = 4;
pub const ANIME_PLAYER_TIMER: f32   = 0.15;

//アニメーションするスプライトの情報（chaser）
pub const ANIME_CHASERS_ASSETS: &[ &[ ( News, &str ) ] ] =
&[  &[  ( News::North, ASSETS_SPRITE_SHEET_CHASER_1_NORTH ),
        ( News::East , ASSETS_SPRITE_SHEET_CHASER_1_EAST  ),
        ( News::West , ASSETS_SPRITE_SHEET_CHASER_1_WEST  ),
        ( News::South, ASSETS_SPRITE_SHEET_CHASER_1_SOUTH ),
    ],
    &[  ( News::North, ASSETS_SPRITE_SHEET_CHASER_2_NORTH ),
        ( News::East , ASSETS_SPRITE_SHEET_CHASER_2_EAST  ),
        ( News::West , ASSETS_SPRITE_SHEET_CHASER_2_WEST  ),
        ( News::South, ASSETS_SPRITE_SHEET_CHASER_2_SOUTH ),
    ],
    &[  ( News::North, ASSETS_SPRITE_SHEET_CHASER_3_NORTH ),
        ( News::East , ASSETS_SPRITE_SHEET_CHASER_3_EAST  ),
        ( News::West , ASSETS_SPRITE_SHEET_CHASER_3_WEST  ),
        ( News::South, ASSETS_SPRITE_SHEET_CHASER_3_SOUTH ),
    ],
    &[  ( News::North, ASSETS_SPRITE_SHEET_CHASER_4_NORTH ),
        ( News::East , ASSETS_SPRITE_SHEET_CHASER_4_EAST  ),
        ( News::West , ASSETS_SPRITE_SHEET_CHASER_4_WEST  ),
        ( News::South, ASSETS_SPRITE_SHEET_CHASER_4_SOUTH ),
    ],
];
pub const ANIME_CHASER_SIZE : Vec2  = Vec2::new( 8.0, 8.0 );
pub const ANIME_CHASER_COLS : usize = 4;
pub const ANIME_CHASER_ROWS : usize = 4;
pub const ANIME_CHASER_TIMER: f32   = 0.15;

////////////////////////////////////////////////////////////////////////////////

//スプライト重なり
pub const DEPTH_SPRITE_DEBUG_GRID : f32 = 999.0; //重なりの最大値
pub const DEPTH_SPRITE_KANI_DOTOWN: f32 = 900.0;
pub const DEPTH_SPRITE_GAME_FRAME : f32 = 800.0;
pub const DEPTH_SPRITE_CHASER     : f32 = 700.0;
pub const DEPTH_SPRITE_PLAYER     : f32 = 600.0;
pub const DEPTH_SPRITE_DOT        : f32 = 500.0;
pub const DEPTH_SPRITE_BRICK_WALL : f32 = 400.0;

//TEXT UI の重なり順
// pub const ZINDEX_TEXTUI_PAUSE: ZIndex = ZIndex::Global ( 999 );

//スプライト色
pub const COLOR_SPRITE_DOT   : Color = Color::rgb( 1.0, 1.0, 0.7 );
pub const COLOR_SPRITE_PLAYER: Color = Color::YELLOW;

//スプライト拡縮
pub const MAGNIFY_SPRITE_DOT   : f32 = 0.08;
pub const MAGNIFY_SPRITE_PLAYER: f32 = 0.4;
pub const MAGNIFY_SPRITE_CHASER: f32 = 0.5;

//SEボリューム
pub const VOLUME_SOUND_BEEP: f32 = 0.1;
// pub const VOLUME_SOUND_BEEP: Volume = Volume::Relative ( VolumeLevel::new( 0.1 ) );
//と書きたいがVolumeLevel::new()がnon-const fnなので書けない。

////////////////////////////////////////////////////////////////////////////////

//マップデータからスプライト座標を計算する場合の調整値
pub const ADJUSTER_MAP_SPRITES: Vec2 = Vec2::new( 0.0, -1.0 * PIXELS_PER_GRID );

//画面デザイン(枠)
pub const SCREEN_FRAME_SPACE_CHAR : char = ' ';
pub const SCREEN_FRAME_LABEL_REGEX: &str = r"[a-zA-Z0-9\.]+";
pub static SCREEN_FRAME: Lazy<ScreenFrame> = Lazy::new
(   ||
    {   let design = vec!
        [  //0123456789_123456789_1234
            "                         ", //0
            "#########################", //1
            "#                       #", //2
            "#                       #", //3
            "#                       #", //4
            "#                       #", //5
            "#                       #", //6
            "#                       #", //7
            "#                       #", //8
            "#                       #", //9
            "#                       #", //10
            "#                       #", //11
            "#                       #", //12
            "#                       #", //13
            "#                       #", //14
            "#                       #", //15
            "#                       #", //16
            "#########################", //17
            "                         ", //18
        ]; //0123456789_123456789_1234

        if design[ 0 ].len() != SCREEN_GRIDS_WIDTH  as usize
        || design.len()      != SCREEN_GRIDS_HEIGHT as usize
        {   panic!( "APP ERR: {}", ER_BAD_SCREEN_DESIGN );
        }

        ScreenFrame { design }
    }
);

//エラーメッセージ
const ER_BAD_SCREEN_DESIGN: &str = "Frame design unmatch screen width/height.";

////////////////////////////////////////////////////////////////////////////////

//PAUSEのキー／ボタン
pub const PAUSE_KEY: KeyCode = KeyCode::Escape;
pub const PAUSE_BUTTON: GamepadButtonType = GamepadButtonType::Select; //PS4[SHARE]

////////////////////////////////////////////////////////////////////////////////

//フルスクリーンのキー／ボタン
pub const FULL_SCREEN_KEY: KeyCode = KeyCode::Return;
counted_array!
(   pub const FULL_SCREEN_KEY_MODIFIER: [ KeyCode; _ ] =
    [   KeyCode::AltRight,
        KeyCode::AltLeft,
    ]
);
pub const FULL_SCREEN_BUTTON: GamepadButtonType = GamepadButtonType::Start; //ps4[OPTIONS]

////////////////////////////////////////////////////////////////////////////////

//マップ縦横幅(Grid)
pub const MAP_GRIDS_WIDTH : i32 = SCREEN_GRIDS_WIDTH;
pub const MAP_GRIDS_HEIGHT: i32 = SCREEN_GRIDS_HEIGHT - 2;

//マップのレンジ（外壁含む）
pub const MAP_GRIDS_X_RANGE: Range<i32> = 0..MAP_GRIDS_WIDTH;
pub const MAP_GRIDS_Y_RANGE: Range<i32> = 0..MAP_GRIDS_HEIGHT;

////////////////////////////////////////////////////////////////////////////////

//プレイヤーの設定値
pub const PLAYER_TIME_PER_GRID: f32 = 0.15;//0.09; //１グリッド進むために必要な時間
pub const PLAYER_SPEED: f32 = PIXELS_PER_GRID / PLAYER_TIME_PER_GRID; //速度

////////////////////////////////////////////////////////////////////////////////

//チェイサーの設定値
pub const CHASER_TIME_PER_GRID: f32 = 0.20;//0.13; //１グリッド進むために必要な時間
pub const CHASER_SPEED: f32 = PIXELS_PER_GRID / CHASER_TIME_PER_GRID; //速度
pub const CHASER_ACCEL: f32 = 0.4; //スピードアップの割増
pub const CHASER_START_POSITION: &[ IVec2 ] = //スタート座標
&[  IVec2::new( 1    , 1     ),
    IVec2::new( 1    , MAX_Y ),
    IVec2::new( MAX_X, 1     ),
    IVec2::new( MAX_X, MAX_Y ),
];
const MAX_X: i32 = MAP_GRIDS_WIDTH  - 2;
const MAX_Y: i32 = MAP_GRIDS_HEIGHT - 2;

////////////////////////////////////////////////////////////////////////////////

//ヘッダー＆フッターのプレイスホルダー
pub const NA2  : &str = "##";
pub const NA5  : &str = "#####";
pub const NA2_5: &str = "##-#####";
pub const NA3_2: &str = "###.##";
const PLACE_HOLDERS_HEAD_FOOT: &[ &str ] = &[ NA2, NA5, NA2_5, NA3_2 ];

//ヘッダーの設定
pub const TEXT_HEADER_LEFT: &[ MessageSect ] =
&[  ( " STAGE ", ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.7, Color::GOLD  ),
    ( NA2      , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.7, Color::WHITE ),
];
pub static PLACE_HOLDER_HEADER_LEFT: Lazy<Option<usize>> = Lazy::new
(   || TEXT_HEADER_LEFT.iter().position( |x| PLACE_HOLDERS_HEAD_FOOT.contains( &x.0 ) )
);

pub const TEXT_HEADER_CENTER: &[ MessageSect ] =
&[  ( " SCORE ", ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.7, Color::GOLD  ),
    ( NA5      , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.7, Color::WHITE ),
];
pub static PLACE_HOLDER_HEADER_CENTER: Lazy<Option<usize>> = Lazy::new
(   || TEXT_HEADER_CENTER.iter().position( |x| PLACE_HOLDERS_HEAD_FOOT.contains( &x.0 ) )
);

pub const TEXT_HEADER_RIGHT: &[ MessageSect ] =
&[  ( " Hi-SCORE ", ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.7, Color::GOLD  ),
    ( NA5         , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.7, Color::WHITE ),
];
pub static PLACE_HOLDER_HEADER_RIGHT: Lazy<Option<usize>> = Lazy::new
(   || TEXT_HEADER_RIGHT.iter().position( |x| PLACE_HOLDERS_HEAD_FOOT.contains( &x.0 ) )
);

//フッターの設定
pub const TEXT_FOOTER_LEFT: &[ MessageSect ] =
&[  ( "  FPS ", ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.60, Color::TEAL   ),
    ( NA3_2   , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.40, Color::SILVER ),
    ( " demo ", ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.45, Color::TEAL   ),
    ( NA2_5   , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.25, Color::SILVER ),
];

pub const TEXT_FOOTER_CENTER: &[ MessageSect ] =
&[  ( "hyoi 2021 - 2023", ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.6, Color::TEAL ),
];

pub const TEXT_FOOTER_RIGHT: &[ MessageSect ] =
&[  ( "Powered by ", ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.6, Color::TEAL   ),
    ( "RUST"       , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.6, Color::SILVER ),
    ( " & "        , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.6, Color::TEAL   ),
    ( "BEVY  "     , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.6, Color::SILVER ),
];

//おまけ(蟹)
pub const GRID_X_KANI: i32 = SCREEN_GRIDS_WIDTH  - 4;
pub const GRID_Y_KANI: i32 = SCREEN_GRIDS_HEIGHT - 1;
pub const MAGNIFY_SPRITE_KANI: f32 = 0.9;
pub const COLOR_SPRITE_KANI: Color = Color::rgba( 1.0, 1.0, 1.0, 0.6 );

////////////////////////////////////////////////////////////////////////////////

//カウントダウンのプレイスホルダー
pub const CDPH: &str = "__Placeholder_for_countdown__";

//メッセージの設定
pub const UI_START: &[ MessageSect ] =
&[  ( "Start\n"   , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 4.0, Color::CYAN ),
    ( "\n"        , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.5, Color::NONE ),
    ( "Ready...\n", ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 3.0, Color::GOLD ),
    ( "\n"        , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.5, Color::NONE ),
    ( CDPH        , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 5.0, Color::GOLD ),
];

pub const UI_CLEAR: &[ MessageSect ] =
&[  ( "C L E A R !!\n" , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 4.0, Color::CYAN ),
    ( "\n"             , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.5, Color::NONE ),
    ( "Next stage...\n", ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 3.0, Color::GOLD ),
    ( "\n"             , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.5, Color::NONE ),
    ( CDPH             , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 5.0, Color::GOLD ),
];

pub const UI_OVER: &[ MessageSect ] =
&[  ( "Game Over\n"  , ASSETS_FONT_REGGAEONE_REGULAR   , PIXELS_PER_GRID * 6.0, Color::RED  ),
    ( "\n"           , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.5, Color::NONE ),
    ( "REPLAY?\n\n"  , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 1.2, Color::GOLD ),
    ( "Hit ANY key!" , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.9, Color::CYAN ),
    ( "\nor\n"       , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.8, Color::CYAN ),
    ( "ANY button!\n", ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.9, Color::CYAN ),
    ( CDPH           , ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 4.0, Color::GOLD ),
];

////////////////////////////////////////////////////////////////////////////////

//タイトルの設定
const TITLE_COLOR1: Color = Color::rgba( 0.6, 1.0, 0.4, 0.75 );
const TITLE_COLOR2: Color = Color::rgba( 0.0, 0.7, 0.5, 0.75 );
pub const UI_TITLE: &[ MessageSect ] =
&[  ( APP_TITLE, ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 3.5, TITLE_COLOR1 ),
    ( "\nv"    , ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.6, TITLE_COLOR2 ),
    ( APP_VER  , ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.6, TITLE_COLOR2 ),
    ( "     \n", ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.6, TITLE_COLOR2 ),
];
pub const UI_DEMO: &[ MessageSect ] =
&[  ( "\nD E M O\n\n", ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 1.2, Color::GOLD ),
    ( "Hit ANY key!" , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.9, Color::CYAN ),
    ( "\nor\n"       , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.8, Color::CYAN ),
    ( "ANY button!"  , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.9, Color::CYAN ),
];

//PAUSEメッセージの設定
pub const UI_PAUSE: &[ MessageSect ] =
&[  ( "P A U S E", ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 4.0, Color::SILVER ),
];

////////////////////////////////////////////////////////////////////////////////

//Hit ANY Keyの処理で無視するキーとボタン
pub const HAK_IGNORE_KEYS: &[ KeyCode ] =
&[  KeyCode::AltLeft    , KeyCode::AltRight,
    KeyCode::ControlLeft, KeyCode::ControlRight,
    KeyCode::ShiftLeft  , KeyCode::ShiftRight,
    PAUSE_KEY,
];
pub const HAK_IGNORE_BUTTONS: &[ GamepadButtonType ] =
&[  PAUSE_BUTTON, FULL_SCREEN_BUTTON,
];

////////////////////////////////////////////////////////////////////////////////

//End of code.