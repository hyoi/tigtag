use super::*;

//リソース：迷路情報
pub struct MapInfo
{	pub array: [ [ MapObj; MAP_HEIGHT ]; MAP_WIDTH ],
	pub count_dots: usize,
}
impl Default for MapInfo
{	fn default() -> Self
	{	Self
		{	array: [ [ MapObj::Space; MAP_HEIGHT ]; MAP_WIDTH ],
			count_dots: 0,
		}
	}
}
#[derive(Copy,Clone,PartialEq)]
pub enum MapObj
{	Space,
	Dot ( Option<Entity> ),
	Wall( Option<Entity> ),
}

////////////////////////////////////////////////////////////////////////////////

//mapを初期化し、壁とドットのスプライトを配置する
pub fn spawn_sprite_new_map
(	mut map: ResMut<MapInfo>,
	mut cmds: Commands,
	( asset_svr, mut color_matl ): ( Res<AssetServer>, ResMut<Assets<ColorMaterial>> )
)
{	//mapを初期化する
	make_new_maze( &mut map );

	//壁とドットのスプライトを配置する
	let mut count_dots = 0;
	for ( map_x, ary ) in map.array.iter_mut().enumerate()
	{	for ( map_y, obj ) in ary.iter_mut().enumerate()
		{	let ( x, y ) = conv_sprite_coordinates( map_x, map_y );
			*obj = match obj
			{	MapObj::Dot(_) =>
				{	count_dots += 1;
					let id = cmds.spawn_bundle( sprite_dot( ( x, y ), &mut color_matl ) ).id(); 
					MapObj::Dot( Some( id ) )
				},
				MapObj::Wall(_) =>
				{	let id = cmds.spawn_bundle( sprite_wall( ( x, y ), &mut color_matl, &asset_svr ) ).id();
					MapObj::Wall( Some( id ) )
				},
				_ => MapObj::Space,
			};
		}
	}
	map.count_dots = count_dots;
}

//新しい迷路を作る
fn make_new_maze( map: &mut ResMut<MapInfo> )
{	//mapを初期化する
	map.array.iter_mut().for_each( |x| (*x).fill( MapObj::Space ) );
	map.count_dots = 0;

	let half_w = MAP_WIDTH  / 2;
	let half_h = MAP_HEIGHT / 2;
	let short_side = if half_w >= half_h { half_h } else { half_w };

	//二次元配列の中の矩形領域を指定値に置き換える無名関数
	let mut box_fill = | obj, mut x1, mut y1, mut x2, mut y2 |
	{	if x1 > x2 { std::mem::swap( &mut x1, &mut x2 ) }
		if y1 > y2 { std::mem::swap( &mut y1, &mut y2 ) }
		for y in y1..=y2
		{	for x in x1..=x2
			{	map.array[ x as usize ][ y as usize ] = obj;
			}
		}
	};

	//基本的な周回の壁と通路
	let dot  = MapObj::Dot ( None );
	let wall = MapObj::Wall( None );
	for xy in 0..=short_side
	{	let obj = if xy % 2 == 0 { wall } else { dot };
		let x2 = MAP_WIDTH  - 1 - xy;
		let y2 = MAP_HEIGHT - 1 - xy;
		box_fill( obj, xy, xy, x2, y2 );
	}

	//十字の通路
	box_fill( dot, 1, half_h, MAP_WIDTH - 2, MAP_HEIGHT - 1 - half_h );
	box_fill( dot, half_w, 1, MAP_WIDTH - 1 - half_w, MAP_HEIGHT - 2 );

	//十字通路の中央に壁を作る
	if short_side % 2 == 0
	{	if half_w >= half_h
		{	if MAP_HEIGHT % 2 != 0
			{	box_fill( wall, short_side, short_side, MAP_WIDTH - 1 - short_side, short_side );
			}
		}
		else if MAP_WIDTH % 2 != 0
		{	box_fill( wall, short_side, short_side, short_side, MAP_HEIGHT - 1 - short_side );
		}
	}

	//ランダムに壁を通路にする
	let mut rng = rand::thread_rng();
	let n = MAP_HEIGHT * MAP_WIDTH / 10; //例: 25*40/10=100
	for _ in 0..n
	{	let x = rng.gen_range( 2..=MAP_WIDTH  - 3 );
		let y = rng.gen_range( 2..=MAP_HEIGHT - 3 );
		map.array[ x ][ y ] = dot;
	}
}

////////////////////////////////////////////////////////////////////////////////

//スプライトを削除する
pub fn despawn_sprite_map
(	map: Res<MapInfo>,
	mut cmds: Commands,
)
{	for ary in map.array.iter()
	{	for obj in ary.iter()
		{	if let MapObj::Wall( opt_entity ) = obj
			{	cmds.entity( opt_entity.unwrap() ).despawn();
			}
			if let MapObj::Dot( opt_entity ) = obj
			{	cmds.entity( opt_entity.unwrap() ).despawn();
			}
		}
	}
}

//End of code.