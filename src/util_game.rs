use super::*;

//二次元配列の添え字から画面座標を算出する
pub fn conv_sprite_coordinates( x: usize, y: usize ) -> ( f32, f32 )
{	let x = ( PIXEL_PER_GRID - SCREEN_WIDTH  ) / 2. + PIXEL_PER_GRID * x as f32;
	let y = ( SCREEN_HEIGHT - PIXEL_PER_GRID ) / 2. - PIXEL_PER_GRID * y as f32;
	( x, y )
}

//移動体が障害物と交差するか
pub fn is_collision
(	( sx, sy ): ( f32, f32 ), //移動開始位置
	( ex, ey ): ( f32, f32 ), //移動終了位置
	( tx, ty ): ( f32, f32 ), //障害物の位置
) -> bool
{	//衝突する場合はXかYが先に一致している(このゲームには斜め移動がないので)
	if sy as i32 == ty as i32
	{	if sx <= tx && tx <= ex { return true }
		if sx >= tx && tx >= ex { return true }
	}
	else
	if sx as i32 == tx as i32
	{	if sy <= ty && ty <= ey { return true }
		if sy >= ty && ty >= ey { return true }
	}

	false
}

//マップ座標の上下左右の定数
pub const UP   : ( i32, i32 ) = (  0, -1 );
pub const LEFT : ( i32, i32 ) = ( -1,  0 );
pub const RIGHT: ( i32, i32 ) = (  1,  0 );
pub const DOWN : ( i32, i32 ) = (  0,  1 );

//マップの上下左右にあるものを取り出す
pub fn get_map_obj_ulrd( ( x, y ): ( usize, usize ), map: &MapInfo ) -> ( MapObj, MapObj, MapObj, MapObj )
{	let get_map_obj = | ( dx, dy ) |
	{	let x = x as i32 + dx;
		let y = y as i32 + dy;
		map.array[ x as usize][ y as usize]
	};

	let up    = get_map_obj( UP    );
	let left  = get_map_obj( LEFT  );
	let right = get_map_obj( RIGHT );
	let down  = get_map_obj( DOWN  );

	( up, left, right, down )
}

//eventで渡されたstateへ遷移する(キューの先頭だけ処理)
pub fn goto_state_event_queue
(	( mut state, mut events ): ( ResMut<State<GameState>>, EventReader<GameState> )
)
{	if let Some( next_state ) = events.iter().next()
	{	state.overwrite_set( *next_state ).unwrap();
	}
}

//ゲームオーバー時にスコアを初期化する
pub fn clear_score( mut record: ResMut<Record> )
{	record.score = 0;
}

//SPACEキーが入力され次第ステートを変更する
pub fn judge_space_key_input
(	mut inkey: ResMut<Input<KeyCode>>,
	mut state: ResMut<State<GameState>>,
)
{	if inkey.just_pressed( KeyCode::Space ) 
	{	let next_state = match state.current()
		{	GameState::DemoPlay => GameState::GameStart,
			GameState::GameOver => GameState::GameStart,
			_                   => unreachable!(),
		};
		state.overwrite_set( next_state ).unwrap();
		inkey.reset( KeyCode::Space ); //https://bevy-cheatbook.github.io/programming/states.html#with-input
	}
}

//End of code.