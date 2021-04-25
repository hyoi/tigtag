use super::*;

//移動ウェイト
const CHASER_WAIT : f32 = 0.13;
const CHASER_ACCEL: f32 = 0.4; //スピードアップの割増

//スプライトの動きを滑らかにするための中割係数
const CHASER_MOVE_COEF  : f32 = PIXEL_PER_GRID / CHASER_WAIT;
const CHASER_ROTATE_COEF: f32 = 90. / CHASER_WAIT;

//向きを表す列挙型
#[derive(Clone,Copy,PartialEq)]
enum Direction
{	Up,
	Left,
	Right,
	Down,
}

//Component
pub struct Chaser
{	wait: Timer,
	map_location: ( usize, usize ),
	pub sprite_location: ( f32, f32 ),
	direction: Direction,
	stop: bool,
	color: Color,
	speedup: f32,
}

////////////////////////////////////////////////////////////////////////////////

//追手のスプライトを初期位置に配置する
pub fn spawn_sprite_chasers
(	mut cmds: Commands,
	mut color_matl: ResMut<Assets<ColorMaterial>>,
)
{	//追手は複数なのでループする
	for ( color, ( map_x, map_y ) ) in CHASER_SPRITE_PARAMS.iter()
	{	let ( sprite_x, sprite_y ) = conv_sprite_coordinates( *map_x, *map_y );

		let chaser = Chaser
		{	wait: Timer::from_seconds( CHASER_WAIT, false ),
			map_location: ( *map_x, *map_y ),
			sprite_location: ( sprite_x, sprite_y ),
			direction: Direction::Up,
			stop: true,
			color: *color,
			speedup: 1.,
		};

		//追手のスプライトを初期位置に配置する
		let sprite = sprite_chaser( chaser.sprite_location, *color, &mut color_matl );
		cmds.spawn_bundle( sprite ).insert( chaser );
	}
}

////////////////////////////////////////////////////////////////////////////////

//追手のスプライトを移動する
pub fn move_sprite_chasers
(	( q_player, mut q_chasers ): ( Query<&Player>, Query<( &mut Chaser, &mut Transform )> ),
	( state, mut event ): ( Res<State<GameState>>, EventWriter<GameState> ),
	map: Res<MapInfo>,
	time: Res<Time>,
)
{	let time_delta = time.delta();
	let is_demo = matches!( state.current(), GameState::DemoPlay );

	let player = q_player.single().unwrap();
	let player_xy = player.sprite_location; 

	//ループして追手を処理する
	q_chasers.for_each_mut
	(	| ( mut chaser, mut transform ) |
		{	//重なっている追手の係数を変える
			let delta_speedup = time_delta.mul_f32( chaser.speedup );

			if ! chaser.wait.tick( delta_speedup ).finished()
			{	//停止中なら何も処理しない
				if chaser.stop { return }

				//スプライトを滑らかに移動させるための中割アニメーション
				let delta = CHASER_MOVE_COEF * delta_speedup.as_secs_f32();
				let locate = &mut transform.translation;
				match chaser.direction
				{	Direction::Up    => locate.y += delta,
					Direction::Left  => locate.x -= delta,
					Direction::Right => locate.x += delta,
					Direction::Down  => locate.y -= delta,
				}
				let old_xy = chaser.sprite_location;
				let new_xy = ( locate.x, locate.y );
				chaser.sprite_location = new_xy;

				//追手の回転アニメーション
				let angle = CHASER_ROTATE_COEF * delta_speedup.as_secs_f32();
				let quat  = Quat::from_rotation_z( angle.to_radians() );
				transform.rotate( quat );

				//移動中の交差判定
				if is_collision( old_xy, new_xy, player_xy )
				{	//衝突したので、eventをセットして関数から脱出
					let next = if is_demo { GameState::DemoLoop } else { GameState::GameOver };
					event.send( next );
					return;
				}
			}
			else
			{	//スプライトの表示位置を更新する
				let ( mut map_x, mut map_y ) = chaser.map_location;
				let ( sprite_x, sprite_y ) = conv_sprite_coordinates( map_x, map_y );
				let locate = &mut transform.translation;
				locate.x = sprite_x;
				locate.y = sprite_y;
				let old_xy = chaser.sprite_location;
				let new_xy = ( locate.x, locate.y );
				chaser.sprite_location = new_xy;

				//追手の回転アニメーション
				let angle = CHASER_ROTATE_COEF * delta_speedup.as_secs_f32();
				let quat  = Quat::from_rotation_z( angle.to_radians() );
				transform.rotate( quat );

				//交差判定
				if is_collision( old_xy, new_xy, player_xy )
				{	//衝突したので、eventをセットして関数から脱出
					let next = if is_demo { GameState::DemoLoop } else { GameState::GameOver };
					event.send( next );
					return;
				}

				//追手の上下左右にあるものを取り出す
				let ( mut up, mut left, mut right, mut down ) = get_map_obj_ulrd( ( map_x, map_y ), &map );

				//移動中の場合(STOP以外の場合)、進行方向の逆側は壁があることにする
				if ! chaser.stop
				{	match chaser.direction
					{	Direction::Up    => down  = MapObj::Wall( None ),
						Direction::Left  => right = MapObj::Wall( None ),
						Direction::Right => left  = MapObj::Wall( None ),
						Direction::Down  => up    = MapObj::Wall( None ),
					}
				}

				//移動できる道を数える。(移動中は１～3、停止中は2～4)
				//結果的にcountが1なら、追手の進行方向は道なりに決まる
				let mut count = 0;
				if ! matches!( up   , MapObj::Wall(_) ) { count += 1; chaser.direction = Direction::Up    }
				if ! matches!( left , MapObj::Wall(_) ) { count += 1; chaser.direction = Direction::Left  }
				if ! matches!( right, MapObj::Wall(_) ) { count += 1; chaser.direction = Direction::Right }
				if ! matches!( down , MapObj::Wall(_) ) { count += 1; chaser.direction = Direction::Down  }

				//countが2以上なら分かれ道なので、改めて進行方向を決める
				if count > 1
				{	chaser.direction = decide_direction( &chaser, &player, up, left, right, down );
				}

				//データ上の位置を更新する。
				//まだスプライトの表示位置は変えない。
				let ( dx, dy ) = match chaser.direction
				{	Direction::Up    => UP,
					Direction::Left  => LEFT,
					Direction::Right => RIGHT,
					Direction::Down  => DOWN,
				};
				map_x = ( map_x as i32 + dx ) as usize;
				map_y = ( map_y as i32 + dy ) as usize;
				chaser.map_location = ( map_x, map_y );
				chaser.stop = false;

				//ウェイトをリセットする
				chaser.wait.reset();
			}
		}
	);

	//追手は重なると速度アップする
	let mut work = [ ( Color::BLACK, ( 0, 0 ) ); CHASER_COUNT ];
	for ( i, ( mut chaser, _ ) ) in q_chasers.iter_mut().enumerate()
	{	work[ i ] = ( chaser.color, chaser.map_location );
		chaser.speedup = 1.;
	}

	for work in work.iter()
	{	let ( color, ( map_x, map_y ) ) = work;
		for ( mut chaser, _ ) in q_chasers.iter_mut()
		{	if ( *map_x, *map_y ) != chaser.map_location || *color == chaser.color { continue }
			chaser.speedup += CHASER_ACCEL;
		}
	}
}

