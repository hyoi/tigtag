use super::*;

//Resource Score
#[derive(Default)]
pub struct Record
{	pub score: usize,
	pub high_score: usize,
}

//Plugin
pub struct PluginInitialize;

impl Plugin for PluginInitialize
{	fn build( &self, app: &mut AppBuilder )
	{	app
		.add_plugin( FrameTimeDiagnosticsPlugin::default() )		//fps計測のプラグイン

		.init_resource::<Record>()									//スコア等のリソース
		.init_resource::<MapInfo>()									//マップ情報のリソース

		//GameState::Initialize
		.add_system_set
		(	SystemSet::on_enter( GameState::Initialize )			//開始
			.with_system( start_preloading_assets.system() )		//Assetのロード開始
			.with_system( spawn_preloading_anime_tile.system() )	//スプライトの生成
			.with_system( spawn_ui_text.system() )					//各種UIの配置(非表示)
		)
		.add_system_set
		(	SystemSet::on_update( GameState::Initialize )			//繰り返し
			.with_system( move_preloading_anime_tile.system() )		//スプライトのアニメーション
			.with_system( goto_demo_after_preloading.system() )		//Assetロード完了 ⇒ DemoPlayへ遷移
		)
		.add_system_set
		(	SystemSet::on_exit( GameState::Initialize )				//終了
			.with_system( despawn_preloading_anime_tile.system() )	//スプライトの削除
			.with_system( show_header_ui.system() )					//UI Headerを表示
		)

		//CoreStage::Update
		.add_system( update_header_ui_left.system() )				//Left   更新
		.add_system( update_header_ui_center.system() )				//Center 更新
		.add_system( update_header_ui_right.system() )				//Right  更新
		;
	}
}

////////////////////////////////////////////////////////////////////////////////

//Component
struct HeaderUiLeft;
struct HeaderUiCenter;
struct HeaderUiRight;
struct HeaderUi;

pub struct MessageDemo;
pub struct MessageStart;
pub struct MessageClear;
pub struct MessageOver;
pub struct MessagePause;

//テキストを配置する(非表示)
fn spawn_ui_text
(	mut cmds: Commands,
	asset_svr: Res<AssetServer>,
)
{	//Header UIの準備
	let mut header_ui_left   = header_ui_left  ( &asset_svr );
	let mut header_ui_center = header_ui_center( &asset_svr );
	let mut header_ui_right  = header_ui_right ( &asset_svr );
	header_ui_left.style.align_self  = AlignSelf::FlexStart;
	header_ui_center.style.align_self = AlignSelf::Center;
	header_ui_right.style.align_self = AlignSelf::FlexEnd;
	header_ui_left.visible.is_visible   = false;
	header_ui_center.visible.is_visible = false;
	header_ui_right.visible.is_visible  = false;

	//メッセージの準備
	let mut message_demo  = ui_text_demo ( &asset_svr );
	let mut message_start = ui_text_start( &asset_svr );
	let mut message_clear = ui_text_clear( &asset_svr );
	let mut message_over  = ui_text_over ( &asset_svr );
	let mut message_pause = ui_text_pause( &asset_svr );
	message_demo.visible.is_visible  = false;
	message_start.visible.is_visible = false;
	message_clear.visible.is_visible = false;
	message_over.visible.is_visible  = false;
	message_pause.visible.is_visible = false;

	//隠しフレームの上に子要素を作成する
	cmds.spawn_bundle( frame_full_screen() ).with_children
	(	| cmds |
		{	cmds.spawn_bundle( frame_map_size() ).with_children
			(	| cmds |
				{	cmds.spawn_bundle( header_ui_left   ).insert( HeaderUi ).insert( HeaderUiLeft   );
					cmds.spawn_bundle( header_ui_center ).insert( HeaderUi ).insert( HeaderUiCenter );
					cmds.spawn_bundle( header_ui_right  ).insert( HeaderUi ).insert( HeaderUiRight  );
				}
			);
			cmds.spawn_bundle( message_demo  ).insert( MessageDemo  );
			cmds.spawn_bundle( message_start ).insert( MessageStart );
			cmds.spawn_bundle( message_clear ).insert( MessageClear );
			cmds.spawn_bundle( message_over  ).insert( MessageOver  );
			cmds.spawn_bundle( message_pause ).insert( MessagePause );
		}
	);
}

//Header UIを表示する
fn show_header_ui( q_ui: Query<&mut Visible, With<HeaderUi>> )
{	q_ui.for_each_mut( | mut visible | { visible.is_visible = true } );
}

////////////////////////////////////////////////////////////////////////////////

//FPSの表示を更新する
fn update_header_ui_left
(	mut q_ui: Query<&mut Text, With<HeaderUiLeft>>,
	diag: Res<Diagnostics>,
)
{	if let Some( fps ) = diag.get( FrameTimeDiagnosticsPlugin::FPS )
	{	if let Some( fps_avg ) = fps.average()
		{	if let Ok( mut ui ) = q_ui.single_mut()
			{	ui.sections[ 1 ].value = format!( "{:.2}", fps_avg );	
			}
		}
	}
}

//スコアとステージのドット数を表示する
fn update_header_ui_center
(	mut q_ui: Query<&mut Text, With<HeaderUiCenter>>,
	( record, map ): ( Res<Record>, Res<MapInfo> ),
)
{	if let Ok( mut ui ) = q_ui.single_mut()
	{	ui.sections[ 1 ].value = format!( "{:06}", record.score );
		ui.sections[ 2 ].value = format!( "/{:03}", map.count_dots );
	}
}

//ハイスコアを表示する
fn update_header_ui_right
(	mut q_ui: Query<&mut Text, With<HeaderUiRight>>,
	record: Res<Record>,
)
{	if let Ok( mut ui ) = q_ui.single_mut()
	{	ui.sections[ 1 ].value = format!( "{:06}", record.high_score );
	}
}

//End of code.