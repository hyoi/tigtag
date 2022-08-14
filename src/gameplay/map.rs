use super::*;

//スプライト識別用Component
#[derive(Component)]
pub struct SpriteMap;

//マップのスプライト
const SPRITE_MAP_DEPTH : f32   = 10.0;
const SPRITE_WALL_PIXEL: f32   = PIXEL_PER_GRID;
const SPRITE_DOT_RAIDUS: f32   = PIXEL_PER_GRID / 14.0;
const SPRITE_DOT_COLOR : Color = Color::WHITE;

////////////////////////////////////////////////////////////////////////////////////////////////////

//mapを初期化し、壁とドットのスプライトを配置する
pub fn spawn_sprite_new_map
(	q: Query<Entity, With<SpriteMap>>,
	mut map: ResMut<MapInfo>,
	mut cmds: Commands,
	asset_svr: Res<AssetServer>,
)
{	//スプライトがあれば削除する
	q.for_each( | id | cmds.entity( id ).despawn() );

	//mapを初期化する
	make_new_maze( &mut map );

	//mapのスプライトを配置する
	let mut count = 0;
	for ( x, ary ) in map.array.iter_mut().enumerate()
	{	for ( y, obj ) in ary.iter_mut().enumerate()
		{	let xy = conv_sprite_coordinates( x, y );
			*obj = match obj
			{	MapObj::Dot(_) =>
				{	count += 1;
					let id = cmds.spawn_bundle( sprite_dot( xy ) ).insert( SpriteMap ).id(); 
					MapObj::Dot( Some( id ) )
				},
				MapObj::Wall =>
				{	cmds.spawn_bundle( sprite_wall( xy, &asset_svr ) ).insert( SpriteMap );
					MapObj::Wall
				},
				_ => MapObj::Space,
			};
		}
	}
	map.count_dots = count;
}

//新しい迷路を作る
fn make_new_maze( map: &mut ResMut<MapInfo> )
{	let dot  = MapObj::Dot ( None );
	let wall = MapObj::Wall;
	let half_w = MAP_WIDTH  / 2;
	let half_h = MAP_HEIGHT / 2;
	let short_side = if half_w >= half_h { half_h } else { half_w };

	//二次元配列の中の矩形領域を置き換える無名関数
	let mut box_fill = | obj, ( mut x1, mut y1), ( mut x2, mut y2 ) |
	{	if x1 > x2 { std::mem::swap( &mut x1, &mut x2 ) }
		if y1 > y2 { std::mem::swap( &mut y1, &mut y2 ) }
		for y in y1..=y2
		{	for x in x1..=x2
			{	map.array[ x as usize ][ y as usize ] = obj;
			}
		}
	};

	//基本的な回廊
	for xy in 0..=short_side
	{	let obj = if xy % 2 == 0 { wall } else { dot };
		let xy1 = ( xy, xy );
		let xy2 = ( MAP_WIDTH - 1 - xy, MAP_HEIGHT - 1 - xy );
		box_fill( obj, xy1, xy2 );
	}

	//十字の通路
	let xy1 = ( 1, half_h );
	let xy2 = ( MAP_WIDTH - 2, MAP_HEIGHT - 1 - half_h );
	box_fill( dot, xy1, xy2 );
	let xy1 = ( half_w, 1 );
	let xy2 = ( MAP_WIDTH - 1 - half_w, MAP_HEIGHT - 2 );
	box_fill( dot, xy1, xy2 );

	//十字通路の中央に壁を作る
	if short_side % 2 == 0
	{	if half_w >= half_h
		{	if MAP_HEIGHT % 2 != 0
			{	let xy1 = ( short_side, short_side );
				let xy2 = ( MAP_WIDTH - 1 - short_side, short_side );
				box_fill( wall, xy1, xy2 );
			}
		}
		else if MAP_WIDTH % 2 != 0
		{	let xy1 = ( short_side, short_side );
			let xy2 = ( short_side, MAP_HEIGHT - 1 - short_side );
			box_fill( wall, xy1, xy2 );
		}
	}

	//ランダムに壁を通路にする
	let mut rng = rand::thread_rng();
	let n = MAP_HEIGHT * MAP_WIDTH / 10; //例: 25*40/10=100
	for _ in 0..n
	{	let x = rng.gen_range( 2..MAP_WIDTH  - 2 );
		let y = rng.gen_range( 2..MAP_HEIGHT - 2 );
		map.array[ x ][ y ] = dot;
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//壁用のスプライトバンドルを生成
fn sprite_wall( ( x, y ): ( f32, f32 ), asset_svr: &Res<AssetServer> ) -> SpriteBundle
{	let position = Vec3::new( x, y, SPRITE_MAP_DEPTH );
	let square   = Vec2::new( SPRITE_WALL_PIXEL, SPRITE_WALL_PIXEL );

	let texture   = asset_svr.load( IMAGE_SPRITE_WALL );
	let transform = Transform::from_translation( position );
	let sprite    = Sprite { custom_size: Some( square ), ..default() };

	SpriteBundle { texture, transform, sprite, ..default() }
}

//ドット用のスプライトバンドルを生成
fn sprite_dot( ( x, y ): ( f32, f32 ) ) -> ShapeBundle
{	let circle    = &shapes::Circle { radius: SPRITE_DOT_RAIDUS, ..shapes::Circle::default() };
	let drawmode  = DrawMode::Fill( FillMode { options: FillOptions::default(), color: SPRITE_DOT_COLOR } );
	let transform = Transform::from_translation( Vec3::new( x, y, SPRITE_MAP_DEPTH ) );

	GeometryBuilder::build_as( circle, drawmode, transform )
}

//End of code.