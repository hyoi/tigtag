use super::*;

//移動ウェイト
const PLAYER_WAIT: f32 = 0.09;

//スプライトの動きを滑らかにするための中割係数
const PLAYER_MOVE_COEF: f32 = PIXEL_PER_GRID / PLAYER_WAIT;

//向きを表す列挙型
#[derive(Clone,Copy,PartialEq)]
enum Direction
{	Up,
	Left,
	Right,
	Down,
}

//Component
pub struct Player
{	wait: Timer,
	pub map_location: ( usize, usize ),
	pub sprite_location: ( f32, f32 ),
	direction: Direction,
	new_direction: Direction,
	stop: bool,
}

////////////////////////////////////////////////////////////////////////////////

//自機のスプライトを初期位置に配置する
pub fn spawn_sprite_player
(	map: Res<MapInfo>,
	mut cmds: Commands,
	mut color_matl: ResMut<Assets<ColorMaterial>>,
)
{	let ( map_x, map_y ) = init_location_player( &map );
	let ( sprite_x, sprite_y ) = conv_sprite_coordinates( map_x, map_y );

	let player = Player
	{	wait: Timer::from_seconds( PLAYER_WAIT, false ),
		map_location: ( map_x, map_y ),
		sprite_location: ( sprite_x, sprite_y ),
		direction: Direction::Up,
		new_direction: Direction::Up,
		stop: true,
	};

	//自機のスプライトを初期位置に配置する
	let sprite = sprite_player( player.sprite_location, &mut color_matl );
	cmds.spawn_bundle( sprite ).insert( player );
}

//マップ中央の矩形内でランダムに自機の初期位置を決める
fn init_location_player( map: &MapInfo ) -> ( usize, usize )
{   let half_w = MAP_WIDTH  / 2;
	let half_h = MAP_HEIGHT / 2;
	let short_side = if half_w >= half_h { half_h } else { half_w };

	let x1 = short_side - 1;
	let y1 = short_side - 1;
	let x2 = MAP_WIDTH  - short_side;
	let y2 = MAP_HEIGHT - short_side;

	let ( mut x, mut y );
	let mut rng = rand::thread_rng();
	loop
	{	x = rng.gen_range( x1..=x2 );
		y = rng.gen_range( y1..=y2 );
		if ! matches!( map.array[ x ][ y ], MapObj::Wall(_) ){ return ( x, y ) }
	}
}

////////////////////////////////////////////////////////////////////////////////

