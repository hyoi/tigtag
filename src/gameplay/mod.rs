use super::*;

//external modules
use bevy_prototype_lyon::{ prelude::*, entity::ShapeBundle };
use bevy_kira_audio::{ Audio, AudioPlugin };
use rand::prelude::*;

//Sub module
mod map;
mod player;
mod chasers;
mod countdown_ui;
mod util;

//Re export
pub use map::*;
pub use player::*;
pub use chasers::*;
pub use countdown_ui::*;
pub use util::*;

//Pluginの手続き
pub struct PluginGamePlay;
impl Plugin for PluginGamePlay
{	fn build( &self, app: &mut App )
	{	app
		//------------------------------------------------------------------------------------------
		.add_plugin( ShapePlugin )								// bevy_prototype_lyon
		.add_plugin( AudioPlugin )								// bevy_kira_audio
		//==========================================================================================
		.add_system_set											// ＜GameState::GameStart＞
		(	SystemSet::on_enter( GameState::GameStart )			// ＜on_enter()＞
				.with_system( show_ui::<MessageStart> )			// スタートメッセージを表示する
				.with_system( reset_gamestart_counter )			// カウントダウン用のカウンタークリア
		)
		.add_system_set											// ＜GameState::GameStart＞
		(	SystemSet::on_enter( GameState::GameStart )			// ＜on_enter()＞
				.label( Label::GenerateMap )					// ＜label＞
				.with_system( spawn_sprite_new_map )			// 新マップを生成して表示
		)
		.add_system_set											// ＜GameState::GameStart＞
		(	SystemSet::on_enter( GameState::GameStart )			// ＜on_enter()＞
				.after( Label::GenerateMap )					// ＜after＞
				.with_system( spawn_sprite_player )				// 自機を配置(マップ生成後)
				.with_system( spawn_sprite_chasers )			// 追手を配置
		)
		//------------------------------------------------------------------------------------------
		.add_system_set											// ＜GameState::GameStart＞
		(	SystemSet::on_update( GameState::GameStart )		// ＜on_update()＞
				.with_system( change_state_gameplay_with_cd )	// カウントダウン終了⇒GamePlayへ遷移
		)
		//------------------------------------------------------------------------------------------
		.add_system_set											// ＜GameState::GameStart＞
		(	SystemSet::on_exit( GameState::GameStart )			// ＜on_exit()＞
				.with_system( hide_ui::<MessageStart> )			// スタートメッセージを隠す
		)
		//==========================================================================================
		.add_system_set											// ＜GameState::GamePlay＞
		(	SystemSet::on_update( GameState::GamePlay )			// ＜on_update()＞
				.before( Label::MoveSpriteCharacters )			// ＜before＞
				.with_system( detect_score_and_collision )		// クリア⇒GameClear、衝突⇒GameOver
		)
		.add_system_set											// ＜GameState::GamePlay＞
		(	SystemSet::on_update( GameState::GamePlay )			// ＜on_update()＞
				.label( Label::MoveSpriteCharacters )			// ＜label＞
				.with_system( move_sprite_player )				// 自機のスプライトを移動する
				.with_system( move_sprite_chaser )				// 追手のスプライトを移動する
		)
		//==========================================================================================
		.add_system_set											// ＜GameState::GameClear＞
		(	SystemSet::on_enter( GameState::GameClear )			// ＜on_enter()＞
				.with_system( show_ui::<MessageClear> )			// クリアメッセージを表示する
				.with_system( reset_gameclear_counter )			// カウントダウン用のカウンタークリア
		)
		//------------------------------------------------------------------------------------------
		.add_system_set											// ＜GameState::GameClear＞
		(	SystemSet::on_update( GameState::GameClear )		// ＜on_update()＞
				.with_system( change_state_gamestart_with_cd )	// カウントダウン終了⇒GameStartへ遷移
		)
		//------------------------------------------------------------------------------------------
		.add_system_set											// ＜GameState::Clear＞
		(	SystemSet::on_exit( GameState::GameClear )			// ＜on_exit()＞
				.with_system( hide_ui::<MessageClear> )			// クリアメッセージを隠す
				.with_system( increment_record )				// ステージを＋１する
		)
		//==========================================================================================
		.add_system_set											// ＜GameState::GameOver＞
		(	SystemSet::on_enter( GameState::GameOver )			// ＜on_enter()＞
				.with_system( show_ui::<MessageOver> )			// ゲームオーバーを表示する
				.with_system( reset_gameover_counter )			// カウントダウン用のカウンタークリア
		)
		//------------------------------------------------------------------------------------------
		.add_system_set											// ＜GameState::GameOver＞
		(	SystemSet::on_update( GameState::GameOver )			// ＜on_update()＞
				.with_system( change_state_gamestart_by_key )	// SPACEキー入力⇒GameStartへ遷移
				.with_system( change_state_demostart_with_cd )	// カウントダウン終了⇒DemoStartへ遷移
		)
		//------------------------------------------------------------------------------------------
		.add_system_set											// ＜GameState::GameOver＞
		(	SystemSet::on_exit( GameState::GameOver )			// ＜on_exit()＞
				.with_system( hide_ui::<MessageOver> )			// ゲームオーバーを隠す
				.with_system( clear_record )					// スコアとステージを初期化
		)
		//------------------------------------------------------------------------------------------
		;
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//得点と衝突を判定する。クリアならGameClearへ、衝突ならGameOverへ遷移する
pub fn detect_score_and_collision
(	mut q_set: ParamSet
	<(	Query<( &mut Player, &mut Transform )>,
		Query<( &mut Chaser, &mut Transform )>,
	)>,
	mut state : ResMut<State<GameState>>,
	mut record: ResMut<Record>,
	mut map   : ResMut<MapInfo>,
	asset_svr : Res<AssetServer>,
	audio     : Res<Audio>,
	mut cmds  : Commands,
)
{	let is_demoplay = matches!( state.current(), GameState::DemoPlay );

	//自機のgrid座標のオブジェクトがドットなら
	let mut q0 = q_set.p0();
	let ( mut player, mut transform ) = q0.iter_mut().next().unwrap();
	let ( p_grid_x, p_grid_y ) = player.grid_position;
	if let MapObj::Dot( opt_dot ) = map.array[ p_grid_x ][ p_grid_y ]
	{	//得点処理
		audio.play( asset_svr.load( SOUND_BEEP ) );
		record.score += 1;
		map.array[ p_grid_x ][ p_grid_y ] = MapObj::Space;
		map.count_dots -= 1;
		cmds.entity( opt_dot.unwrap() ).despawn();

		//ハイスコアの更新
		if ! is_demoplay && record.score > record.high_score
		{	record.high_score = record.score;
		}

		//クリアならstateをセットして関数から脱出
		if map.count_dots == 0
		{	player.stop = true;
			fit_pixel_position_to_grid( &mut transform, p_grid_x, p_grid_y );

			let next = if is_demoplay { GameState::DemoLoop } else { GameState::GameClear };
			let _ = state.overwrite_set( next );
			return;
		}
	}

	//追手と自機のpixel座標が衝突しているか？
	let mut is_over = false;
	let ( p_new_xf32, p_new_yf32 ) = player.pixel_position;
	let ( p_old_xf32, p_old_yf32 ) = player.pixel_position_old;
	let p_stop = player.stop;

	let ( mut p_new_x, mut p_new_y ) = ( ( p_new_xf32 * 100.0 ) as i32, ( p_new_yf32 * 100.0 ) as i32 );
	let ( mut p_old_x, mut p_old_y ) = ( ( p_old_xf32 * 100.0 ) as i32, ( p_old_yf32 * 100.0 ) as i32 );

	for ( mut chaser, _ ) in q_set.p1().iter_mut()
	{	let ( c_new_xf32, c_new_yf32 ) = chaser.pixel_position;
		let ( c_old_xf32, c_old_yf32 ) = chaser.pixel_position_old;

		let ( mut c_new_x, mut c_new_y ) = ( ( c_new_xf32 * 100.0 ) as i32, ( c_new_yf32 * 100.0 ) as i32 );
		let ( mut c_old_x, mut c_old_y ) = ( ( c_old_xf32 * 100.0 ) as i32, ( c_old_yf32 * 100.0 ) as i32 );
	
		let is_collision =
			if p_new_y == c_new_y			//Y軸が一致するなら
			{	if p_new_x > p_old_x { std::mem::swap( &mut p_new_x, &mut p_old_x ) }
				if c_new_x > c_old_x { std::mem::swap( &mut c_new_x, &mut c_old_x ) }

				( p_new_x..=p_old_x ).contains( &c_new_x ) ||
				( p_new_x..=p_old_x ).contains( &c_old_x ) ||
				( c_new_x..=c_old_x ).contains( &p_new_x ) ||	
				( c_new_x..=c_old_x ).contains( &p_old_x )
			}
			else if p_new_x == c_new_x 	//X軸が一致するなら
			{	if p_new_y > p_old_y { std::mem::swap( &mut p_new_y, &mut p_old_y ) }
				if c_new_y > c_old_y { std::mem::swap( &mut c_new_y, &mut c_old_y ) }

				( p_new_y..=p_old_y ).contains( &c_new_y ) ||
				( p_new_y..=p_old_y ).contains( &c_old_y ) ||
				( c_new_y..=c_old_y ).contains( &p_new_y ) ||
				( c_new_y..=c_old_y ).contains( &p_old_y )
			}
			else
			{ false };

		//衝突ならフラグをセットして脱出
		if is_collision
		{	is_over = true;
			chaser.collision = true;

			//playerが移動中にchaserに衝突したなら
			if ! p_stop
			{	let mut q0 = q_set.p0();
				let ( mut player, mut transform ) = q0.iter_mut().next().unwrap();
				player.stop = true;
				let position = &mut transform.translation;
				position.x = c_new_xf32;
				position.y = c_new_yf32;
			}

			break;	//QuerySetのmutable borrowに影響するので必要らしい
		}
	}

	//衝突ならstateをセットして関数から脱出
	if is_over
	{	//衝突時にchaserの表示位置を調整する
		for ( mut chaser, mut transform ) in q_set.p1().iter_mut()
		{	chaser.stop = true;
			if p_stop && chaser.collision
			{	//playerが停止中ならchaserがplayerへ衝突した
				let position = &mut transform.translation;
				position.x = p_new_xf32;
				position.y = p_new_yf32;
			}
		}
		
		let next = if is_demoplay { GameState::DemoLoop } else { GameState::GameOver };
		let _ = state.overwrite_set( next );
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

//End of code.