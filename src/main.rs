//external modules
use bevy::{ prelude::*, diagnostic::* };
use bevy_prototype_lyon::{ prelude::*, entity::ShapeBundle };
use rand::prelude::*;

//internal modules
mod plugin_initialize;
mod plugin_demoplay;
mod plugin_gameplay;
mod asset_loading;
mod text_bundles;
mod sprite_bundles;
mod map;
mod player;
mod chasers;
mod util_game;

use plugin_initialize::*;
use plugin_demoplay::*;
use plugin_gameplay::*;
use asset_loading::*;
use text_bundles::*;
use sprite_bundles::*;
use map::*;
use player::*;
use chasers::*;
use util_game::*;

////////////////////////////////////////////////////////////////////////////////

//迷路の縦横のマス数
pub const MAP_WIDTH : usize = 39;
pub const MAP_HEIGHT: usize = 21;

//表示倍率、ウィンドウの縦横pixel数と背景色
pub const SCREEN_SCALING: usize = 4;
pub const PIXEL_PER_GRID: f32   = ( 8 * SCREEN_SCALING ) as f32;
pub const SCREEN_WIDTH  : f32   = PIXEL_PER_GRID * MAP_WIDTH  as f32;
pub const SCREEN_HEIGHT : f32   = PIXEL_PER_GRID * MAP_HEIGHT as f32;
pub const SCREEN_BGCOLOR: Color = Color::rgb_linear( 0.025, 0.025, 0.04 );

////////////////////////////////////////////////////////////////////////////////

//状態遷移
#[derive(Clone,Copy,Debug,Eq,PartialEq,Hash)]
pub enum GameState
{	Initialize,
	DemoPlay,
	DemoLoop,
	GameStart,
	GamePlay,
	GameClear,
	GameOver,
	Pause,
}

//メイン関数
fn main()
{	let main_window = WindowDescriptor
	{	title    : String::from( "Don't get caught." ),
		width    : SCREEN_WIDTH,
		height   : SCREEN_HEIGHT,
		resizable: false,
		..Default::default()
	};
	
	let mut app = App::build();

	app
	.insert_resource( main_window )						//メインウィンドウ
	.insert_resource( ClearColor( SCREEN_BGCOLOR ) )	//背景色
	.insert_resource( Msaa { samples: 4 } )				//アンチエイリアス

	.add_plugins( DefaultPlugins )						//デフォルトプラグイン
	.add_plugin( ShapePlugin )							//bevy_prototype_lyonを使う

	.add_state( GameState::Initialize )					//状態遷移のリソース
	.add_event::<GameState>()							//状態遷移のイベント

	.add_startup_system( spawn_camera.system() )		//bevyのカメラ設置
	.add_system( judge_esc_key_input.system() )			//一時停止処理

	.add_plugin( PluginInitialize )
	.add_plugin( PluginDemoPlay )
	.add_plugin( PluginGamePlay )
	;

	//WASM用のプラグイン
	#[cfg(target_arch = "wasm32")]
	app.add_plugin( bevy_webgl2::WebGL2Plugin );

	//WASMに不要なキー操作
	#[cfg(not(target_arch = "wasm32"))]
	app.add_system( toggle_window_mode.system() ); //[Alt]+[Enter]でフルスクリーン

	//アプリの実行
	app.run();
}

////////////////////////////////////////////////////////////////////////////////

//bevyのカメラの設置
fn spawn_camera( mut cmds: Commands )
{	cmds.spawn_bundle( UiCameraBundle::default() );
	cmds.spawn_bundle( OrthographicCameraBundle::new_2d() );
}

//[Alt]+[Enter]でウィンドウとフルスクリーンを切り替える
#[cfg(not(target_arch = "wasm32"))]
fn toggle_window_mode
(	inkey: Res<Input<KeyCode>>,
	mut window: ResMut<Windows>,
)
{	use KeyCode::*;
	let is_alt = inkey.pressed( LAlt ) || inkey.pressed( RAlt );
	let is_alt_return = is_alt && inkey.just_pressed( Return );

	if is_alt_return
	{	use bevy::window::WindowMode::*;
		if let Some( window ) = window.get_primary_mut()
		{	let mode = if window.mode() == Windowed
				{ Fullscreen { use_size: true } } else { Windowed };
			window.set_mode( mode );
		}
	}
}

//ESCキーが入力さたら一時停止する
fn judge_esc_key_input
(	mut q_ui: Query<&mut Visible, With<MessagePause>>,
	mut inkey: ResMut<Input<KeyCode>>,
	mut state: ResMut<State<GameState>>,
)
{	if let Ok( mut ui ) = q_ui.single_mut()
	{	if inkey.just_pressed( KeyCode::Escape ) 
		{	match state.current()
			{	GameState::Pause => { ui.is_visible = false; state.pop().unwrap() },
				_                => { ui.is_visible = true ; state.push( GameState::Pause ).unwrap() },
			};
			inkey.reset( KeyCode::Escape ); //https://bevy-cheatbook.github.io/programming/states.html#with-input
		}
	}
}

//End of code.