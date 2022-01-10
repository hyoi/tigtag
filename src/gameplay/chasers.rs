use super::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

//定義と定数

//移動ウェイト
const CHASER_WAIT : f32 = 0.13;
const CHASER_ACCEL: f32 = 0.4; //スピードアップの割増

//スプライトの動きを滑らかにするための中割係数
const CHASER_MOVE_COEF  : f32 = PIXEL_PER_GRID / CHASER_WAIT;
const CHASER_ROTATE_COEF: f32 = 90. / CHASER_WAIT;

//向きを表す列挙型
use super::util::Direction;

//スプライト識別用Component
#[ derive( Component ) ]
pub struct Chaser
{	pub grid_position: ( usize, usize ),
	pub pixel_position: ( f32, f32 ),
	pub pixel_position_old: ( f32, f32 ),
	direction: Direction,
	wait: Timer,
	pub stop: bool,
	pub collision: bool,
	color: Color,
	speedup: f32,
}

//追手の初期位置(四隅のグリッド座標)
const MAX_X: usize = MAP_WIDTH  - 2;
const MAX_Y: usize = MAP_HEIGHT - 2;
const CHASER_START_POSITION: [ ( usize, usize ); 4 ] =
[	( 1    , 1     ),
	( 1    , MAX_Y ),
	( MAX_X, 1     ),
	( MAX_X, MAX_Y ),
];

//追手のスプライト
const SPRITE_CHASER_DEPTH: f32 = 30.0;
const SPRITE_CHASER_PIXEL: f32 = PIXEL_PER_GRID / 2.0;
pub const CHASER_COUNT: usize = 4;
pub const CHASER_SPRITE_PARAMS: [ Color; CHASER_COUNT ] =
[	Color::RED  ,
	Color::BLUE ,
	Color::GREEN,
	Color::PINK ,
];

////////////////////////////////////////////////////////////////////////////////////////////////////

