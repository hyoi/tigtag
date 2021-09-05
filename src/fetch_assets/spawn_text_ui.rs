use super::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

//定義と定数

type MessageSect<'a> = ( &'a str, &'a str, f32, Color );

//中央
pub struct MessageDemo;
const MESSAGE_DEMO: [ MessageSect; 4 ] =
[	( "TigTag\n\n"    , FONT_PRESSSTART2P_REGULAR, PIXEL_PER_GRID * 3.5, Color::rgb_linear( 0.3, 1.0, 0.1 ) ),
	( "D E M O\n\n"   , FONT_PRESSSTART2P_REGULAR, PIXEL_PER_GRID * 1.0, Color::YELLOW ),
	( "Game Start\n\n", FONT_PRESSSTART2P_REGULAR, PIXEL_PER_GRID * 1.0, Color::CYAN   ),
	( "Hit SPACE Key" , FONT_PRESSSTART2P_REGULAR, PIXEL_PER_GRID * 1.0, Color::CYAN   ),
];

pub struct MessageStart
{	pub count: usize,
	pub timer: Timer,
}
const MESSAGE_START: [ MessageSect; 1 ] =
[	( "", FONT_ORBITRON_BLACK, PIXEL_PER_GRID * 4.0, Color::CYAN ),
];
pub const COUNTDOWN_TEXT: [ &str; 4 ] = [ "Go!", "Ready...\n1", "Ready...\n2", "Ready...\n3" ];

pub struct MessageClear
{	pub count: usize,
	pub timer: Timer,
}
const MESSAGE_CLEAR: [ MessageSect; 3 ] =
[	( "Clear!!"            , FONT_ORBITRON_BLACK      , PIXEL_PER_GRID * 6.0, Color::YELLOW ),
	( "\nNext stage...\n\n", FONT_PRESSSTART2P_REGULAR, PIXEL_PER_GRID * 2.0, Color::WHITE  ),
	( ""                   , FONT_ORBITRON_BLACK      , PIXEL_PER_GRID * 4.0, Color::WHITE  ),
];
pub const GAMECLEAR_COUNTDOWN: usize = 10;

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

pub struct MessagePause;
const MESSAGE_PAUSE: [ MessageSect; 1 ] =
[	( "P A U S E", FONT_ORBITRON_BLACK, PIXEL_PER_GRID * 4.0, Color::WHITE ),
];

//上端
pub struct UiUpperLeft;
const UI_UPPER_LEFT: [ MessageSect; 2 ] =
[	( "STAGE", FONT_PRESSSTART2P_REGULAR, PIXEL_PER_GRID * 0.5, Color::ORANGE ),
	( ""     , FONT_PRESSSTART2P_REGULAR, PIXEL_PER_GRID * 0.9, Color::WHITE  ),
];

pub struct UiUpperCenter;
const UI_UPPER_CENTER: [ MessageSect; 2 ] =
[	( "YOUR", FONT_PRESSSTART2P_REGULAR, PIXEL_PER_GRID * 0.5, Color::ORANGE ),
	( ""    , FONT_PRESSSTART2P_REGULAR, PIXEL_PER_GRID * 0.9, Color::WHITE  ),
//	( ""    , FONT_PRESSSTART2P_REGULAR, PIXEL_PER_GRID * 0.4, Color::SILVER ),
];

pub struct UiUpperRight;
const UI_UPPER_RIGHT: [ MessageSect; 2 ] =
[	( "HIGH", FONT_PRESSSTART2P_REGULAR, PIXEL_PER_GRID * 0.5, Color::ORANGE ),
	( ""    , FONT_PRESSSTART2P_REGULAR, PIXEL_PER_GRID * 0.9, Color::WHITE  ),
];

//下端
pub struct UiLowerLeft;
const UI_LOWER_LEFT: [ MessageSect; 2 ] =
[	( "FPS", FONT_REGGAEONE_REGULAR, PIXEL_PER_GRID * 1.0, Color::ORANGE ),
	( ""   , FONT_REGGAEONE_REGULAR, PIXEL_PER_GRID * 1.4, Color::WHITE  ),
];

pub struct UiLowerCenter;
const UI_LOWER_CENTER: [ MessageSect; 0 ] =
[//	( "鬼ごっこ", FONT_REGGAEONE_REGULAR, PIXEL_PER_GRID * 2.0, Color::WHITE  ),
//	( APP_TITLE, FONT_REGGAEONE_REGULAR, PIXEL_PER_GRID * 1.0, Color::ORANGE ),
];

pub struct UiLowerRight;
const UI_LOWER_RIGHT: [ MessageSect; 1 ] =
[	( "© 2021 hyoi", FONT_REGGAEONE_REGULAR, PIXEL_PER_GRID * 1.0, Color::WHITE ),
];

////////////////////////////////////////////////////////////////////////////////////////////////////

