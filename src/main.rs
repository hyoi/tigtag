//external modules
use bevy::{ prelude::*, diagnostic::* };
use bevy_kira_audio::{ Audio, AudioPlugin };

//internal modules
mod types;
mod consts;
mod util;

use types::*;
use consts::*;
use util::*;

mod fetch_assets;
mod ui;
mod gameplay;
mod demoplay;

use fetch_assets::*;
use ui::*;
use gameplay::*;
use demoplay::*;

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
	
	let mut app = App::new();
	app
	//----------------------------------------------------------------------------------------------
	.insert_resource( main_window )							// メインウィンドウ
	.insert_resource( ClearColor( SCREEN_BGCOLOR ) )		// 背景色
	.insert_resource( Msaa { samples: 4 } )					// アンチエイリアス
	//----------------------------------------------------------------------------------------------
	.add_plugins( DefaultPlugins )							// デフォルトプラグイン
	.add_plugin( FrameTimeDiagnosticsPlugin::default() )	// fps計測のプラグイン
	.add_plugin( AudioPlugin )
	.add_startup_system( start_background_audio )
	//----------------------------------------------------------------------------------------------
	.add_state( GameState::Init )							// 状態遷移の初期値
	.add_event::<GameState>()								// 状態遷移のイベント
	.init_resource::<Record>()								// スコア等のリソース
	.init_resource::<MapInfo>()								// マップ情報のリソース
	//----------------------------------------------------------------------------------------------
	.add_startup_system( spawn_camera )						// bevyのカメラ設置
	.add_system( handle_esc_key_for_pause )					// [Esc]でpause処理
	//----------------------------------------------------------------------------------------------
	.add_plugin( PluginFetchAssets )
	.add_plugin( PluginUi )
	.add_plugin( PluginGamePlay )
	.add_plugin( PluginDemoPlay )
	//----------------------------------------------------------------------------------------------
	;

	#[cfg(not(target_arch = "wasm32"))]						// WASMで不要なキー操作
	app.add_system( toggle_window_mode );					// [Alt]+[Enter]でフルスクリー

	app.run();												// アプリの実行
}

fn start_background_audio
(	asset_server: Res<AssetServer>,
	audio: Res<Audio>
)
{	audio.play_looped( asset_server.load( SE_BEEP ) );
}
//End of code.