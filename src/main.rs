//external modules
use bevy::{ prelude::*, diagnostic::* };
use bevy_prototype_lyon::{ prelude::*, entity::ShapeBundle };
use rand::prelude::*;

//internal modules
mod fetch_assets;
mod ui;
mod demoplay;
mod gameplay;

use fetch_assets::*;
use ui::*;
use demoplay::*;
use gameplay::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

//アプリのTitle
const APP_TITLE: &str = "tigtag";

//表示倍率、ウィンドウの縦横pixel数と背景色
pub const SCREEN_SCALING: usize = 4;
pub const PIXEL_PER_GRID: f32   = ( 8 * SCREEN_SCALING ) as f32;
pub const SCREEN_WIDTH  : f32   = PIXEL_PER_GRID * MAP_WIDTH  as f32;
pub const SCREEN_HEIGHT : f32   = PIXEL_PER_GRID * ( MAP_HEIGHT + 2 ) as f32;
pub const SCREEN_BGCOLOR: Color = Color::rgb_linear( 0.025, 0.025, 0.04 );

////////////////////////////////////////////////////////////////////////////////////////////////////

//ゲームの状態遷移
#[derive(Clone,Copy,Debug,Eq,PartialEq,Hash)]
pub enum GameState
{	Init,
	DemoStart,
	DemoPlay,
	DemoLoop,
	GameStart,
	GamePlay,
	GameClear,
	GameOver,
	Pause,
}

//ECSのSystem Labels
#[derive(Clone,Hash,Debug,Eq,PartialEq,SystemLabel)]
enum Label
{	GenerateMap,
	MoveCharacter,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//メイン関数
fn main()
{	//Main Window
	let main_window = WindowDescriptor
	{	title    : String::from( APP_TITLE ),
		width    : SCREEN_WIDTH,
		height   : SCREEN_HEIGHT,
		resizable: false,
		..Default::default()
	};
	
	let mut app = App::build();
	app
	//----------------------------------------------------------------------------------------------
	.insert_resource( main_window )									// メインウィンドウ
	.insert_resource( ClearColor( SCREEN_BGCOLOR ) )				// 背景色
	.insert_resource( Msaa { samples: 4 } )							// アンチエイリアス
	//----------------------------------------------------------------------------------------------
	.add_plugins( DefaultPlugins )									// デフォルトプラグイン
	.add_plugin( FrameTimeDiagnosticsPlugin::default() )			// fps計測のプラグイン
	.add_plugin( ShapePlugin )										// bevy_prototype_lyon
	//----------------------------------------------------------------------------------------------
	.add_state( GameState::Init )									// 状態遷移の初期値
	.add_event::<GameState>()										// 状態遷移のイベント
	//----------------------------------------------------------------------------------------------
	.add_startup_system( spawn_camera.system() )					// bevyのカメラ設置
	.add_system( handle_esc_key_for_pause.system() )				// [Esc]でpause処理
	//----------------------------------------------------------------------------------------------
	.add_plugin( PluginFetchAssets )
	.add_plugin( PluginUi )
	.add_plugin( PluginDemoPlay )
	.add_plugin( PluginGamePlay )
	//----------------------------------------------------------------------------------------------
	;

	//----------------------------------------------------------------------------------------------
	#[cfg(target_arch = "wasm32")]
	app.add_plugin( bevy_webgl2::WebGL2Plugin );					// WASM用のプラグイン
	//----------------------------------------------------------------------------------------------
	#[cfg(not(target_arch = "wasm32"))]								// WASMで不要なキー操作
	app.add_system( toggle_window_mode.system() );					// [Alt]+[Enter]でフルスクリーン
	//----------------------------------------------------------------------------------------------

	app.run();														// アプリの実行
}

////////////////////////////////////////////////////////////////////////////////////////////////////

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
{	let is_pressed_alt    = inkey.pressed( KeyCode::LAlt ) || inkey.pressed( KeyCode::RAlt );
	let is_pressed_return = inkey.just_pressed( KeyCode::Return );

	if is_pressed_alt && is_pressed_return
	{	use bevy::window::WindowMode::*;
		if let Some( window ) = window.get_primary_mut()
		{	let x = if window.mode() == Windowed { Fullscreen { use_size: true } } else { Windowed };
			window.set_mode( x );
		}
	}
}

//ESCキーが入力さたら一時停止する
fn handle_esc_key_for_pause
(	mut q: Query<&mut Visible, With<MessagePause>>,
	mut inkey: ResMut<Input<KeyCode>>,
	mut state: ResMut<State<GameState>>,
)
{	if let Ok( mut ui ) = q.single_mut()
	{	if inkey.just_pressed( KeyCode::Escape ) 
		{	match state.current()
			{	GameState::Pause => { ui.is_visible = false; state.pop().unwrap() },
				_                => { ui.is_visible = true ; state.push( GameState::Pause ).unwrap() },
			};
			//https://bevy-cheatbook.github.io/programming/states.html#with-input
			inkey.reset( KeyCode::Escape );
		}
	}
}


//End of code.