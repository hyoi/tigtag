use super::*;

//Textの情報をまとめたタプルのエイリアス
type MessageSect<'a> = ( &'a str, &'a str, f32, Color );

////////////////////////////////////////////////////////////////////////////////

//Header UI Left（データ表示枠付き）
const HEADER_UI_LEFT: [ MessageSect; 2 ] =
[	( "FPS", UI_FONT_FILE, 0.7, Color::ORANGE ),
	( ""   , UI_FONT_FILE, 0.9, Color::WHITE  ),
];

//Header UI Center（データ表示枠２つ付き）
const HEADER_UI_CENTER: [ MessageSect; 3 ] =
[	( "Score", UI_FONT_FILE, 0.7, Color::ORANGE ),
	( ""     , UI_FONT_FILE, 0.9, Color::WHITE  ),
	( ""     , UI_FONT_FILE, 0.6, Color::SILVER ),
];

//Header UI Right（データ表示枠付き）
const HEADER_UI_RIGHT: [ MessageSect; 2 ] =
[	( "High-Score", UI_FONT_FILE, 0.7, Color::ORANGE ),
	( ""          , UI_FONT_FILE, 0.9, Color::WHITE  ),
];

//DemoPlayのメッセージ（タイトル）
const MESSAGE_DEMO: [ MessageSect; 4 ] =
[	( "TigTag\n\n"    , UI_FONT_FILE, 4., Color::rgb_linear( 0.3, 1.0, 0.1 ) ),
	( "D E M O\n\n"   , UI_FONT_FILE, 1., Color::YELLOW ),
	( "Game Start\n\n", UI_FONT_FILE, 1., Color::CYAN   ),
	( "Hit SPACE Key" , UI_FONT_FILE, 1., Color::CYAN   ),
];

//GameStartのメッセージ（データ表示枠のみ）
const MESSAGE_START: [ MessageSect; 1 ] =
[	( "", COUNTDOWN_FONT_FILE, 5., Color::CYAN ),
];

//GameClearのメッセージ（データ表示枠付き）
const MESSAGE_CLEAR: [ MessageSect; 2 ] =
[	( "Stage Clear\n\n", UI_FONT_FILE       , 2., Color::GOLD ),
	( ""               , COUNTDOWN_FONT_FILE, 4., Color::GOLD ),
];

//GameOverのメッセージ（データ表示枠付き）
const MESSAGE_OVER: [ MessageSect; 3 ] =
[	( "Game Over\n\n", UI_FONT_FILE, 2., Color::RED ),
	( "Replay?\n\nHit SPACE Key\n", UI_FONT_FILE, 1., Color::YELLOW ),
	( "", COUNTDOWN_FONT_FILE, 4., Color::CYAN ),
];

//一時停止のメッセージ
const MESSAGE_PAUSE: [ MessageSect; 1 ] =
[	( "P A U S E", COUNTDOWN_FONT_FILE, 5., Color::WHITE ),
];

////////////////////////////////////////////////////////////////////////////////

//Ui Header Leftのテキストを生成
pub fn header_ui_left( asset_svr: &Res<AssetServer> ) -> TextBundle
{	text_messsage( &HEADER_UI_LEFT, asset_svr )
}

//Ui Header Centerのテキストを生成
pub fn header_ui_center( asset_svr: &Res<AssetServer> ) -> TextBundle
{	text_messsage( &HEADER_UI_CENTER, asset_svr )
}

//Ui Header Rightのテキストを生成
pub fn header_ui_right( asset_svr: &Res<AssetServer> ) -> TextBundle
{	text_messsage( &HEADER_UI_RIGHT, asset_svr )
}

//DemoPlayのテキストを生成
pub fn ui_text_demo( asset_svr: &Res<AssetServer> ) -> TextBundle
{	text_messsage( &MESSAGE_DEMO, asset_svr )
}

//GameStartのテキストを生成
pub fn ui_text_start( asset_svr: &Res<AssetServer> ) -> TextBundle
{	text_messsage( &MESSAGE_START, asset_svr )
}

//GameClearのテキストを生成
pub fn ui_text_clear( asset_svr: &Res<AssetServer> ) -> TextBundle
{	text_messsage( &MESSAGE_CLEAR, asset_svr )
}

//GameOverのテキストを生成
pub fn ui_text_over( asset_svr: &Res<AssetServer> ) -> TextBundle
{	text_messsage( &MESSAGE_OVER, asset_svr )
}

//一時停止のテキストを生成
pub fn ui_text_pause( asset_svr: &Res<AssetServer> ) -> TextBundle
{	text_messsage( &MESSAGE_PAUSE, asset_svr )
}

////////////////////////////////////////////////////////////////////////////////

//TextBundleを作る
pub fn text_messsage
(	message: &[ MessageSect ],
	asset_svr: &Res<AssetServer>,
) -> TextBundle
{	//TextSectionのVecを作る
	let mut sections = Vec::new();
	for ( mess, font, size, color ) in message.iter()
	{	let section = TextSection
		{	value: mess.to_string(),
			style: TextStyle
			{	font     : asset_svr.load( *font ),
				font_size: PIXEL_PER_GRID * size,
				color    : *color,
			}
		};
		sections.push( section );
	}

	//戻り値
	TextBundle
	{	style: Style
		{	position_type: PositionType::Absolute,
			..Default::default()
		},
		text: Text
		{	sections,
			alignment: TextAlignment
			{	vertical  : VerticalAlign::Center,
				horizontal: HorizontalAlign::Center,
			}
		},
		..Default::default()
	}
}

////////////////////////////////////////////////////////////////////////////////

//フルスクリーンの画面サイズに対し縦横100%の隠しフレーム
//※Flexboxのセンタリング用
pub fn frame_full_screen() -> NodeBundle
{	let per100 = Val::Percent( 100. );
	NodeBundle
	{	style: Style
		{	size: Size::new( per100, per100 ),
			position_type  : PositionType::Absolute,
			justify_content: JustifyContent::Center,
			align_items    : AlignItems::Center,
			..Default::default()
		},
		visible: Visible { is_visible: false, ..Default::default() },
		..Default::default()
	}
}

//アプリのウィンドウ(ゲーム画面)と同じサイズの隠しフレーム
//※Header UIをゲーム画面上端に寄せるために使用
pub fn frame_map_size() -> NodeBundle
{	let width  = Val::Px( SCREEN_WIDTH  );
	let height = Val::Px( SCREEN_HEIGHT );
	NodeBundle
	{	style: Style
		{	size: Size::new( width, height ),
			flex_direction : FlexDirection::Column,
			justify_content: JustifyContent::FlexEnd, //画面の上端
			..Default::default()
		},
		visible: Visible { is_visible: false, ..Default::default() },
		..Default::default()
	}
}

//End of code.