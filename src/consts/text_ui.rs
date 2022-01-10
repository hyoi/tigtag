use super::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

//定義と定数

//事前ロード対象のAsset
pub const FONT_PRESSSTART2P_REGULAR: &str = "fonts/PressStart2P-Regular.ttf";
pub const FONT_ORBITRON_BLACK	   : &str = "fonts/Orbitron-Black.ttf";
pub const FONT_REGGAEONE_REGULAR   : &str = "fonts/ReggaeOne-Regular.ttf";
pub const IMAGE_SPRITE_WALL		   : &str = "sprites/wall.png";

//TEXT UIのメッセージセクションの型
type MessageSect<'a> = ( &'a str, &'a str, f32, Color );

//中央
#[ derive( Component ) ]
pub struct MessageDemo;
const MESSAGE_DEMO: [ MessageSect; 4 ] =
[	( "TigTag\n\n"    , FONT_PRESSSTART2P_REGULAR, PIXEL_PER_GRID * 3.5, Color::rgb_linear( 0.3, 1.0, 0.1 ) ),
	( "D E M O\n\n"   , FONT_PRESSSTART2P_REGULAR, PIXEL_PER_GRID * 1.0, Color::YELLOW ),
	( "Game Start\n\n", FONT_PRESSSTART2P_REGULAR, PIXEL_PER_GRID * 1.0, Color::CYAN   ),
	( "Hit SPACE Key" , FONT_PRESSSTART2P_REGULAR, PIXEL_PER_GRID * 1.0, Color::CYAN   ),
];

#[ derive( Component ) ]
pub struct MessageStart
{	pub count: usize,
	pub timer: Timer,
}
const MESSAGE_START: [ MessageSect; 1 ] =
[	( "", FONT_ORBITRON_BLACK, PIXEL_PER_GRID * 4.0, Color::CYAN ),
];
pub const COUNTDOWN_TEXT: [ &str; 4 ] = [ "Go!", "Ready...\n1", "Ready...\n2", "Ready...\n3" ];

#[ derive( Component ) ]
pub struct MessageClear
{	pub count: usize,
	pub timer: Timer,
}
const MESSAGE_CLEAR: [ MessageSect; 3 ] =
[	( "Clear!!"            , FONT_ORBITRON_BLACK      , PIXEL_PER_GRID * 6.0, Color::YELLOW ),
	( "\nNext stage...\n\n", FONT_PRESSSTART2P_REGULAR, PIXEL_PER_GRID * 2.0, Color::WHITE  ),
	( ""                   , FONT_ORBITRON_BLACK      , PIXEL_PER_GRID * 4.0, Color::WHITE  ),
];
pub const GAMECLEAR_COUNTDOWN: usize = 5;

#[ derive( Component ) ]
pub struct MessageOver
{	pub count: usize,
	pub timer: Timer,
}
const MESSAGE_OVER: [ MessageSect; 3 ] =
[	( "GameOver"                      , FONT_REGGAEONE_REGULAR   , PIXEL_PER_GRID * 6.0, Color::RED    ),
	( "\n\nReplay?\n\nHit SPACE Key\n", FONT_PRESSSTART2P_REGULAR, PIXEL_PER_GRID * 1.0, Color::YELLOW ),
	( ""                              , FONT_ORBITRON_BLACK      , PIXEL_PER_GRID * 4.0, Color::CYAN   ),
];
pub const GAMEOVER_COUNTDOWN: usize = 10;

#[ derive( Component ) ]
pub struct MessagePause;
const MESSAGE_PAUSE: [ MessageSect; 1 ] =
[	( "P A U S E", FONT_ORBITRON_BLACK, PIXEL_PER_GRID * 4.0, Color::WHITE ),
];

//上端
#[ derive( Component ) ]
pub struct UiUpperLeft;
const UI_UPPER_LEFT: [ MessageSect; 2 ] =
[	( "STAGE", FONT_PRESSSTART2P_REGULAR, PIXEL_PER_GRID * 0.5, Color::ORANGE ),
	( ""     , FONT_PRESSSTART2P_REGULAR, PIXEL_PER_GRID * 0.9, Color::WHITE  ),
];

#[ derive( Component ) ]
pub struct UiUpperCenter;
const UI_UPPER_CENTER: [ MessageSect; 2 ] =
[	( "YOUR", FONT_PRESSSTART2P_REGULAR, PIXEL_PER_GRID * 0.5, Color::ORANGE ),
	( ""    , FONT_PRESSSTART2P_REGULAR, PIXEL_PER_GRID * 0.9, Color::WHITE  ),
//	( ""    , FONT_PRESSSTART2P_REGULAR, PIXEL_PER_GRID * 0.4, Color::SILVER ),
];

#[ derive( Component ) ]
pub struct UiUpperRight;
const UI_UPPER_RIGHT: [ MessageSect; 2 ] =
[	( "HIGH", FONT_PRESSSTART2P_REGULAR, PIXEL_PER_GRID * 0.5, Color::ORANGE ),
	( ""    , FONT_PRESSSTART2P_REGULAR, PIXEL_PER_GRID * 0.9, Color::WHITE  ),
];

//下端
#[ derive( Component ) ]
pub struct UiLowerLeft;
const UI_LOWER_LEFT: [ MessageSect; 2 ] =
[	( "FPS", FONT_REGGAEONE_REGULAR, PIXEL_PER_GRID * 1.0, Color::ORANGE ),
	( ""   , FONT_REGGAEONE_REGULAR, PIXEL_PER_GRID * 1.4, Color::WHITE  ),
];

#[ derive( Component ) ]
pub struct UiLowerCenter;
const UI_LOWER_CENTER: [ MessageSect; 0 ] =
[//	( "鬼ごっこ", FONT_REGGAEONE_REGULAR, PIXEL_PER_GRID * 2.0, Color::WHITE  ),
//	( APP_TITLE, FONT_REGGAEONE_REGULAR, PIXEL_PER_GRID * 1.0, Color::ORANGE ),
];

#[ derive( Component ) ]
pub struct UiLowerRight;
const UI_LOWER_RIGHT: [ MessageSect; 1 ] =
[	( "2021 hyoi", FONT_REGGAEONE_REGULAR, PIXEL_PER_GRID * 0.7, Color::WHITE ),
];

////////////////////////////////////////////////////////////////////////////////////////////////////

//End of code.