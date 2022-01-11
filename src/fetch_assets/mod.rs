use super::*;

mod spawn_text_ui;
pub use spawn_text_ui::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

//Pluginの手続き
pub struct PluginFetchAssets;
impl Plugin for PluginFetchAssets
{	fn build( &self, app: &mut App )
	{	app
		//------------------------------------------------------------------------------------------
		.add_system_set											// ＜GameState::Init＞
		(	SystemSet::on_enter( GameState::Init )				// ＜on_enter()＞
				.with_system( start_fetching_assets )			// Assetの事前ロード開始
				.with_system( spawn_preload_anime_tile )		// ローディングアニメ用スプライトの生成
		)
		.add_system_set											// ＜GameState::Init＞
		(	SystemSet::on_update( GameState::Init )				// ＜on_update()＞
				.with_system( change_state_after_loading )		// ロード完了⇒GameState::DemoStartへ
				.with_system( move_preload_anime_tile )			// ローディングアニメ
		)
		.add_system_set											// ＜GameState::Init＞
		(	SystemSet::on_exit( GameState::Init )				// ＜on_exit()＞
				.with_system( despawn_preloading_anime_tile )	// ローディングアニメ用スプライトの削除
		)
		//------------------------------------------------------------------------------------------
		.add_system_set											// ＜GameState::Init＞
		(	SystemSet::on_exit( GameState::Init )				// ＜on_exit()＞
				.with_system( spawn_text_ui_message )			// assetesプリロード後にUIを非表示で生成
		)
		//------------------------------------------------------------------------------------------
		;
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//定義と定数

const FETCH_ASSETS: [ &str; 4 ] =
[	FONT_PRESSSTART2P_REGULAR,
	FONT_ORBITRON_BLACK,
	FONT_REGGAEONE_REGULAR,
	IMAGE_SPRITE_WALL,
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
#[ derive( Component ) ]
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

	FETCH_ASSETS.iter()
		.for_each( | f | preload.push( asset_svr.load_untyped( *f ) ) );

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
fn spawn_preload_anime_tile
(	mut cmds: Commands,
)
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

//ローディングアニメ用スプライトを削除する
fn despawn_preloading_anime_tile
(	q: Query<Entity, With<PreloadingAnimeTile>>,
	mut cmds: Commands,
)
{	q.for_each( | id | { cmds.entity( id ).despawn() } );
}

//ローディングアニメ用スプライトのバンドルを生成
fn sprite_preloading_anime_tile
(	( x, y ): ( f32, f32 ),
) -> SpriteBundle
{	let position = Vec3::new( x, y, SPRITE_TILE_DEPTH );
	let square   = Vec2::new( SPRITE_TILE_PIXEL, SPRITE_TILE_PIXEL );

	let transform = Transform::from_translation( position );
	let sprite    = Sprite
	{	color: SPRITE_TILE_COLOR,
		custom_size: Some( square ),
		..Default::default()
	};

	SpriteBundle { transform, sprite, ..Default::default() }
}

//End of code.