//自機のスプライトを移動する
pub fn move_sprite_player
(	( mut q_player, q_chasers ): ( Query<( &mut Player, &mut Transform )>, Query<&Chaser> ),
	( state, mut event ): ( Res<State<GameState>>, EventWriter<GameState> ),
	( mut map, mut record ): ( ResMut<MapInfo>, ResMut<Record> ),
	mut cmds: Commands,
	( time, inkey ): ( Res<Time>, Res<Input<KeyCode>> ),
)
{	let time_delta = time.delta();
	let is_demo = matches!( state.current(), GameState::DemoPlay );

	let ( mut player, mut transform ) = q_player.single_mut().unwrap();

	if ! player.wait.tick( time_delta ).finished()
	{	//停止中なら何も処理しない
		if player.stop { return }

		//スプライトを滑らかに移動させるための中割アニメーション
		let delta = PLAYER_MOVE_COEF * time_delta.as_secs_f32();
		let locate = &mut transform.translation;
		match player.direction
		{	Direction::Up    => locate.y += delta,
			Direction::Left  => locate.x -= delta,
			Direction::Right => locate.x += delta,
			Direction::Down  => locate.y -= delta,
		}
		let old_xy = player.sprite_location;
		let new_xy = ( locate.x, locate.y );
		player.sprite_location = new_xy;

		//スプライト(三角形)の表示向きを更新する
		if player.direction != player.new_direction
		{	let angle = decide_angle( player.direction, player.new_direction );
			let quat = Quat::from_rotation_z( angle.to_radians() );
			transform.rotate( quat );
			player.direction = player.new_direction;
		}

		//移動中の交差判定
		q_chasers.for_each
		(	| chaser |
			{	let chaser_xy = chaser.sprite_location;
				if is_collision( old_xy, new_xy, chaser_xy )
				{	//衝突したので、eventをセットして関数から脱出
					let next = if is_demo { GameState::DemoLoop } else { GameState::GameOver };
					event.send( next );
					return;
				}
			}
		);
	}
	else
	{	//スプライトの表示位置を更新する
		let ( mut map_x, mut map_y ) = player.map_location;
		let ( sprite_x, sprite_y ) = conv_sprite_coordinates( map_x, map_y );
		let locate = &mut transform.translation;
		locate.x = sprite_x;
		locate.y = sprite_y;
		let old_xy = player.sprite_location;
		let new_xy = ( locate.x, locate.y );
		player.sprite_location = new_xy;

		//スプライト(三角形)の表示向きを更新する
		if player.direction != player.new_direction
		{	let angle = decide_angle( player.direction, player.new_direction );
			let quat = Quat::from_rotation_z( angle.to_radians() );
			transform.rotate( quat );
			player.direction = player.new_direction;
		}

		//ドット獲得判定
		if let MapObj::Dot( opt_dot ) = map.array[ map_x ][ map_y ]
		{	cmds.entity( opt_dot.unwrap() ).despawn();
			map.array[ map_x ][ map_y ] = MapObj::Space;
			map.count_dots -= 1;
			record.score += 1;
			if record.score > record.high_score { record.high_score = record.score }

			if map.count_dots <= 0
			{	//クリアしたので、eventをセットして関数から脱出
				let next = if is_demo { GameState::DemoLoop } else { GameState::GameClear };
				event.send( next );
				return;
			}
		}

		//交差判定
		q_chasers.for_each
		(	| chaser |
			{	let chaser_xy = chaser.sprite_location;
				if is_collision( old_xy, new_xy, chaser_xy )
				{	//衝突したので、eventをセットして関数から脱出
					let next = if is_demo { GameState::DemoLoop } else { GameState::GameOver };
					event.send( next );
					return;
				}
			}
		);

		//上下左右にあるものを取り出す
		let ( mut up, mut left, mut right, mut down ) = get_map_obj_ulrd( ( map_x, map_y ), &map );

		//自機の移動方向を、Demoの場合は自動で、Playの場合はキー入力で決める
		let ( mut key_left, mut key_right, mut key_up, mut key_down ) =
			( false, false, false, false );

		if ! is_demo
		{	//Playの場合、キー入力を取得する
			key_left  = inkey.pressed( KeyCode::Left  );
			key_right = inkey.pressed( KeyCode::Right );
			key_up    = inkey.pressed( KeyCode::Up    );
			key_down  = inkey.pressed( KeyCode::Down  );
		}
		else
		{	//Demoの場合、キー入力を詐称する

			//移動中の場合(STOP以外の場合)、進行方向の逆側は壁があることにする
			if ! player.stop
			{	match player.direction
				{	Direction::Up    => down  = MapObj::Wall( None ),
					Direction::Left  => right = MapObj::Wall( None ),
					Direction::Right => left  = MapObj::Wall( None ),
					Direction::Down  => up    = MapObj::Wall( None ),
				}
			}

			//移動できる道を数える。(移動中は１～3、停止中は2～4)
			//結果的にcountが1なら、追手の進行方向は道なりに決まる
			let mut count = 0;
			let mut new_direction = player.direction;
			if ! matches!( up   , MapObj::Wall(_) ) { count += 1; new_direction = Direction::Up    }
			if ! matches!( left , MapObj::Wall(_) ) { count += 1; new_direction = Direction::Left  }
			if ! matches!( right, MapObj::Wall(_) ) { count += 1; new_direction = Direction::Right }
			if ! matches!( down , MapObj::Wall(_) ) { count += 1; new_direction = Direction::Down  }

			//countが2以上なら分かれ道なので、改めて進行方向を決める
			if count > 1
			{	let mut rng = rand::thread_rng();
				loop
				{	let ( obj, direct ) = match rng.gen_range( 0..=3 )
					{	0 => ( left , Direction::Left  ),
						1 => ( right, Direction::Right ),
						2 => ( up   , Direction::Up    ),
						_ => ( down , Direction::Down  ),
					};
					if ! matches!( obj, MapObj::Wall(_) )
					{	new_direction = direct;
						break;
					}
				}
			}

			//キー入力の詐称
			match new_direction
			{	Direction::Up    => key_up    = true,
				Direction::Left  => key_left  = true,
				Direction::Right => key_right = true,
				Direction::Down  => key_down  = true,
			} 
		}

		//カーソルキーの入力により自機の向きを変える
		if key_left
		{	player.new_direction = Direction::Left;
			player.stop = matches!( left, MapObj::Wall(_) );
			if ! player.stop { map_x -= 1 }
		}
		else
		if key_right
		{	player.new_direction = Direction::Right;
			player.stop = matches!( right, MapObj::Wall(_) );
			if ! player.stop { map_x += 1 }
		}
		else
		if key_up
		{	player.new_direction = Direction::Up;
			player.stop = matches!( up, MapObj::Wall(_) );
			if ! player.stop { map_y -= 1 }
		}
		else
		if key_down
		{	player.new_direction = Direction::Down;
			player.stop = matches!( down, MapObj::Wall(_) );
			if ! player.stop { map_y += 1 }
		}
		else
		{	player.stop = true
		}
		player.map_location = ( map_x, map_y );

		//ウェイトをリセットする
		player.wait.reset();
	}
}

//自機(三角形)の新旧の向きから、表示角度差分を決める
fn decide_angle( old: Direction, new: Direction ) -> f32
{	match old
	{	Direction::Up =>
		{	if matches!( new, Direction::Left  ) { return  90. }
			if matches!( new, Direction::Right ) { return -90. }
		}
		Direction::Left =>
		{	if matches!( new, Direction::Down  ) { return  90. }
			if matches!( new, Direction::Up    ) { return -90. }
		}
		Direction::Right =>
		{	if matches!( new, Direction::Up    ) { return  90. }
			if matches!( new, Direction::Down  ) { return -90. }
		}
		Direction::Down =>
		{	if matches!( new, Direction::Right ) { return  90. }
			if matches!( new, Direction::Left  ) { return -90. }
		}
	}

	//呼出側でold != newが保証されているので、±90°以外はすべて180°
	180.
}

////////////////////////////////////////////////////////////////////////////////

//自機のスプライトを削除する
pub fn despawn_sprite_player
(	mut q_player: Query<Entity, With<Player>>,
	mut cmds: Commands,
)
{	let player = q_player.single_mut().unwrap();
	cmds.entity( player ).despawn();
}

//End of code.