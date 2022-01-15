use super::*;

//ゲームスタートのカウンターを初期化する
pub fn reset_gamestart_counter( mut q: Query<&mut MessageStart> )
{	if let Ok( mut counter ) = q.get_single_mut()
	{	counter.count = COUNTDOWN_TEXT.len();
		counter.timer.reset();
	}
}

//ゲームスタートのカウントダウンが終わったらGamePlayへ遷移する
pub fn change_state_gameplay_with_cd
(	mut q: Query<(&mut Text, &mut MessageStart)>,
	mut state: ResMut<State<GameState>>,
	time: Res<Time>,
)
{	if let Ok( ( mut text, mut counter ) ) = q.get_single_mut()
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
pub fn reset_gameclear_counter( mut q: Query<&mut MessageClear> )
{	if let Ok( mut counter ) = q.get_single_mut()
	{	counter.count = GAMECLEAR_COUNTDOWN + 1;
		counter.timer.reset();
	}
}

//ゲームクリアのカウントダウンが終わったらGameStartへ遷移する
pub fn change_state_gamestart_with_cd
(	mut q: Query<(&mut Text, &mut MessageClear)>,
	mut state: ResMut<State<GameState>>,
	time: Res<Time>,
)
{	if let Ok( ( mut text, mut counter ) ) = q.get_single_mut()
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
pub fn reset_gameover_counter( mut q: Query<&mut MessageOver> )
{	if let Ok( mut counter ) = q.get_single_mut()
	{	counter.count = GAMEOVER_COUNTDOWN + 1;
		counter.timer.reset();
	}
}

//ゲームオーバーのカウントダウンが終わったらDemoStartへ遷移する
pub fn change_state_demostart_with_cd
(	mut q: Query<(&mut Text, &mut MessageOver)>,
	mut state: ResMut<State<GameState>>,
	time: Res<Time>,
)
{	if let Ok( ( mut text, mut counter ) ) = q.get_single_mut()
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