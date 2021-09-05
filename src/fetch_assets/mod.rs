use super::*;

mod spawn_text_ui;
pub use spawn_text_ui::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

//Pluginの手続き
pub struct PluginFetchAssets;
impl Plugin for PluginFetchAssets
{	fn build( &self, app: &mut AppBuilder )
	{	app
		//------------------------------------------------------------------------------------------
		.add_system_set													// ＜GameState::Init＞
		(	SystemSet::on_enter( GameState::Init )						// ＜on_enter()＞
				.with_system( start_fetching_assets.system() )			// Assetの事前ロード開始
				.with_system( spawn_preload_anime_tile.system() )		// ローディングアニメ用スプライトの生成
		)
		.add_system_set													// ＜GameState::Init＞
		(	SystemSet::on_update( GameState::Init )						// ＜on_update()＞
				.with_system( change_state_after_loading.system() )		// ロード完了⇒GameState::DemoStartへ
				.with_system( move_preload_anime_tile.system() )		// ローディングアニメ
		)
		.add_system_set													// ＜GameState::Init＞
		(	SystemSet::on_exit( GameState::Init )						// ＜on_exit()＞
				.with_system( despawn_preloading_anime_tile.system() )	// ローディングアニメ用スプライトの削除
		)
		//------------------------------------------------------------------------------------------
		.add_system_set													// ＜GameState::Init＞
		(	SystemSet::on_exit( GameState::Init )						// ＜on_exit()＞
				.with_system( spawn_text_ui_message.system() )			// assetesプリロード後にUIを非表示で生成
		)
		//------------------------------------------------------------------------------------------
		;
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//定義と定数

//事前ロード対象のAsset
pub const FONT_PRESSSTART2P_REGULAR: &str = "fonts/PressStart2P-Regular.ttf";
pub const FONT_ORBITRON_BLACK	   : &str = "fonts/Orbitron-Black.ttf";
pub const FONT_REGGAEONE_REGULAR   : &str = "fonts/ReggaeOne-Regular.ttf";
pub const IMAGE_SPRITE_WALL		   : &str = "sprites/wall.png";

const FETCH_ASSETS: [ &str; 4 ] =
[	FONT_PRESSSTART2P_REGULAR,
	FONT_ORBITRON_BLACK,
	FONT_REGGAEONE_REGULAR,
	IMAGE_SPRITE_WALL,
];

//ロードしたAssetのハンドルの保存先
struct LoadedAssets { preload: Vec<HandleUntyped> }

//ローディングアニメ関係
const PRELOADING_MESSAGE_SCALE: f32 = 0.7;
const PRELOADING_MESSAGE_ARRAY: [ &str; 13 ] = 
[//	 0123456789 123456789 123456789 123456789 123
	"##  #           #                           ", //0
	"##  # ### #   # #    ###  #  ##  # #  #  ## ", //1
	"# # # # # # # # #    # # # # # #   ## # #   ", //2
	"# # # # # # # # #    # # # # # # # #### # ##", //3
	"#  ## # #  # #  #    # # ### # # # # ## #  #", //4
	"#  ## ###  # #  #### ### # # ##  # #  #  ## ", //5
	"",
	"###                      #   #              ", //7
	"#  # #   ###  #  ### ### # # #  #  # ### # #", //8
	"#  # #   #   # # #   #   # # # # #    #  # #", //9
	"###  #   ### # # ### ### # # # # # #  #  # #", //10
	"#    #   #   ###   # #    # #  ### #  #     ", //11
	"#    ### ### # # ### ###  # #  # # #  #  # #", //12
];

//スプライト識別用Component
struct PreloadingAnimeTile ( usize, usize );

//タイルのスプライト
const SPRITE_TILE_PIXEL: f32   = PIXEL_PER_GRID - 1.0;
const SPRITE_TILE_COLOR: Color = Color::rgb( 0.25, 0.06, 0.04 );
const SPRITE_TILE_DEPTH: f32   = 0.0;

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
	mut color_matl: ResMut<Assets<ColorMaterial>>,
)
{	let mut rng = rand::thread_rng();

	for ( map_y, s ) in PRELOADING_MESSAGE_ARRAY.iter().enumerate()
	{	for ( map_x, c ) in s.chars().enumerate()
		{	if c == ' ' { continue }	//空白文字は無視

			//スプライトの初期位置は乱数で決める
			let x  = rng.gen_range( 0..MAP_WIDTH  );
			let y  = rng.gen_range( 0..MAP_HEIGHT );
			let xy = conv_sprite_coordinates( x, y );

			cmds.spawn_bundle( sprite_preloading_anime_tile( xy, &mut color_matl ) )
				.insert( PreloadingAnimeTile ( map_x, map_y ) );
		} 
	}
}

//スプライトを動かしてローディングアニメを見せる
fn move_preload_anime_tile
(	q: Query<( &mut Transform, &PreloadingAnimeTile )>,
	time: Res<Time>,
)
{	let time_delta = time.delta().as_secs_f32() * 5.0;
	q.for_each_mut
	(	| ( mut transform, tile ) |
		{	let locate = &mut transform.translation;
			let ( goal_x, goal_y ) = conv_sprite_coordinates( tile.0, tile.1 );

			locate.x += ( goal_x * PRELOADING_MESSAGE_SCALE - locate.x ) * time_delta;
			locate.y += ( goal_y * PRELOADING_MESSAGE_SCALE - locate.y ) * time_delta;
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
	color_matl: &mut ResMut<Assets<ColorMaterial>>,
) -> SpriteBundle
{	let position = Vec3::new( x, y, SPRITE_TILE_DEPTH );
	let square   = Vec2::new( SPRITE_TILE_PIXEL, SPRITE_TILE_PIXEL );

	SpriteBundle
	{	material : color_matl.add( SPRITE_TILE_COLOR.into() ),
		transform: Transform::from_translation( position ),
		sprite   : Sprite::new( square ),
		..Default::default()
	}
}

//End of code.