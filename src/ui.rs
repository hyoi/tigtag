use super::*;

//Pluginの手続き
pub struct PluginUi;
impl Plugin for PluginUi
{	fn build( &self, app: &mut AppBuilder )
	{	app
		//------------------------------------------------------------------------------------------
		.add_system( update_ui_upper_left.system() )					// UIの表示を更新
		.add_system( update_ui_upper_center.system() )					// UIの表示を更新
		.add_system( update_ui_upper_right.system() )					// UIの表示を更新
		.add_system( update_ui_lower_left.system() )					// UIの表示を更新
		//------------------------------------------------------------------------------------------
		;
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//ステージ数の表示を更新する
fn update_ui_upper_left
(	mut q: Query<&mut Text, With<UiUpperLeft>>,
	record: Res<Record>,
)
{	if let Ok( mut ui ) = q.single_mut()
	{	ui.sections[ 1 ].value = format!( "{:02}", record.stage );
	}
}

//スコアと残ドット数の表示を更新する
fn update_ui_upper_center
(	mut q: Query<&mut Text, With<UiUpperCenter>>,
	record: Res<Record>,
//	map: Res<MapInfo>,
)
{	if let Ok( mut ui ) = q.single_mut()
	{	ui.sections[ 1 ].value = format!( "{:05}", record.score );
//		ui.sections[ 2 ].value = format!( "/{:03}", map.count_dots );
	}
}

//ハイスコアの表示を更新する
fn update_ui_upper_right
(	mut q: Query<&mut Text, With<UiUpperRight>>,
	record: Res<Record>,
)
{	if let Ok( mut ui ) = q.single_mut()
	{	ui.sections[ 1 ].value = format!( "{:05}", record.high_score );
	}
}

//FPSの表示を更新する
fn update_ui_lower_left
(	mut q: Query<&mut Text, With<UiLowerLeft>>,
	diag: Res<Diagnostics>,
)
{	if let Ok( mut ui ) = q.single_mut()
	{	let na = "--.--".to_string();
		let fps_avr = if let Some( fps ) = diag.get( FrameTimeDiagnosticsPlugin::FPS )
		{	match fps.average()
			{	Some( avg ) => format!( "{:.2}", avg ),
				None        => na,
			}
		} else { na };
		ui.sections[ 1 ].value = fps_avr;
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//タイトルを表示する
pub fn show_message_demo( mut q: Query<&mut Visible, With<MessageDemo>> )
{	let _ = q.single_mut().map( | mut ui | ui.is_visible = true );
}

//タイトルを隠す
pub fn hide_message_demo( mut q: Query<&mut Visible, With<MessageDemo>> )
{	let _ = q.single_mut().map( | mut ui | ui.is_visible = false );
}

//スタートメッセージを表示
pub fn show_message_start( mut q: Query<&mut Visible, With<MessageStart>> )
{	let _ = q.single_mut().map( | mut ui | ui.is_visible = true );
}

//スタートメッセージを隠す
pub fn hide_message_start( mut q: Query<&mut Visible, With<MessageStart>> )
{	let _ = q.single_mut().map( | mut ui | ui.is_visible = false );
}

//クリアメッセージを表示
pub fn show_message_clear( mut q: Query<&mut Visible, With<MessageClear>> )
{	let _ = q.single_mut().map( | mut ui | ui.is_visible = true );
}

//クリアメッセージを隠す
pub fn hide_message_clear( mut q: Query<&mut Visible, With<MessageClear>> )
{	let _ = q.single_mut().map( | mut ui | ui.is_visible = false );
}

//ゲームオーバーを表示
pub fn show_message_over( mut q: Query<&mut Visible, With<MessageOver>> )
{	let _ = q.single_mut().map( | mut ui | ui.is_visible = true );
}

//ゲームオーバーを隠す
pub fn hide_message_over( mut q: Query<&mut Visible, With<MessageOver>> )
{	let _ = q.single_mut().map( | mut ui | ui.is_visible = false );
}

//End of code.