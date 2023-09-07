use super::*;

////////////////////////////////////////////////////////////////////////////////

//アプリの情報
const _CARGO_TOML_NAME: &str = env!( "CARGO_PKG_NAME"    );
const _CARGO_TOML_VER : &str = env!( "CARGO_PKG_VERSION" );

pub const APP_TITLE: &str = _CARGO_TOML_NAME; //アプリタイトル
pub const APP_VER  : &str = _CARGO_TOML_VER;  //アプリのバージョン

////////////////////////////////////////////////////////////////////////////////

//単位Gridの縦横幅(Pixel)
const BASE_PIXELS: i32 = 8;
const SCALING: f32 = 4.0;
pub const PIXELS_PER_GRID: f32 = BASE_PIXELS as f32 * SCALING;

//GridのSize(縦横Pixel)
pub const SIZE_GRID: Px2d = Px2d::new( PIXELS_PER_GRID, PIXELS_PER_GRID );

////////////////////////////////////////////////////////////////////////////////

//ウィンドウ縦横幅(Grid)
pub const SCREEN_GRIDS_WIDTH  : i32 = 25; //best 43
pub const SCREEN_GRIDS_HEIGHT : i32 = 19; //best 24
pub const SCREEN_GRIDS_X_RANGE: Range<i32> = 0..SCREEN_GRIDS_WIDTH;
pub const SCREEN_GRIDS_Y_RANGE: Range<i32> = 0..SCREEN_GRIDS_HEIGHT;

//ウィンドウ縦横幅(Pixel)
pub const SCREEN_PIXELS_WIDTH : f32 = PIXELS_PER_GRID * SCREEN_GRIDS_WIDTH  as f32;
pub const SCREEN_PIXELS_HEIGHT: f32 = PIXELS_PER_GRID * SCREEN_GRIDS_HEIGHT as f32;

//ウィンドウ背景色
pub const SCREEN_BACKGROUND_COLOR: Color = Color::rgb( 0.13, 0.13, 0.18 );

//ウィンドウの設定
pub static MAIN_WINDOW: Lazy<Option<Window>> = Lazy::new
(   ||
    {   let title = format!( "{APP_TITLE} v{APP_VER}" );
        let window = Window
        {   title,
            resolution: ( SCREEN_PIXELS_WIDTH, SCREEN_PIXELS_HEIGHT ).into(),
            resizable: false,
            cursor: bevy::window::Cursor { visible: false, ..default() },
            fit_canvas_to_parent: true,
            ..default()
        };

        Some ( window )
    }
);

////////////////////////////////////////////////////////////////////////////////

//マップ縦横幅(Grid)
pub const MAP_GRIDS_WIDTH  : i32 = SCREEN_GRIDS_WIDTH;
pub const MAP_GRIDS_HEIGHT : i32 = SCREEN_GRIDS_HEIGHT - 2;
pub const MAP_GRIDS_X_RANGE: Range<i32> = 0..MAP_GRIDS_WIDTH;
pub const MAP_GRIDS_Y_RANGE: Range<i32> = 0..MAP_GRIDS_HEIGHT;

//マップのスクリーン上の原点
pub const MAP_ORIGIN_GRID: Grid = Grid::new( 0, 1 );

//枠のデザイン
pub static DESIGN_GAME_FRAME: Lazy<Vec<&str>> = Lazy::new
(   ||
    {   let frame = vec!
        [  //0123456789_123456789_1234
            "#########################", //0
            "#                       #", //1
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
            "#########################", //16
        ]; //0123456789_123456789_1234
        if frame[ 0 ].len() != MAP_GRIDS_WIDTH  as usize { panic!( "BAD WIDTH GAME FRAME"  ) }
        if frame.len()      != MAP_GRIDS_HEIGHT as usize { panic!( "BAD HEIGHT GAME FRAME" ) }

        frame
    }
);

////////////////////////////////////////////////////////////////////////////////

//スプライト、音声、フォント
pub const ASSETS_SPRITE_BRICK_WALL        : &str = "sprites/brick_wall.png";         //レンガ(壁)
pub const ASSETS_SPRITE_KANI_DOTOWN       : &str = "sprites/kani_DOTOWN.png";        //蟹
pub const ASSETS_SOUND_BEEP               : &str = "audio/sounds/beep.ogg";          //ピコ音
pub const ASSETS_FONT_ORBITRON_BLACK      : &str = "fonts/Orbitron-Black.ttf";       //フォント
pub const ASSETS_FONT_REGGAEONE_REGULAR   : &str = "fonts/ReggaeOne-Regular.ttf";    //フォント
pub const ASSETS_FONT_PRESSSTART2P_REGULAR: &str = "fonts/PressStart2P-Regular.ttf"; //フォント