//分かれ道で追手の進行方向を決める
fn decide_direction
(	chaser: &Mut<Chaser>,
	player: &Player,
	up: MapObj, left: MapObj, right: MapObj, down: MapObj,
)
-> Direction
{	//追手は色ごとに、分かれ道で優先する方向が違う
	let ( cx, cy ) = chaser.map_location;
	let ( px, py ) = player.map_location;
	if chaser.color == Color::RED
	{	if px < cx && ! matches!( left , MapObj::Wall(_) ) { return Direction::Left  }
		if px > cx && ! matches!( right, MapObj::Wall(_) ) { return Direction::Right }
		if py < cy && ! matches!( up   , MapObj::Wall(_) ) { return Direction::Up    }
		if py > cy && ! matches!( down , MapObj::Wall(_) ) { return Direction::Down  }
	}
	else if chaser.color == Color::BLUE
	{	if py > cy && ! matches!( down , MapObj::Wall(_) ) { return Direction::Down  }
		if px < cx && ! matches!( left , MapObj::Wall(_) ) { return Direction::Left  }
		if px > cx && ! matches!( right, MapObj::Wall(_) ) { return Direction::Right }
		if py < cy && ! matches!( up   , MapObj::Wall(_) ) { return Direction::Up    }
	}
	else if chaser.color == Color::GREEN
	{	if py < cy && ! matches!( up   , MapObj::Wall(_) ) { return Direction::Up    }
		if py > cy && ! matches!( down , MapObj::Wall(_) ) { return Direction::Down  }
		if px < cx && ! matches!( left , MapObj::Wall(_) ) { return Direction::Left  }
		if px > cx && ! matches!( right, MapObj::Wall(_) ) { return Direction::Right }
	}
	else if chaser.color == Color::PINK
	{	if px > cx && ! matches!( right, MapObj::Wall(_) ) { return Direction::Right }
		if py < cy && ! matches!( up   , MapObj::Wall(_) ) { return Direction::Up    }
		if py > cy && ! matches!( down , MapObj::Wall(_) ) { return Direction::Down  }
		if px < cx && ! matches!( left , MapObj::Wall(_) ) { return Direction::Left  }
	}

	//ここに到達したら、ランダムに方向を決める
	let mut rng = rand::thread_rng();
	loop
	{	let ( obj, result ) = match rng.gen_range( 0..=3 )
		{	0 => ( left , Direction::Left  ),
			1 => ( right, Direction::Right ),
			2 => ( up   , Direction::Up    ),
			_ => ( down , Direction::Down  ),
		};
		if ! matches!( obj, MapObj::Wall(_) ) { return result }
	}
}

////////////////////////////////////////////////////////////////////////////////

//追手のスプライトを削除する
pub fn despawn_sprite_chasers
(	q_chasers: Query<Entity, With<Chaser>>,
	mut cmds: Commands,
)
{	q_chasers.for_each( | chaser | { cmds.entity( chaser ).despawn() } );
}

//End of code.