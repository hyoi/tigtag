use super::*;

//Plugin
pub struct PluginGamePlay;

#[derive(Clone,Debug,Eq,PartialEq,Hash,SystemLabel)]
struct MarkerLabel1;
#[derive(Clone,Debug,Eq,PartialEq,Hash,SystemLabel)]
struct MarkerLabel2;

impl Plugin for PluginGamePlay
{	fn build( &self, app: &mut AppBuilder )
	{	app
		//GameState::GameStart
		.add_system_set
		(	SystemSet::on_enter( GameState::GameStart )		//開始１
			.label( MarkerLabel1 )
			.with_system( spawn_sprite_new_map.system() )	//新マップを生成して表示
		)
		.add_system_set
		(	SystemSet::on_enter( GameState::GameStart )		//開始２
			.after( MarkerLabel1 )
			.label( MarkerLabel2 )
			.with_system( spawn_sprite_player.system() )	//自機の初期位置を決めて配置
			.with_system( spawn_sprite_chasers.system() )	//追手を初期位置に配置
		)
		.add_system_set
		(	SystemSet::on_update( GameState::GameStart )	//繰り返し
			.after( MarkerLabel2 )
			.with_system( countdown_gamestart.system() )	//カウントダウン終了 ⇒ GamePlayへ遷移
		)

		//GameState::GamePlay
		.add_system_set
		(	SystemSet::on_update( GameState::GamePlay )		//繰り返し１
			.label( MarkerLabel1 )
			.with_system( move_sprite_player.system() )		//自機を移動
			.with_system( move_sprite_chasers.system() )	//追手を移動
		)
		.add_system_set
		(	SystemSet::on_update( GameState::GamePlay )		//繰り返し２
			.after( MarkerLabel1 )
			.with_system( goto_state_event_queue.system() )	//勝利 ⇒ GameClear、敗北 ⇒ GameOverへ遷移
		)

		//GameState::GameClear
		.add_system_set
		(	SystemSet::on_enter( GameState::GameClear )		//開始
			.with_system( show_message_clear.system() )		//クリアメッセージを表示する
		)
		.add_system_set
		(	SystemSet::on_update( GameState::GameClear )	//繰り返し
			.with_system( countdown_gameclear.system() )	//カウントダウン(4まで) ⇒ GamePlayへ遷移
		)
		.add_system_set
		(	SystemSet::on_exit( GameState::GameClear )		//終了
			.with_system( hide_message_clear.system() )		//メッセージを隠す
			.with_system( despawn_sprite_map.system() )		//マップを削除
			.with_system( despawn_sprite_player.system() )	//自機を削除
			.with_system( despawn_sprite_chasers.system() )	//追手を削除
		)

		//GameState::GameOver
		.add_system_set
		(	SystemSet::on_enter( GameState::GameOver )		//開始
			.with_system( show_message_over.system() )		//ゲームオーバーを表示する
		)
		.add_system_set
		(	SystemSet::on_update( GameState::GameOver )		//繰り返し
			.with_system( judge_space_key_input.system() )	//キー入力 ⇒ GameStartへ遷移
			.with_system( countdown_gameover.system() )		//カウントダウン終了 ⇒ DemoPlayへ遷移
		)
		.add_system_set
		(	SystemSet::on_exit( GameState::GameOver )		//終了
			.with_system( hide_message_over.system() )		//メッセージを隠す
			.with_system( despawn_sprite_map.system() )		//マップを削除
			.with_system( despawn_sprite_player.system() )	//自機を削除
			.with_system( despawn_sprite_chasers.system() )	//追手を削除
			.with_system( clear_score.system() )			//スコアを初期化
		)
		;
	}
}

////////////////////////////////////////////////////////////////////////////////

//クリアメッセージを表示
fn show_message_clear( mut q_ui: Query<&mut Visible, With<MessageClear>> )
{	if let Ok( mut ui ) = q_ui.single_mut()
	{	ui.is_visible = true;
	}
}

//クリアメッセージを隠す
fn hide_message_clear( mut q_ui: Query<&mut Visible, With<MessageClear>> )
{	if let Ok( mut ui ) = q_ui.single_mut()
	{	ui.is_visible = false;
	}
}

//ゲームオーバーを表示
fn show_message_over( mut q_ui: Query<&mut Visible, With<MessageOver>> )
{	if let Ok( mut ui ) = q_ui.single_mut()
	{	ui.is_visible = true;
	}
}

