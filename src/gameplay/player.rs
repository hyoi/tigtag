use super::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

//定義と定数

//移動ウェイト
const PLAYER_WAIT: f32 = 0.09;

//スプライトの動きを滑らかにするための中割係数
const PLAYER_MOVE_COEF: f32 = PIXEL_PER_GRID / PLAYER_WAIT;

//向きを表す列挙型
use super::util::Direction;

//スプライト識別用Component
pub struct Player
{	pub grid_position  : ( usize, usize ),
	pub pixel_position: ( f32, f32 ),
	pub pixel_position_old: ( f32, f32 ),
	direction     : Direction,
	next_direction: Direction,
	wait: Timer,
	stop: bool,
}

//自機のスプライト
const SPRITE_PLAYER_DEPTH: f32   = 20.0;
const SPRITE_PLAYER_PIXEL: f32   = PIXEL_PER_GRID / 2.5;
const SPRITE_PLAYER_COLOR: Color = Color::YELLOW;

////////////////////////////////////////////////////////////////////////////////////////////////////

//自機のスプライトを初期位置に配置する
pub fn spawn_sprite_player
(	q: Query<Entity, With<Player>>,
	map: Res<MapInfo>,
	mut cmds: Commands,
	mut color_matl: ResMut<Assets<ColorMaterial>>,
)
{	//スプライトがあれば削除する
	q.for_each( | id | cmds.entity( id ).despawn() );

	//初期位置を乱数で決める
	let ( grid_x, grid_y ) = init_position_player( &map );
	let ( sprite_x, sprite_y ) = conv_sprite_coordinates( grid_x, grid_y );

	//スプライトを初期位置に配置する
	let player = Player
	{	grid_position: ( grid_x, grid_y ),
		pixel_position: ( sprite_x, sprite_y ),
		pixel_position_old: ( sprite_x, sprite_y ),
		direction: Direction::Up,
		next_direction: Direction::Up,
		wait: Timer::from_seconds( PLAYER_WAIT, false ),
		stop: true,
	};
	let sprite = sprite_player( player.pixel_position, &mut color_matl );
	cmds.spawn_bundle( sprite ).insert( player );
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//自機の移動方向を決める
pub fn move_sprite_player
(	mut q_player: Query<( &mut Player, &mut Transform )>,
	state: Res<State<GameState>>,
	map  : ResMut<MapInfo>,
	inkey: Res<Input<KeyCode>>,
	time : Res<Time>
)
{	let is_demoplay = matches!( state.current(), GameState::DemoPlay );

	let ( mut player, mut transform ) = q_player.single_mut().unwrap();
	let ( mut grid_x, mut grid_y ) = player.grid_position;

	let time_delta = time.delta();
	let is_wait_finished = player.wait.tick( time_delta ).finished();

	//スプライト(三角形)の表示向きを更新する
	if player.direction != player.next_direction
	{	let angle = decide_angle( player.direction, player.next_direction );
		update_sprite_rotation( &mut transform, angle );
		player.direction = player.next_direction;
	}

	if is_wait_finished
	{	//移動先の決定の準備
		let ( mut obj_up, mut obj_left, mut obj_right, mut obj_down ) = get_map_obj_ulrd( ( grid_x, grid_y ), &map );
		let ( mut key_up, mut key_left, mut key_right, mut key_down ) = ( false, false, false, false );

		//キー入力を取得する
		if ! is_demoplay
		{	//プレイヤーのキー入力を取得する
			key_up    = inkey.pressed( KeyCode::Up    );
			key_left  = inkey.pressed( KeyCode::Left  );
			key_right = inkey.pressed( KeyCode::Right );
			key_down  = inkey.pressed( KeyCode::Down  );
		}
		else
		{	//DemoPlayの場合、進行方向の逆側は壁があることにする(STOP以外の場合)
			if ! player.stop
			{	match player.direction
				{	Direction::Up    => obj_down  = MapObj::Wall,
					Direction::Left  => obj_right = MapObj::Wall,
					Direction::Right => obj_left  = MapObj::Wall,
					Direction::Down  => obj_up    = MapObj::Wall,
				}
			}

			//移動できる道を探す
			let mut key = Vec::new();
			if ! matches!( obj_up   , MapObj::Wall ) { key.push( Direction::Up    ) }
			if ! matches!( obj_left , MapObj::Wall ) { key.push( Direction::Left  ) }
			if ! matches!( obj_right, MapObj::Wall ) { key.push( Direction::Right ) }
			if ! matches!( obj_down , MapObj::Wall ) { key.push( Direction::Down  ) }

			//プレイヤーのキー入力を詐称する
			let mut rng = rand::thread_rng();
			match key[ rng.gen_range( 0..key.len() ) ]
			{	Direction::Up    => key_up    = true,
				Direction::Left  => key_left  = true,
				Direction::Right => key_right = true,
				Direction::Down  => key_down  = true,
			} 
		}

		//キー入力に従って自機の向きを変える
		if key_up
		{	player.next_direction = Direction::Up;
			player.stop = matches!( obj_up, MapObj::Wall );
		}
		else if key_left
		{	player.next_direction = Direction::Left;
			player.stop = matches!( obj_left, MapObj::Wall );
		}
		else if key_right
		{	player.next_direction = Direction::Right;
			player.stop = matches!( obj_right, MapObj::Wall );
		}
		else if key_down
		{	player.next_direction = Direction::Down;
			player.stop = matches!( obj_down, MapObj::Wall );
		}
		else
		{	player.stop = true;
		}
	}

	//停止中なら何もしない
	if player.stop { return }

	//スプライトの表示位置を更新する
	let pixel_xy =
		if is_wait_finished
		{	//グリッドにそろえて表示する
			fit_pixel_position_to_grid( &mut transform, grid_x, grid_y )
		}
		else
		{	//移動中の中割の位置に表示する
			let delta = PLAYER_MOVE_COEF * time_delta.as_secs_f32();
			update_pixel_position_by_delta( &mut transform, delta, player.direction )
		};
	player.pixel_position_old = player.pixel_position;
	player.pixel_position = pixel_xy;

	//自機のグリッドの位置を変更する
	if is_wait_finished
	{	match player.next_direction
		{	Direction::Up    => grid_y -= 1,
			Direction::Left  => grid_x -= 1,
			Direction::Right => grid_x += 1,
			Direction::Down  => grid_y += 1,
		}
		player.grid_position = ( grid_x, grid_y );

		//ウェイトをリセットする
		player.wait.reset();
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//自機のスプライトバンドルを生成
//Native
//#[cfg(not(target_arch = "wasm32"))]
fn sprite_player
(	( x, y ): ( f32, f32 ),
	_color_matl: &mut ResMut<Assets<ColorMaterial>>,
) -> ShapeBundle
{	let position = Vec3::new( x, y, SPRITE_PLAYER_DEPTH );
	let triangle = &shapes::RegularPolygon
	{	sides: 3,
		feature: shapes::RegularPolygonFeature::Radius( SPRITE_PLAYER_PIXEL ),
		..shapes::RegularPolygon::default()
	};

	GeometryBuilder::build_as
	(	triangle,
		ShapeColors::new( SPRITE_PLAYER_COLOR ),
        DrawMode::Fill( FillOptions::default() ),
		Transform::from_translation( position )
	)
}
/*//WASM
#[cfg(target_arch = "wasm32")]
fn sprite_player
(	( x, y ): ( f32, f32 ),
	color_matl: &mut ResMut<Assets<ColorMaterial>>,
) -> SpriteBundle
{	let position = Vec3::new( x, y, SPRITE_PLAYER_DEPTH );
	let square = Vec2::new( SPRITE_PLAYER_PIXEL, SPRITE_PLAYER_PIXEL );

	SpriteBundle
	{	material : color_matl.add( SPRITE_PLAYER_COLOR.into() ),
		transform: Transform::from_translation( position ),
		sprite   : Sprite::new( square ),
		..Default::default()
	}
}
*/

//End of code.