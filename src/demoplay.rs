use super::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

//Pluginの手続き
pub struct PluginDemoPlay;
impl Plugin for PluginDemoPlay
{	fn build( &self, app: &mut AppBuilder )
	{	app
		//------------------------------------------------------------------------------------------
		// .add_system_set												// ＜GameState::DemoStart＞
		// (	SystemSet::on_enter( GameState::DemoStart )				// ＜on_enter()＞
		// 		.with_system( show_message_demo.system() )				// タイトルを表示する
		// )
		.add_system_set													// ＜GameState::DemoStart＞
		(	SystemSet::on_enter( GameState::DemoStart )					// ＜on_enter()＞
				.label( Label::GenerateMap )							// ＜label＞
				.with_system( spawn_sprite_new_map.system() )			// 新マップを生成して表示
		)
		.add_system_set													// ＜GameState::DemoStart＞
		(	SystemSet::on_enter( GameState::DemoStart )					// ＜on_enter()＞
				.after( Label::GenerateMap )							// ＜after＞
				.with_system( spawn_sprite_player.system() )			// 自機を配置(マップ生成後)
				.with_system( spawn_sprite_chasers.system() )			// 追手を配置
		)
		.add_system_set													// ＜GameState::DemoStart＞
		(	SystemSet::on_update( GameState::DemoStart )				// ＜on_update()＞
				.with_system( change_state_demoplay.system() )			// 無条件⇒DemoPlayへ遷移
		)
		//------------------------------------------------------------------------------------------
		.add_system_set													// ＜GameState::DemoPlay＞
		(	SystemSet::on_update( GameState::DemoPlay )					// ＜on_update()＞
				.with_system( change_state_gamestart_by_key.system() )	// SPACEキー入力⇒GameStartへ遷移
		)
		.add_system_set													// ＜GameState::DemoPlay＞
		(	SystemSet::on_update( GameState::DemoPlay )					// ＜on_update()＞
				.before( Label::MoveSpriteCharacters )					// ＜before＞
				.with_system( detect_score_and_collision.system() )		// クリア⇒DemoLoop、衝突⇒DemoLoop
		)
		.add_system_set													// ＜GameState::DemoPlay＞
		(	SystemSet::on_update( GameState::DemoPlay )					// ＜on_update()＞
				.label( Label::MoveSpriteCharacters )					// ＜label＞
				.with_system( move_sprite_player.system() )				// 自機のスプライトを移動する
				.with_system( move_sprite_chaser.system() )				// 追手のスプライトを移動する
		)
		.add_system_set													// ＜GameState::Demo＞
		(	SystemSet::on_exit( GameState::DemoPlay )					// ＜on_exit()＞
				.with_system( hide_message_demo.system() )				// タイトルを隠す
				.with_system( clear_record.system() )					// スコアとステージを初期化
		)
		//------------------------------------------------------------------------------------------
		.add_system_set													// ＜GameState::DemoLoop＞
		(	SystemSet::on_enter( GameState::DemoLoop )					// ＜on_enter()＞
				.with_system( change_state_demostart.system() )			// 無条件⇒DemoStartへ遷移
		)
		//------------------------------------------------------------------------------------------
		;
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//無条件にDemoPlayへ遷移する
fn change_state_demoplay( mut state: ResMut<State<GameState>> )
{	let _ = state.overwrite_set( GameState::DemoPlay );
}

//無条件にDemoStartへ遷移する
fn change_state_demostart( mut state: ResMut<State<GameState>> )
{	use std::{ thread, time };
	thread::sleep( time::Duration::from_secs( 3 ) );

	let _ = state.overwrite_set( GameState::DemoStart );
}

//End of code.