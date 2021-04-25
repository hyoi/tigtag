use super::*;

//Plugin
pub struct PluginDemoPlay;

#[derive(Clone,Debug,Eq,PartialEq,Hash,SystemLabel)]
struct MarkerLabel;

impl Plugin for PluginDemoPlay
{	fn build( &self, app: &mut AppBuilder )
	{	app
		//GameState::DemoPlay
		.add_system_set
		(	SystemSet::on_enter( GameState::DemoPlay )		//開始１
			.label( MarkerLabel )
			.with_system( show_message_demo.system() )		//タイトルを表示する
			.with_system( spawn_sprite_new_map.system() )	//新マップを生成して表示
		)
		.add_system_set
		(	SystemSet::on_enter( GameState::DemoPlay )		//開始２
			.after( MarkerLabel )
			.with_system( spawn_sprite_player.system() )	//自機の初期位置を決めて配置
			.with_system( spawn_sprite_chasers.system() )	//追手を初期位置に配置
		)
		.add_system_set
		(	SystemSet::on_update( GameState::DemoPlay )		//繰り返し
			.with_system( judge_space_key_input.system() )	//キー入力 ⇒ GameStartへ遷移
		)
		.add_system_set
		(	SystemSet::on_update( GameState::DemoPlay )		//繰り返し１
			.label( MarkerLabel )
			.with_system( move_sprite_player.system() )		//自機を移動
			.with_system( move_sprite_chasers.system() )	//追手を移動
		)
		.add_system_set
		(	SystemSet::on_update( GameState::DemoPlay )		//繰り返し２
			.after( MarkerLabel )
			.with_system( goto_state_event_queue.system() )	//勝利 or 敗北 ⇒ DemoLoopへ遷移
		)
		.add_system_set
		(	SystemSet::on_exit( GameState::DemoPlay )		//終了
			.with_system( hide_message_demo.system() )		//メッセージを隠す
			.with_system( despawn_sprite_map.system() )		//マップを削除
			.with_system( despawn_sprite_player.system() )	//自機を削除
			.with_system( despawn_sprite_chasers.system() )	//追手を削除
			.with_system( clear_score.system() )			//スコアを初期化
		)

		//GameState::DemoLoop
		.add_system_set
		(	SystemSet::on_enter( GameState::DemoLoop )		//開始
			.with_system( goto_state_demo.system() )		//無条件にDemoPlayへ遷移
		)
		;
	}
}

////////////////////////////////////////////////////////////////////////////////

//タイトルを表示する
fn show_message_demo( mut q_ui: Query<&mut Visible, With<MessageDemo>> )
{	if let Ok( mut ui ) = q_ui.single_mut()
	{	ui.is_visible = true;	
	}
}

//タイトルを隠す
fn hide_message_demo( mut q_ui: Query<&mut Visible, With<MessageDemo>> )
{	if let Ok( mut ui ) = q_ui.single_mut()
	{	ui.is_visible = false;	
	}
}

//無条件にDemoPlayへ遷移する
fn goto_state_demo( mut state: ResMut<State<GameState>> )
{	state.overwrite_set( GameState::DemoPlay ).unwrap();
}

//End of code.