//追手のスプライトを初期位置に配置する
pub fn spawn_sprite_chasers
(	q: Query<Entity, With<Chaser>>,
	record: Res<Record>,
	mut cmds: Commands,
)
{	//スプライトが居れば削除する
	q.for_each( | id | cmds.entity( id ).despawn() );

	//追手は複数なのでループする
	( 0.. ).zip( CHASER_SPRITE_PARAMS ).for_each( | ( i, color ) |
	{	let ( grid_x, grid_y ) = CHASER_START_POSITION[ ( record.stage - 1 + i ) % 4 ];
		let ( sprite_x, sprite_y ) = conv_sprite_coordinates( grid_x, grid_y );

		//スプライトを初期位置に配置する
		let chaser = Chaser
		{	grid_position: ( grid_x, grid_y ),
			pixel_position: ( sprite_x, sprite_y ),
			pixel_position_old: ( sprite_x, sprite_y ),
			direction: Direction::Up,
			wait: Timer::from_seconds( CHASER_WAIT, false ),
			stop: true,
			collision: false,
			color,
			speedup: 1.,
		};
		let sprite = sprite_chaser( chaser.pixel_position, color );
		cmds.spawn_bundle( sprite ).insert( chaser );
	} );
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//自機のスプライトを移動する
pub fn move_sprite_chaser
(	mut q_chaser: Query<( &mut Chaser, &mut Transform )>,
	q_player: Query<&Player>,
	map     : ResMut<MapInfo>,
	time    : Res<Time>
)
{	let time_delta = time.delta();
	let player = q_player.get_single().unwrap();

	//ループして追手を処理する
	q_chaser.for_each_mut
	(	| ( mut chaser, mut transform ) |
		{	//移動先の決定の準備
			let ( mut grid_x, mut grid_y ) = chaser.grid_position;
			let ( mut up, mut left, mut right, mut down ) = get_map_obj_ulrd( ( grid_x, grid_y ), &map );

			let time_delta = time_delta.mul_f32( chaser.speedup );
			let is_wait_finished = chaser.wait.tick( time_delta ).finished();

			//追手の回転アニメーション
			let angle = CHASER_ROTATE_COEF * time_delta.as_secs_f32();
			update_sprite_rotation( &mut transform, angle );

			//追手の移動方向を決める
			if is_wait_finished
			{	//進行方向の逆側は壁があることにする(STOP以外の場合)
				if ! chaser.stop
				{	match chaser.direction
					{	Direction::Up    => down  = MapObj::Wall,
						Direction::Left  => right = MapObj::Wall,
						Direction::Right => left  = MapObj::Wall,
						Direction::Down  => up    = MapObj::Wall,
					}
				}
	
				//追手の進行方向を決める
				chaser.direction = decide_direction( &chaser, player, up, left, right, down );
				chaser.stop = false;
			}

			//停止中なら何もしない
			if chaser.stop { return }

			//スプライトの表示位置を更新する
			let pixel_xy =
				if is_wait_finished
				{	//グリッドにそろえて表示する
					fit_pixel_position_to_grid( &mut transform, grid_x, grid_y )
				}
				else
				{	//移動中の中割の位置に表示する
					let delta  = CHASER_MOVE_COEF * time_delta.as_secs_f32();
					update_pixel_position_by_delta( &mut transform, delta, chaser.direction )
				};
			chaser.pixel_position_old = chaser.pixel_position;
			chaser.pixel_position = pixel_xy;

			//追手のグリッドの位置を変更する
			if is_wait_finished
			{	let ( dx, dy ) = match chaser.direction
				{	Direction::Up    => UP,
					Direction::Left  => LEFT,
					Direction::Right => RIGHT,
					Direction::Down  => DOWN,
				};
				grid_x = ( grid_x as i32 + dx ) as usize;
				grid_y = ( grid_y as i32 + dy ) as usize;
				chaser.grid_position = ( grid_x, grid_y );
				chaser.stop = false;

				//ウェイトをリセットする
				chaser.wait.reset();
			}
		}
	);

	//追手は重なると速度アップする
	let mut work = [ ( Color::BLACK, ( 0, 0 ) ); CHASER_COUNT ];
	for ( i, ( mut chaser, _ ) ) in q_chaser.iter_mut().enumerate()
	{	work[ i ] = ( chaser.color, chaser.grid_position );
		chaser.speedup = 1.0;
	}
	for work in work
	{	let ( color, ( grid_x, grid_y ) ) = work;
		for ( mut chaser, _ ) in q_chaser.iter_mut()
		{	if ( grid_x, grid_y ) != chaser.grid_position || color == chaser.color { continue }
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
	let ( cx, cy ) = chaser.grid_position;
	let ( px, py ) = player.grid_position;
	if chaser.color == CHASER_SPRITE_PARAMS[ 0 ]
	{	if px < cx && ! matches!( left , MapObj::Wall ) { return Direction::Left  }
		if px > cx && ! matches!( right, MapObj::Wall ) { return Direction::Right }
		if py < cy && ! matches!( up   , MapObj::Wall ) { return Direction::Up    }
		if py > cy && ! matches!( down , MapObj::Wall ) { return Direction::Down  }
	}
	else if chaser.color == CHASER_SPRITE_PARAMS[ 1 ]
	{	if py > cy && ! matches!( down , MapObj::Wall ) { return Direction::Down  }
		if px < cx && ! matches!( left , MapObj::Wall ) { return Direction::Left  }
		if px > cx && ! matches!( right, MapObj::Wall ) { return Direction::Right }
		if py < cy && ! matches!( up   , MapObj::Wall ) { return Direction::Up    }
	}
	else if chaser.color == CHASER_SPRITE_PARAMS[ 2 ]
	{	if py < cy && ! matches!( up   , MapObj::Wall ) { return Direction::Up    }
		if py > cy && ! matches!( down , MapObj::Wall ) { return Direction::Down  }
		if px < cx && ! matches!( left , MapObj::Wall ) { return Direction::Left  }
		if px > cx && ! matches!( right, MapObj::Wall ) { return Direction::Right }
	}
	else if chaser.color == CHASER_SPRITE_PARAMS[ 3 ]
	{	if px > cx && ! matches!( right, MapObj::Wall ) { return Direction::Right }
		if py < cy && ! matches!( up   , MapObj::Wall ) { return Direction::Up    }
		if py > cy && ! matches!( down , MapObj::Wall ) { return Direction::Down  }
		if px < cx && ! matches!( left , MapObj::Wall ) { return Direction::Left  }
	}

	//ここに到達したら、ランダムに方向を決める
	let mut rng = rand::thread_rng();
	loop
	{	let ( obj, result ) = match rng.gen_range( 0..=3 )
		{	0 => ( up   , Direction::Up    ),
			1 => ( left , Direction::Left  ),
			2 => ( right, Direction::Right ),
			_ => ( down , Direction::Down  ),
		};
		if ! matches!( obj, MapObj::Wall ) { return result }
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//追手のスプライトバンドルを生成
fn sprite_chaser
(	( x, y ): ( f32, f32 ),
	color: Color,
) -> SpriteBundle
{	let position = Vec3::new( x, y, SPRITE_CHASER_DEPTH );
	let square   = Vec2::new( SPRITE_CHASER_PIXEL, SPRITE_CHASER_PIXEL );

	let transform = Transform::from_translation( position );
	let sprite = Sprite { color, custom_size: Some( square ), ..Default::default() };

	let mut sprite = SpriteBundle { transform, sprite, ..Default::default() };

	//45°傾けて菱形に見せる
	let quat = Quat::from_rotation_z( 45_f32.to_radians() );
	sprite.transform.rotate( quat ); //.rotate()は()を返すのでメソッドチェーンできない

	sprite
}

//End of code.