use super::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

//定義と定数

// //Resource Score
// pub struct Record
// {	pub score	  : usize,
// 	pub high_score: usize,
// 	pub stage	  : usize,
// }
// impl Default for Record
// {	fn default() -> Self
// 	{	Self
// 		{	score	  : 0,
// 			high_score: 0,
// 			stage	  : 1,
// 		}
// 	}
// }

//向きを表す列挙型
#[derive(Clone,Copy,PartialEq)]
pub enum Direction { Up, Left, Right, Down, }

//グリッドの上下左右の定数
pub const UP   : ( i32, i32 ) = (  0, -1 );
pub const LEFT : ( i32, i32 ) = ( -1,  0 );
pub const RIGHT: ( i32, i32 ) = (  1,  0 );
pub const DOWN : ( i32, i32 ) = (  0,  1 );

////////////////////////////////////////////////////////////////////////////////////////////////////

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

//二次元配列の添え字から画面座標を算出する
pub fn conv_sprite_coordinates( x: usize, y: usize ) -> ( f32, f32 )
{	let x = ( PIXEL_PER_GRID - SCREEN_WIDTH  ) / 2.0 + PIXEL_PER_GRID * x as f32;
	let y = ( SCREEN_HEIGHT - PIXEL_PER_GRID ) / 2.0 - PIXEL_PER_GRID * y as f32 - PIXEL_PER_GRID;
	( x, y )
}

//スプライトの位置をグリッドに合わせて更新する
pub fn fit_pixel_position_to_grid( transform: &mut Transform, x: usize, y: usize ) -> ( f32, f32 )
{	let ( x, y ) = conv_sprite_coordinates( x, y );
	let position = &mut transform.translation;
	position.x = x;
	position.y = y;

	( position.x, position.y )
}

//スプライトの位置を向きとΔで更新する(グリッドの間の移動)
pub fn update_pixel_position_by_delta( transform: &mut Transform, delta: f32, direction: Direction ) -> ( f32, f32 )
{	let position = &mut transform.translation;
	match direction
	{	Direction::Up    => position.y += delta,
		Direction::Left  => position.x -= delta,
		Direction::Right => position.x += delta,
		Direction::Down  => position.y -= delta,
	}

	( position.x, position.y )
}

//スプライトの表示角度を更新する
pub fn update_sprite_rotation( transform: &mut Transform, angle: f32 )
{	let quat = Quat::from_rotation_z( angle.to_radians() );
	transform.rotate( quat );
}

//自機(三角形)の新旧の向きから、表示角度差分を決める
pub fn decide_angle( old: Direction, new: Direction ) -> f32
{	match old
	{	Direction::Up =>
		{	if matches!( new, Direction::Left  ) { return  90.0 }
			if matches!( new, Direction::Right ) { return -90.0 }
		}
		Direction::Left =>
		{	if matches!( new, Direction::Down  ) { return  90.0 }
			if matches!( new, Direction::Up    ) { return -90.0 }
		}
		Direction::Right =>
		{	if matches!( new, Direction::Up    ) { return  90.0 }
			if matches!( new, Direction::Down  ) { return -90.0 }
		}
		Direction::Down =>
		{	if matches!( new, Direction::Right ) { return  90.0 }
			if matches!( new, Direction::Left  ) { return -90.0 }
		}
	}

	180.0 //呼出側でold != newが保証されているので、±90°以外はすべて180°
}

//マップ中央の矩形内でランダムに自機の初期位置を決める
pub fn init_position_player( map: &MapInfo ) -> ( usize, usize )
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
		if ! matches!( map.array[ x ][ y ], MapObj::Wall ){ break }
	}
	( x, y )
}

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

//End of code.