//事前ロード対象のAsset
counted_array!
(   pub const FETCH_ASSETS: [ &str; _ ] =
    [   ASSETS_SPRITE_BRICK_WALL,
        ASSETS_SPRITE_KANI_DOTOWN,
        ASSETS_SOUND_BEEP,
        ASSETS_FONT_ORBITRON_BLACK,
        ASSETS_FONT_REGGAEONE_REGULAR,
        ASSETS_FONT_PRESSSTART2P_REGULAR,
    ]
);

////////////////////////////////////////////////////////////////////////////////

//TEXT UI の重なり順
pub const ZINDEX_TEXTUI_PAUSE: ZIndex = ZIndex::Global ( 999 );

//スプライト重なり順
pub const DEPTH_SPRITE_KANI_DOTOWN: f32 = 150.0;
pub const DEPTH_SPRITE_GAME_FRAME : f32 = 100.0; //ゲームの枠
pub const DEPTH_SPRITE_CHASER     : f32 = 40.0;
pub const DEPTH_SPRITE_PLAYER     : f32 = 30.0;
pub const DEPTH_SPRITE_DOT        : f32 = 20.0;
pub const DEPTH_SPRITE_BRICK_WALL : f32 = 10.0;

//スプライト色
pub const COLOR_SPRITE_DOT   : Color = Color::rgb( 1.0, 1.0, 0.7 );
pub const COLOR_SPRITE_PLAYER: Color = Color::YELLOW;

//スプライト拡縮率
pub const MAGNIFY_SPRITE_DOT   : f32 = 0.08;
pub const MAGNIFY_SPRITE_PLAYER: f32 = 0.4;
pub const MAGNIFY_SPRITE_CHASER: f32 = 0.5;
pub const MAGNIFY_SPRITE_KANI  : f32 = 0.9;

//SEボリューム
pub const VOLUME_SOUND_BEEP: f32 = 0.1;
// pub const VOLUME_SOUND_BEEP: Volume = Volume::Relative ( VolumeLevel::new( 0.1 ) );
//と書きたいがVolumeLevel::new()がnon-const fnなので書けない。

////////////////////////////////////////////////////////////////////////////////

//フルスクリーン切替のキーとパッドボタン
counted_array!
(   pub const FULL_SCREEN_KEYS: [ ( Option<KeyCode>, KeyCode ); _ ] =
    [   ( Some ( KeyCode::AltLeft  ), KeyCode::Return ),
        ( Some ( KeyCode::AltRight ), KeyCode::Return ),
    ]
);
pub const FULL_SCREEN_BUTTON: GamepadButtonType = GamepadButtonType::Select; //PS4:SHARE

//PAUSEのキーとパッドボタン
pub const PAUSE_KEY: KeyCode = KeyCode::Escape;
pub const PAUSE_BUTTON: GamepadButtonType = GamepadButtonType::Start; //PS4:OPTIONS

//Hit ANY Keyの処理で押されても無視するキーとボタン
counted_array!
(   pub const HAK_IGNORE_KEYS: [ KeyCode; _ ] =
    [   KeyCode::AltLeft    , KeyCode::AltRight,
        KeyCode::ControlLeft, KeyCode::ControlRight,
        KeyCode::ShiftLeft  , KeyCode::ShiftRight,
        PAUSE_KEY,
    ]
);
counted_array!
(   pub const HAK_IGNORE_BUTTONS: [ GamepadButtonType; _ ] =
    [   PAUSE_BUTTON, FULL_SCREEN_BUTTON,
        GamepadButtonType::Mode,
    ]
);

//パッド十字ボタンチェック用
pub static CROSS_BUTTONS: Lazy<HashSet<GamepadButtonType>> = Lazy::new
(   ||
    HashSet::from
    (   [   GamepadButtonType::DPadRight, GamepadButtonType::DPadLeft,
            GamepadButtonType::DPadDown,  GamepadButtonType::DPadUp,
        ]
    )
);

////////////////////////////////////////////////////////////////////////////////

//中央に表示するtext UI
const TITLE_COLOR1: Color = Color::rgba( 0.6, 1.0, 0.4, 0.75 );
const TITLE_COLOR2: Color = Color::rgba( 0.0, 0.7, 0.5, 0.75 );
counted_array!
(   pub const CENTER_TITLE_TEXT: [ MessageSect; _ ] =
    [   ( APP_TITLE, ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 3.5, TITLE_COLOR1 ),
        ( "\n"     , ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.7, TITLE_COLOR1 ),
        ( "v"      , ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.6, TITLE_COLOR2 ),
        ( APP_VER  , ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.6, TITLE_COLOR2 ),
        ( "    "   , ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.6, TITLE_COLOR2 ),
    ]
);

