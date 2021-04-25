use super::*;

//Assets
pub const UI_FONT_FILE       : &str = "fonts/PressStart2P-Regular.ttf";
pub const COUNTDOWN_FONT_FILE: &str = "fonts/Orbitron-Black.ttf";
pub const SPRITE_WALL_FILE   : &str = "sprites/wall.png";

//ゲーム開始前にローディングを完了させておくアセット
const PRELOADING_ASSET_FILES: [ &str; 3 ] =
[	UI_FONT_FILE,
	COUNTDOWN_FONT_FILE,
	SPRITE_WALL_FILE,
];

//Resource
#[derive(Default)]
pub struct LoadedHandles ( Vec<HandleUntyped> );

//Assetの事前ロードを開始する
pub fn start_preloading_assets
(	mut cmds: Commands,
	server: Res<AssetServer>,
)
{	//Assetのロードを開始し、ハンドルをVecに保管
	let mut preloading = Vec::new();
	PRELOADING_ASSET_FILES.iter()
		.for_each( | file | preloading.push( server.load_untyped( *file ) ) );

	//VecをResourceに登録
	//これが常に存在するので、bevyのGCがAssetを解放することはないはず
	cmds.insert_resource( LoadedHandles ( preloading ) );
}

//Assetの事前ロードが完了したら、DemoPlayへ遷移
pub fn goto_demo_after_preloading
(	mut state: ResMut<State<GameState>>,
	preloading: Res<LoadedHandles>,
	server: Res<AssetServer>,
)
{	//事前ロードが未完のAssetがあれば、関数を脱出
	use bevy::asset::LoadState::*;
	for handle in preloading.0.iter()
	{	match server.get_load_state( handle )
		{	Loaded => {}
			Failed => { panic!() }	//パニック
			_      => { return   }	//関数脱出
		}
	}

	//DemoPlayへ遷移する
	state.set( GameState::DemoPlay ).unwrap();
}

////////////////////////////////////////////////////////////////////////////////

//ローディングアニメーション関係
const LOADING_MESSAGE_SCALE: f32 = 0.7;
const LOADING_MESSAGE_ARRAY: [ &str; 13 ] = 
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

//Component
pub struct MessageTile ( usize, usize );

//スプライトを生成する
pub fn spawn_preloading_anime_tile
(	mut cmds: Commands,
	mut color_matl: ResMut<Assets<ColorMaterial>>,
)
{	let mut rng = rand::thread_rng();

	for ( map_y, s ) in LOADING_MESSAGE_ARRAY.iter().enumerate()
	{	for ( map_x, c ) in s.chars().enumerate()
		{	if c == ' ' { continue }	//空白文字は無視

			//スプライトの初期位置は乱数で決める
			let rnd_x = rng.gen_range( 0..MAP_WIDTH  );
			let rnd_y = rng.gen_range( 0..MAP_HEIGHT );
			let ( x, y ) = conv_sprite_coordinates( rnd_x, rnd_y );

			cmds.spawn_bundle( sprite_tile( ( x, y ), &mut color_matl ) )
				.insert( MessageTile ( map_x, map_y ) );
		} 
	}
}

//スプライトを動かしてローディングメッセージのアニメーションを見せる
pub fn move_preloading_anime_tile
(	q_tiles: Query<( &mut Transform, &MessageTile )>,
	time: Res<Time>,
)
{	let time_delta = time.delta().as_secs_f32();
	q_tiles.for_each_mut
	(	| ( mut transform, tile ) |
		{	let locate = &mut transform.translation;
			let ( goal_x, goal_y ) = conv_sprite_coordinates( tile.0, tile.1 );

			locate.x += ( goal_x * LOADING_MESSAGE_SCALE - locate.x ) * time_delta;
			locate.y += ( goal_y * LOADING_MESSAGE_SCALE - locate.y ) * time_delta;
    	}
	);
}

//スプライトを削除する
pub fn despawn_preloading_anime_tile
(	q_tiles: Query<Entity, With<MessageTile>>,
	mut cmds: Commands,
)
{	q_tiles.for_each( | tile | { cmds.entity( tile ).despawn() } );
}

//End of code.