//テキストUIを配置する
pub fn spawn_text_ui_message( mut cmds: Commands, asset_svr: Res<AssetServer> )
{	//中央に表示するtext
	let mut demo_text  = text_messsage( &MESSAGE_DEMO , &asset_svr );
	let mut start_text = text_messsage( &MESSAGE_START, &asset_svr );
	let mut clear_text = text_messsage( &MESSAGE_CLEAR, &asset_svr );
	let mut over_text  = text_messsage( &MESSAGE_OVER , &asset_svr );
	let mut pause_text = text_messsage( &MESSAGE_PAUSE, &asset_svr );

	//初期は非表示
	demo_text.visible.is_visible  = false;
	start_text.visible.is_visible = false;
	clear_text.visible.is_visible = false;
	over_text.visible.is_visible  = false;
	pause_text.visible.is_visible = false;

	//上端・下端に表示するtext
	let mut ui_upper_left   = text_messsage( &UI_UPPER_LEFT  , &asset_svr );
	let mut ui_lower_left   = text_messsage( &UI_LOWER_LEFT  , &asset_svr );
	let mut ui_upper_center = text_messsage( &UI_UPPER_CENTER, &asset_svr );
	let mut ui_lower_center = text_messsage( &UI_LOWER_CENTER, &asset_svr );
	let mut ui_upper_right  = text_messsage( &UI_UPPER_RIGHT , &asset_svr );
	let mut ui_lower_right  = text_messsage( &UI_LOWER_RIGHT , &asset_svr );

	ui_upper_left.style.align_self   = AlignSelf::FlexStart;
	ui_lower_left.style.align_self   = AlignSelf::FlexStart;
	ui_upper_center.style.align_self = AlignSelf::Center;
	ui_lower_center.style.align_self = AlignSelf::Center;
	ui_upper_right.style.align_self  = AlignSelf::FlexEnd;
	ui_lower_right.style.align_self  = AlignSelf::FlexEnd;

	ui_upper_left.text.alignment.horizontal   = HorizontalAlign::Left;
	ui_lower_left.text.alignment.horizontal   = HorizontalAlign::Left;
	ui_upper_center.text.alignment.horizontal = HorizontalAlign::Center;
	ui_lower_center.text.alignment.horizontal = HorizontalAlign::Center;
	ui_upper_right.text.alignment.horizontal  = HorizontalAlign::Right;
	ui_lower_right.text.alignment.horizontal  = HorizontalAlign::Right;

	//隠しフレームの上に子要素を作成する
	let timer = Timer::from_seconds( 1.0, false );
	cmds.spawn_bundle( hidden_frame_for_centering() ).with_children( | cmds |
	{	cmds.spawn_bundle( demo_text  ).insert( MessageDemo  );
		cmds.spawn_bundle( start_text ).insert( MessageStart { count: 0, timer: timer.clone() } );
		cmds.spawn_bundle( clear_text ).insert( MessageClear { count: 0, timer: timer.clone() } );
		cmds.spawn_bundle( over_text  ).insert( MessageOver  { count: 0, timer: timer.clone() } );
		cmds.spawn_bundle( pause_text ).insert( MessagePause );

		cmds.spawn_bundle( hidden_upper_frame() ).with_children( | cmds |
		{	cmds.spawn_bundle( ui_upper_left   ).insert( UiUpperLeft   );
			cmds.spawn_bundle( ui_upper_center ).insert( UiUpperCenter );
			cmds.spawn_bundle( ui_upper_right  ).insert( UiUpperRight  );
		} );

		cmds.spawn_bundle( hidden_lower_frame() ).with_children( | cmds |
		{	cmds.spawn_bundle( ui_lower_left   ).insert( UiLowerLeft   );
			cmds.spawn_bundle( ui_lower_center ).insert( UiLowerCenter );
			cmds.spawn_bundle( ui_lower_right  ).insert( UiLowerRight  );
		} );
	} );
}

//TextBundleを作る
fn text_messsage( message: &[ MessageSect ], asset_svr: &Res<AssetServer> ) -> TextBundle
{	let mut sections = Vec::new();
	for ( line, file, size, color ) in message
	{	let value = line.to_string();
		let style = TextStyle
		{	font     : asset_svr.load( *file ),
			font_size: *size,
			color    : *color
		};
		sections.push( TextSection { value, style } );
	}

	let alignment = TextAlignment
	{	vertical  : VerticalAlign::Center,
		horizontal: HorizontalAlign::Center,
	};
	let position_type = PositionType::Absolute;

	let text  = Text { sections, alignment };
	let style = Style { position_type, ..Default::default() };
	TextBundle { style, text, ..Default::default() }
}

//中央寄せ用の隠しフレーム
fn hidden_frame_for_centering() -> NodeBundle
{	let per100 = Val::Percent( 100.0 );
	let style = Style
	{	size           : Size::new( per100, per100 ),
		position_type  : PositionType::Absolute,
		justify_content: JustifyContent::Center,
		align_items    : AlignItems::Center,
		..Default::default()
	};
	let visible = Visible { is_visible: false, ..Default::default() };
	NodeBundle { style, visible, ..Default::default() }
}

//上端幅合せ用の隠しフレーム
fn hidden_upper_frame() -> NodeBundle
{	let style = Style
	{	size           : Size::new( Val::Px( SCREEN_WIDTH ), Val::Px( SCREEN_HEIGHT ) ),
		position_type  : PositionType::Absolute,
		flex_direction : FlexDirection::Column,
		justify_content: JustifyContent::FlexEnd, //画面の上端
		..Default::default()
	};
	let visible = Visible { is_visible: false, ..Default::default() };
	NodeBundle { style, visible, ..Default::default() }
}

//下端幅合せ用の隠しフレーム
fn hidden_lower_frame() -> NodeBundle
{	let style = Style
	{	size           : Size::new( Val::Px( SCREEN_WIDTH ), Val::Px( SCREEN_HEIGHT ) ),
		position_type  : PositionType::Absolute,
		flex_direction : FlexDirection::Column,
		justify_content: JustifyContent::FlexStart, //画面の下端
		..Default::default()
	};
	let visible = Visible { is_visible: false, ..Default::default() };
	NodeBundle { style, visible, ..Default::default() }
}

//End of code.