counted_array!
(   pub const CENTER_DEMO_TEXT: [ MessageSect; _ ] =
    [   ( " "            , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 1.2, Color::NONE ),
        ( "\nD E M O\n\n", ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 1.2, Color::GOLD ),
        ( "Hit ANY key!" , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.9, Color::CYAN ),
        ( "\nor\n"       , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.8, Color::CYAN ),
        ( "ANY button!"  , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.9, Color::CYAN ),
    ]
);
pub const TEXT_UI_TITLE: TextUiTitle = TextUiTitle( MyState::StageStart );

counted_array!
(   pub const CENTER_START_TEXT: [ MessageSect; _ ] =
    [   ( "Game Start\n", ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 4.0, Color::CYAN ),
        ( "\n"          , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.5, Color::NONE ),
        ( "Ready...\n"  , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 3.0, Color::GOLD ),
        ( "\n"          , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.5, Color::NONE ),
        ( ""            , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 5.0, Color::GOLD ),
    ]
);
pub const TEXT_UI_START: TextUiStart = TextUiStart ( 3, MyState::MainLoop, 4, cd_string_start );
fn cd_string_start( n: i32 ) -> String { if n == 0 { "Go!!".to_string() } else { n.to_string() } }

counted_array!
(   pub const CENTER_CLEAR_TEXT: [ MessageSect; _ ] =
    [   ( "C L E A R !!\n" , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 4.0, Color::CYAN ),
        ( "\n"             , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.5, Color::NONE ),
        ( "Next stage...\n", ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 3.0, Color::GOLD ),
        ( "\n"             , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 0.5, Color::NONE ),
        ( ""               , ASSETS_FONT_ORBITRON_BLACK, PIXELS_PER_GRID * 5.0, Color::GOLD ),
    ]
);
pub const TEXT_UI_CLEAR: TextUiClear = TextUiClear ( 1, MyState::StageStart, 4, cd_string_clear );
fn cd_string_clear( n: i32 ) -> String { ( n + 4 ).to_string() }

counted_array!
(   pub const CENTER_OVER_TEXT: [ MessageSect; _ ] =
    [   ( "Game Over\n"  , ASSETS_FONT_REGGAEONE_REGULAR   , PIXELS_PER_GRID * 6.0, Color::RED  ),
        ( "\n"           , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.5, Color::NONE ),
        ( "REPLAY?\n\n"  , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 1.2, Color::GOLD ),
        ( "Hit ANY key!" , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.9, Color::CYAN ),
        ( "\nor\n"       , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.8, Color::CYAN ),
        ( "ANY button!\n", ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.9, Color::CYAN ),
        ( ""             , ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 4.0, Color::GOLD ),
    ]
);
pub const TEXT_UI_OVER: TextUiOver = TextUiOver( 10, MyState::Title, 6, cd_string_over, MyState::StageStart );
fn cd_string_over( n: i32 ) -> String { n.to_string() }

//text UIの設定値
#[allow( dead_code )]
pub const NA2  : &str = "##";
pub const NA3  : &str = "###";
pub const NA5  : &str = "#####";
pub const NA2_5: &str = "##-#####";
pub const NA3_2: &str = "###.##";

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
        ( NA3      , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.5, Color::SILVER ),  //placeholder for debug
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
        ( NA3_2   , ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.4, Color::SILVER ),
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

////////////////////////////////////////////////////////////////////////////////

//自機の設定値
pub const PLAYER_WAIT: f32 = 0.09;                               //移動のウェイト
pub const PLAYER_MOVE_COEF: f32 = PIXELS_PER_GRID / PLAYER_WAIT; //移動の中割係数

//追手の設定値
pub const CHASER_WAIT: f32 = 0.13;                               //移動のウェイト
pub const CHASER_MOVE_COEF: f32 = PIXELS_PER_GRID / CHASER_WAIT; //移動の中割係数
pub const CHASER_ACCEL: f32 = 0.4;                               //スピードアップの割増
pub const CHASER_INIT_POSITION: [ ( i32, i32 ); 4 ] =            //スタート座標(Grid)
[   ( 1    , 1     ),
    ( 1    , MAX_Y ),
    ( MAX_X, 1     ),
    ( MAX_X, MAX_Y ),
];
const MAX_X: i32 = MAP_GRIDS_WIDTH  - 2;
const MAX_Y: i32 = MAP_GRIDS_HEIGHT - 2;

////////////////////////////////////////////////////////////////////////////////

//End of code.