//ゲームオーバーを隠す
fn hide_message_over( mut q_ui: Query<&mut Visible, With<MessageOver>> )
{	if let Ok( mut ui ) = q_ui.single_mut()
	{	ui.is_visible = false;
	}
}

//Resource CountdownTimer
#[derive(Default)]
struct CountDown { timer: Timer }

const COUNTDOWN_TEXT: [ &str; 4 ] = [ "Go!", "Ready...\n1", "Ready...\n2", "Ready...\n3" ];

//ゲームスタートのカウントダウン
fn countdown_gamestart
(	mut q_ui: Query<(&mut Text, &mut Visible), With<MessageStart>>,
	mut state: ResMut<State<GameState>>,
	( mut count, mut countdown ): ( Local<i32>, Local<CountDown> ),
	time: Res<Time>,
)
{	if let Ok( ( mut text, mut visible ) ) = q_ui.single_mut()
	{	if *count <= 0
		{	//カウントダウン開始
			countdown.timer = Timer::from_seconds( 1.0, false );	//1秒タイマー
			*count = 4;
			let mess = COUNTDOWN_TEXT[ ( *count - 1 ) as usize ];
			text.sections[ 0 ].value = format!( "{}", mess );
			visible.is_visible = true;
		}
		else if countdown.timer.tick( time.delta() ).finished()	//1秒経過
		{	*count -= 1;
			countdown.timer.reset();	//1秒タイマーリセット

			if *count > 0
			{	//メッセージの書き換え
				{	let mess = COUNTDOWN_TEXT[ ( *count - 1 ) as usize ];
					text.sections[ 0 ].value = format!( "{}", mess );
				}
			}
			else
			{	//カウントダウンが終わったら、GamePlayへ遷移する
				visible.is_visible = false;
				state.overwrite_set( GameState::GamePlay ).unwrap();
			}
		}
	}
}

//ゲームクリアのカウントダウン
fn countdown_gameclear
(	mut q_ui: Query<(&mut Text, &mut Visible), With<MessageClear>>,
	mut state: ResMut<State<GameState>>,
	( mut count, mut countdown ): ( Local<i32>, Local<CountDown> ),
	time: Res<Time>,
)
{	if let Ok( ( mut text, mut visible ) ) = q_ui.single_mut()
	{	if *count <= 4
		{	//カウントダウン開始
			countdown.timer = Timer::from_seconds( 1.0, false );	//1秒タイマー
			*count = 6;
			text.sections[ 1 ].value = format!( "{}", *count - 1 );
			visible.is_visible = true;
		}
		else if countdown.timer.tick( time.delta() ).finished()	//1秒経過
		{	*count -= 1;
			countdown.timer.reset();	//1秒タイマーリセット

			if *count > 4
			{	//メッセージの書き換え
				{	text.sections[ 1 ].value = format!( "{}", *count - 1 );
				}
			}
			else
			{	//カウントダウンが4まで終わったら、GameStartへ遷移する(3以降はGameStartで)
				visible.is_visible = false;
				state.overwrite_set( GameState::GameStart ).unwrap();
			}
		}
	}
}

//ゲームオーバーのカウントダウン
fn countdown_gameover
(	mut q_ui: Query<(&mut Text, &mut Visible), With<MessageOver>>,
	mut state: ResMut<State<GameState>>,
	( mut count, mut countdown ): ( Local<i32>, Local<CountDown> ),
	time: Res<Time>,
)
{	if let Ok( ( mut text, mut visible ) ) = q_ui.single_mut()
	{	if *count <= 0
		{	//カウントダウン開始
			countdown.timer = Timer::from_seconds( 1.0, false );	//1秒タイマー
			*count = 11;
			text.sections[ 2 ].value = format!( "{}", *count - 1 );
			visible.is_visible = true;
		}
		else if countdown.timer.tick( time.delta() ).finished()	//1秒経過
		{	*count -= 1;
			countdown.timer.reset();	//1秒タイマーリセット

			if *count > 0
			{	//メッセージの書き換え
				{	text.sections[ 2 ].value = format!( "{}", *count - 1 );
				}
			}
			else
			{	//カウントダウンが終わったら、DemoPlayへ遷移する
				visible.is_visible = false;
				state.overwrite_set( GameState::DemoPlay ).unwrap();
			}
		}
	}
}

//End of code.