use super::*;

//Pluginの手続き
pub struct PluginUi;
impl Plugin for PluginUi
{	fn build( &self, app: &mut App )
	{	app
		//------------------------------------------------------------------------------------------
		.add_system( update_ui_upper_left )		// UIの表示を更新
		.add_system( update_ui_upper_center )	// UIの表示を更新
		.add_system( update_ui_upper_right )	// UIの表示を更新
		.add_system( update_ui_lower_left )		// UIの表示を更新
		//------------------------------------------------------------------------------------------
		;
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//ステージ数の表示を更新する
fn update_ui_upper_left
(	mut q: Query<&mut Text, With<UiUpperLeft>>,
	o_record: Option<Res<Record>>,
)
{	if let Ok( mut ui ) = q.get_single_mut()
	{	let na = "--".to_string();
		ui.sections[ 1 ].value = o_record.map_or( na, | x | format!( "{:02}", x.stage ) );
	}
}

//スコアと残ドット数の表示を更新する
fn update_ui_upper_center
(	mut q: Query<&mut Text, With<UiUpperCenter>>,
	o_record: Option<Res<Record>>,
//	map: Res<MapInfo>,
)
{	if let Ok( mut ui ) = q.get_single_mut()
	{	let na = "-----".to_string();
		ui.sections[ 1 ].value = o_record.map_or( na, | x | format!( "{:05}", x.score ) );
//		ui.sections[ 2 ].value = format!( "/{:03}", map.count_dots );
	}
}

//ハイスコアの表示を更新する
fn update_ui_upper_right
(	mut q: Query<&mut Text, With<UiUpperRight>>,
	o_record: Option<Res<Record>>,
)
{	if let Ok( mut ui ) = q.get_single_mut()
	{	let na = "-----".to_string();
		ui.sections[ 1 ].value = o_record.map_or( na, | x | format!( "{:05}", x.high_score ) );
	}
}

//FPSの表示を更新する
fn update_ui_lower_left
(	mut q: Query<&mut Text, With<UiLowerLeft>>,
	diag: Res<Diagnostics>,
)
{	if let Ok( mut ui ) = q.get_single_mut()
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
pub fn show_message_demo( mut q: Query<&mut Visibility, With<MessageDemo>> )
{	let _ = q.get_single_mut().map( | mut ui | ui.is_visible = true );
}

//タイトルを隠す
pub fn hide_message_demo( mut q: Query<&mut Visibility, With<MessageDemo>> )
{	let _ = q.get_single_mut().map( | mut ui | ui.is_visible = false );
}

//スタートメッセージを表示
pub fn show_message_start( mut q: Query<&mut Visibility, With<MessageStart>> )
{	let _ = q.get_single_mut().map( | mut ui | ui.is_visible = true );
}

//スタートメッセージを隠す
pub fn hide_message_start( mut q: Query<&mut Visibility, With<MessageStart>> )
{	let _ = q.get_single_mut().map( | mut ui | ui.is_visible = false );
}

//クリアメッセージを表示
pub fn show_message_clear( mut q: Query<&mut Visibility, With<MessageClear>> )
{	let _ = q.get_single_mut().map( | mut ui | ui.is_visible = true );
}

//クリアメッセージを隠す
pub fn hide_message_clear( mut q: Query<&mut Visibility, With<MessageClear>> )
{	let _ = q.get_single_mut().map( | mut ui | ui.is_visible = false );
}

//ゲームオーバーを表示
pub fn show_message_over( mut q: Query<&mut Visibility, With<MessageOver>> )
{	let _ = q.get_single_mut().map( | mut ui | ui.is_visible = true );
}

//ゲームオーバーを隠す
pub fn hide_message_over( mut q: Query<&mut Visibility, With<MessageOver>> )
{	let _ = q.get_single_mut().map( | mut ui | ui.is_visible = false );
}

//End of code.