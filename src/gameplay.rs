use super::*;

mod map;
mod player;
mod chasers;
mod util;

pub use map::*;
pub use player::*;
pub use chasers::*;
pub use util::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

//Pluginの手続き
pub struct PluginGamePlay;
impl Plugin for PluginGamePlay
{	fn build( &self, app: &mut AppBuilder )
	{	app
		//------------------------------------------------------------------------------------------
		.init_resource::<Record>()										// スコア等のリソース
		.init_resource::<MapInfo>()										// マップ情報のリソース
		//------------------------------------------------------------------------------------------
		.add_system_set													// ＜GameState::GameStart＞
		(	SystemSet::on_enter( GameState::GameStart )					// ＜on_enter()＞
				.with_system( show_message_start.system() )				// スタートメッセージを表示する
				.with_system( reset_gamestart_counter.system() )		// カウントダウン用のカウンタークリア
		)
		.add_system_set													// ＜GameState::GameStart＞
		(	SystemSet::on_enter( GameState::GameStart )					// ＜on_enter()＞
				.label( Label::GenerateMap )							// ＜label＞
				.with_system( spawn_sprite_new_map.system() )			// 新マップを生成して表示
		)
		.add_system_set													// ＜GameState::GameStart＞
		(	SystemSet::on_enter( GameState::GameStart )					// ＜on_enter()＞
				.after( Label::GenerateMap )							// ＜after＞
				.with_system( spawn_sprite_player.system() )			// 自機を配置(マップ生成後)
				.with_system( spawn_sprite_chasers.system() )			// 追手を配置
		)
		.add_system_set													// ＜GameState::GameStart＞
		(	SystemSet::on_update( GameState::GameStart )				// ＜on_update()＞
				.with_system( change_state_gameplay_with_cd.system() )	// カウントダウン終了⇒GamePlayへ遷移
		)
		.add_system_set													// ＜GameState::GameStart＞
		(	SystemSet::on_exit( GameState::GameStart )					// ＜on_exit()＞
				.with_system( hide_message_start.system() )				// スタートメッセージを隠す
		)
		//------------------------------------------------------------------------------------------
		.add_system_set													// ＜GameState::GamePlay＞
		(	SystemSet::on_update( GameState::GamePlay )					// ＜on_update()＞
				.label( Label::MoveCharacter )							// ＜label＞
				.with_system( move_sprite_player.system() )				// 自機を移動
				.with_system( move_sprite_chasers.system() )			// 追手を移動
		)
		.add_system_set													// ＜GameState::GamePlay＞
		(	SystemSet::on_update( GameState::GamePlay )					// ＜on_update()＞
				.after( Label::MoveCharacter )							// ＜after＞
				.with_system( change_state_clear_or_over.system() )		// 勝利⇒GameClear、敗北⇒GameOverへ遷移
		)
		//------------------------------------------------------------------------------------------
		.add_system_set													// ＜GameState::GameClear＞
		(	SystemSet::on_enter( GameState::GameClear )					// ＜on_enter()＞
				.with_system( show_message_clear.system() )				// クリアメッセージを表示する
				.with_system( reset_gameclear_counter.system() )		// カウントダウン用のカウンタークリア
		)
		.add_system_set													// ＜GameState::GameClear＞
		(	SystemSet::on_update( GameState::GameClear )				// ＜on_update()＞
				.with_system( change_state_gamestart_with_cd.system() )	// カウントダウン終了⇒GameStartへ遷移
		)
		.add_system_set													// ＜GameState::Clear＞
		(	SystemSet::on_exit( GameState::GameClear )					// ＜on_exit()＞
				.with_system( hide_message_clear.system() )				// クリアメッセージを隠す
				.with_system( increment_record.system() )				// ステージを＋１する
		)
		//------------------------------------------------------------------------------------------
		.add_system_set													// ＜GameState::GameOver＞
		(	SystemSet::on_enter( GameState::GameOver )					// ＜on_enter()＞
				.with_system( show_message_over.system() )				// ゲームオーバーを表示する
				.with_system( reset_gameover_counter.system() )			// カウントダウン用のカウンタークリア
		)
		.add_system_set													// ＜GameState::GameOver＞
		(	SystemSet::on_update( GameState::GameOver )					// ＜on_update()＞
				.with_system( change_state_gamestart_by_key.system() )	// SPACEキー入力⇒GameStartへ遷移
				.with_system( change_state_demostart_with_cd.system() )	// カウントダウン終了⇒DemoStartへ遷移
		)
		.add_system_set													// ＜GameState::GameOver＞
		(	SystemSet::on_exit( GameState::GameOver )					// ＜on_exit()＞
				.with_system( hide_message_over.system() )				// ゲームオーバーを隠す
				.with_system( clear_record.system() )					// スコアとステージを初期化
		)
		//------------------------------------------------------------------------------------------
		;
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//定義と定数

//Resource Score
pub struct Record
{	pub score	  : usize,
	pub high_score: usize,
	pub stage	  : usize,
}
impl Default for Record
{	fn default() -> Self
	{	Self
		{	score	  : 0,
			high_score: 0,
			stage	  : 1,
		}
	}
}

//ゲームクリア時にステージを＋１する
pub fn increment_record( mut record: ResMut<Record> )
{	record.stage += 1;
}

//ゲームオーバー時にスコアとステージを初期化する
pub fn clear_record( mut record: ResMut<Record> )
{	record.score = 0;
	record.stage = 1;
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//eventで渡されたstateへ遷移する(キューの先頭だけ処理、早い者勝ち)
fn change_state_clear_or_over
(	mut state : ResMut<State<GameState>>,
	mut events: EventReader<GameState>,
)
{	if let Some( next_state ) = events.iter().next()
	{	//if *next_state == GameState::GameOver { return } //Clearしたい時、デバッグ用
		let _ = state.overwrite_set( *next_state );
	}
}

//SPACEキーが入力されたらGameStartへ遷移する
pub fn change_state_gamestart_by_key
(	mut inkey: ResMut<Input<KeyCode>>,
	mut state: ResMut<State<GameState>>,
)
{	if inkey.just_pressed( KeyCode::Space ) 
	{	let _ = state.overwrite_set( GameState::GameStart );

		//https://bevy-cheatbook.github.io/programming/states.html#with-input
		inkey.reset( KeyCode::Space );
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//ゲームスタートのカウンターを初期化する
fn reset_gamestart_counter
(	mut q: Query<&mut MessageStart>,
)
{	if let Ok( mut counter ) = q.single_mut()
	{	counter.count = COUNTDOWN_TEXT.len();
		counter.timer.reset();
	}
}

//ゲームスタートのカウントダウンが終わったらGamePlayへ遷移する
fn change_state_gameplay_with_cd
(	mut q: Query<(&mut Text, &mut MessageStart)>,
	mut state: ResMut<State<GameState>>,
	time: Res<Time>,
)
{	if let Ok( ( mut text, mut counter ) ) = q.single_mut()
	{	//1秒経過したら
		if counter.timer.tick( time.delta() ).finished()
		{	counter.count -= 1;
			counter.timer.reset();	//1秒タイマーリセット
		}

		//カウントダウンが終わったら、GamePlayへ遷移する
		if counter.count == 0
		{	let _ = state.overwrite_set( GameState::GamePlay );
			return;
		}

		//テキストを更新する
		let mess = COUNTDOWN_TEXT[ ( counter.count - 1 ) as usize ];
		text.sections[ 0 ].value = mess.to_string();
	}
}

//--------------------------------------------------------------------------------------------------

//ゲームクリアのカウンターを初期化する
fn reset_gameclear_counter
(	mut q: Query<&mut MessageClear>,
)
{	if let Ok( mut counter ) = q.single_mut()
	{	counter.count = GAMECLEAR_COUNTDOWN + 1;
		counter.timer.reset();
	}
}

//ゲームクリアのカウントダウンが終わったらGameStartへ遷移する
fn change_state_gamestart_with_cd
(	mut q: Query<(&mut Text, &mut MessageClear)>,
	mut state: ResMut<State<GameState>>,
	time: Res<Time>,
)
{	if let Ok( ( mut text, mut counter ) ) = q.single_mut()
	{	//1秒経過したら
		if counter.timer.tick( time.delta() ).finished()
		{	counter.count -= 1;
			counter.timer.reset();	//1秒タイマーリセット
		}

		//カウントダウンが4まで終わったら、GameStartへ遷移する
		if counter.count == 4
		{	let _ = state.overwrite_set( GameState::GameStart );
			return;
		}

		//テキストを更新する
		text.sections[ 2 ].value = format!( "{}", counter.count - 1 );
	}
}

//--------------------------------------------------------------------------------------------------

//ゲームオーバーのカウンターを初期化する
fn reset_gameover_counter
(	mut q: Query<&mut MessageOver>,
)
{	if let Ok( mut counter ) = q.single_mut()
	{	counter.count = GAMEOVER_COUNTDOWN + 1;
		counter.timer.reset();
	}
}

//ゲームオーバーのカウントダウンが終わったらDemoStartへ遷移する
fn change_state_demostart_with_cd
(	mut q: Query<(&mut Text, &mut MessageOver)>,
	mut state: ResMut<State<GameState>>,
	time: Res<Time>,
)
{	if let Ok( ( mut text, mut counter ) ) = q.single_mut()
	{	//1秒経過したら
		if counter.timer.tick( time.delta() ).finished()
		{	counter.count -= 1;
			counter.timer.reset();	//1秒タイマーリセット
		}

		//カウントダウンが終わったら、DemoStartへ遷移する
		if counter.count == 0
		{	let _ = state.overwrite_set( GameState::DemoStart );
			return;
		}

		//テキストを更新する
		text.sections[ 2 ].value = format!( "{}", counter.count - 1 );
	}
}

//End of code.