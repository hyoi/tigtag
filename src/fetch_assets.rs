use super::*;

//external modules
use rand::prelude::*;

//Pluginの手続き
pub struct PluginFetchAssets;
impl Plugin for PluginFetchAssets
{	fn build( &self, app: &mut App )
	{	app
		//------------------------------------------------------------------------------------------
		.add_system_set													// ＜GameState::Init＞
		(	SystemSet::on_enter( GameState::Init )						// ＜on_enter()＞
				.with_system( start_fetching_assets )					// Assetの事前ロード開始
				.with_system( spawn_preload_anime_tile )				// ローディングアニメ用スプライトの生成
		)
		.add_system_set													// ＜GameState::Init＞
		(	SystemSet::on_update( GameState::Init )						// ＜on_update()＞
				.with_system( change_state_after_loading )				// ロード完了⇒GameState::DemoStartへ
				.with_system( move_preload_anime_tile )					// ローディングアニメ
		)
		.add_system_set													// ＜GameState::Init＞
		(	SystemSet::on_exit( GameState::Init )						// ＜on_exit()＞
				.with_system( spawn_text_ui_message )					// プリロード後にUIを生成
				.with_system( despawn_entity::<PreloadingAnimeTile> )	// スプライトの削除
		)
		//------------------------------------------------------------------------------------------
		;
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//定義と定数

const FETCH_ASSETS: [ &str; 5 ] =
[	FONT_PRESSSTART2P_REGULAR,
	FONT_ORBITRON_BLACK,
	FONT_REGGAEONE_REGULAR,
	IMAGE_SPRITE_WALL,
	SOUND_BEEP,
];

//ロードしたAssetのハンドルの保存先
struct LoadedAssets { preload: Vec<HandleUntyped> }

//ローディングアニメ関係
const PRELOADING_MESSAGE_ARRAY: [ &str; 13 ] = 
[//	 0123456789 123456789 123456789 123456789 12345
	" ##  #           #                            ", //0
	" ##  # ### #   # #    ###  #  ##  # #  #  ##  ", //1
	" # # # # # # # # #    # # # # # #   ## # #    ", //2
	" # # # # # # # # #    # # # # # # # #### # ## ", //3
	" #  ## # #  # #  #    # # ### # # # # ## #  # ", //4
	" #  ## ###  # #  #### ### # # ##  # #  #  ##  ", //5
	"",												  //6
	" ###                      #   #               ", //7
	" #  # #   ###  #  ### ### # # #  #  # ### # # ", //8
	" #  # #   #   # # #   #   # # # # #    #  # # ", //9
	" ###  #   ### # # ### ### # # # # # #  #  # # ", //10
	" #    #   #   ###   # #    # #  ### #  #      ", //11
	" #    ### ### # # ### ###  # #  # # #  #  # # ", //12
];

//スプライト識別用Component
#[derive(Component)]
struct PreloadingAnimeTile ( usize, usize );

//タイルのスプライト
const SPRITE_TILE_DEPTH: f32   = 0.0;
const SPRITE_TILE_PIXEL: f32   = PIXEL_PER_GRID;
const SPRITE_TILE_COLOR: Color = Color::rgb_linear( 0.25, 0.06, 0.04 );

////////////////////////////////////////////////////////////////////////////////////////////////////

//Assetの事前ロードを開始する
fn start_fetching_assets
(	mut cmds: Commands,
	asset_svr: Res<AssetServer>,
)
{	//Assetのロードを開始
	let mut preload = Vec::new();
	FETCH_ASSETS.iter().for_each( | f | preload.push( asset_svr.load_untyped( *f ) ) );

	cmds.insert_resource( LoadedAssets { preload } );
}

//Assetのロードが完了したら、Stateを変更する
fn change_state_after_loading
(	mut state : ResMut<State<GameState>>,
	assets: Res<LoadedAssets>,
	asset_svr: Res<AssetServer>,
)
{	for handle in assets.preload.iter()
	{	use bevy::asset::LoadState::*;
		match asset_svr.get_load_state( handle )
		{	Loaded => {}
			Failed => panic!(),	//ロードエラー⇒パニック
			_      => return,	//on_update()なので繰り返し関数が呼び出される
		}
	}

	//DemoStartへ遷移する
	let _ = state.overwrite_set( GameState::DemoStart );
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//ローディングアニメ用スプライトを生成する
fn spawn_preload_anime_tile( mut cmds: Commands )
{	let mut rng = rand::thread_rng();

	for ( grid_y, s ) in PRELOADING_MESSAGE_ARRAY.iter().enumerate()
	{	for ( grid_x, c ) in s.chars().enumerate()
		{	if c == ' ' { continue }	//空白文字は無視

			//スプライトの初期位置は乱数で決める
			let x  = rng.gen_range( 0..MAP_WIDTH  );
			let y  = rng.gen_range( 0..MAP_HEIGHT );
			let xy = conv_sprite_coordinates( x, y );

			cmds.spawn_bundle( sprite_preloading_anime_tile( xy ) )
				.insert( PreloadingAnimeTile ( grid_x, grid_y ) );
		} 
	}
}

//スプライトを動かしてローディングアニメを見せる
fn move_preload_anime_tile
(	mut q: Query<( &mut Transform, &PreloadingAnimeTile )>,
	time: Res<Time>,
)
{	let time_delta = time.delta().as_secs_f32() * 5.0;

	let half_screen_w = SCREEN_WIDTH / 2.0;
	let mess_width = PRELOADING_MESSAGE_ARRAY[ 0 ].len() as f32 * SPRITE_TILE_PIXEL;
	let scale =  SCREEN_WIDTH / mess_width;

	q.for_each_mut
	(	| ( mut transform, tile ) |
		{	let position = &mut transform.translation;
			let ( grid_x, grid_y ) = ( tile.0 , tile.1 );
			let ( goal_x, goal_y ) = conv_sprite_coordinates( grid_x, grid_y );

			//横幅の調整
			let goal_x = ( goal_x + half_screen_w ) * scale - half_screen_w;

			position.x += ( goal_x - position.x ) * time_delta;
			position.y += ( goal_y - position.y ) * time_delta;
		}
	);
}

//ローディングアニメ用スプライトのバンドルを生成
fn sprite_preloading_anime_tile( ( x, y ): ( f32, f32 ) ) -> SpriteBundle
{	let position  = Vec3::new( x, y, SPRITE_TILE_DEPTH );
	let square    = Vec2::new( SPRITE_TILE_PIXEL, SPRITE_TILE_PIXEL );
	let transform = Transform::from_translation( position );
	let sprite    = Sprite
	{	color: SPRITE_TILE_COLOR,
		custom_size: Some( square ),
		..default()
	};

	SpriteBundle { transform, sprite, ..default() }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//テキストUIを配置する
pub fn spawn_text_ui_message( mut cmds: Commands, asset_svr: Res<AssetServer> )
{	//中央に表示するtext
	let mut demo_text  = text_ui( &MESSAGE_DEMO , &asset_svr );
	let mut start_text = text_ui( &MESSAGE_START, &asset_svr );
	let mut clear_text = text_ui( &MESSAGE_CLEAR, &asset_svr );
	let mut over_text  = text_ui( &MESSAGE_OVER , &asset_svr );
	let mut pause_text = text_ui( &MESSAGE_PAUSE, &asset_svr );

	demo_text.visibility.is_visible  = false;
	start_text.visibility.is_visible = false;
	clear_text.visibility.is_visible = false;
	over_text.visibility.is_visible  = false;
	pause_text.visibility.is_visible = false;

	//上端・下端に表示するtext
	let mut ui_upper_left   = text_ui( &UI_UPPER_LEFT  , &asset_svr );
	let mut ui_lower_left   = text_ui( &UI_LOWER_LEFT  , &asset_svr );
	let mut ui_upper_center = text_ui( &UI_UPPER_CENTER, &asset_svr );
	let mut ui_lower_center = text_ui( &UI_LOWER_CENTER, &asset_svr );
	let mut ui_upper_right  = text_ui( &UI_UPPER_RIGHT , &asset_svr );
	let mut ui_lower_right  = text_ui( &UI_LOWER_RIGHT , &asset_svr );

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

	//レイアウト用の隠しフレームを作る
	let per100 = Val::Percent( 100.0 );
	let center_frame = hidden_frame( Style
	{	size           : Size::new( per100, per100 ),
		position_type  : PositionType::Absolute,
		justify_content: JustifyContent::Center,
		align_items    : AlignItems::Center,
		..default()
	} );
	let upper_frame  = hidden_frame( Style
	{	size           : Size::new( Val::Px( SCREEN_WIDTH ), Val::Px( SCREEN_HEIGHT ) ),
		position_type  : PositionType::Absolute,
		flex_direction : FlexDirection::Column,
		justify_content: JustifyContent::FlexEnd, //画面の上端
		..default()
	} );
	let lower_frame  = hidden_frame( Style
	{	size           : Size::new( Val::Px( SCREEN_WIDTH ), Val::Px( SCREEN_HEIGHT ) ),
		position_type  : PositionType::Absolute,
		flex_direction : FlexDirection::Column,
		justify_content: JustifyContent::FlexStart, //画面の下端
		..default()
	} );

	//隠しフレームの上に子要素を作成する
	let timer = Timer::from_seconds( 1.0, false );
	cmds.spawn_bundle( center_frame ).with_children( | cmds |
	{	cmds.spawn_bundle( demo_text  ).insert( MessageDemo  );
		cmds.spawn_bundle( start_text ).insert( MessageStart { count: 0, timer: timer.clone() } );
		cmds.spawn_bundle( clear_text ).insert( MessageClear { count: 0, timer: timer.clone() } );
		cmds.spawn_bundle( over_text  ).insert( MessageOver  { count: 0, timer: timer.clone() } );
		cmds.spawn_bundle( pause_text ).insert( MessagePause );

		cmds.spawn_bundle( upper_frame ).with_children( | cmds |
		{	cmds.spawn_bundle( ui_upper_left   ).insert( UiUpperLeft   );
			cmds.spawn_bundle( ui_upper_center ).insert( UiUpperCenter );
			cmds.spawn_bundle( ui_upper_right  ).insert( UiUpperRight  );
		} );

		cmds.spawn_bundle( lower_frame ).with_children( | cmds |
		{	cmds.spawn_bundle( ui_lower_left   ).insert( UiLowerLeft   );
			cmds.spawn_bundle( ui_lower_center ).insert( UiLowerCenter );
			cmds.spawn_bundle( ui_lower_right  ).insert( UiLowerRight  );
		} );
	} );
}

//TextBundleを作る
fn text_ui( message: &[ MessageSect ], asset_svr: &Res<AssetServer> ) -> TextBundle
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
	let style = Style { position_type, ..default() };

	TextBundle { style, text, ..default() }
}

//レイアウト用に隠しフレームを作る
fn hidden_frame( style: Style ) -> NodeBundle
{	let color = UiColor ( Color::NONE );

    NodeBundle { style, color, ..default() }
}

//End of code.