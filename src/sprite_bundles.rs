use super::*;

//タイルのスプライト
const TILE_PIXEL: f32   = PIXEL_PER_GRID - 1.;
const TILE_COLOR: Color = Color::rgb_linear( 0.25, 0.06, 0.04 );

//壁のスプライト
const WALL_PIXEL: f32   = PIXEL_PER_GRID;

//ドットのスプライト
const DOT_RAIDUS: f32   = PIXEL_PER_GRID / 14.;
const DOT_COLOR : Color = Color::WHITE;

//自機のスプライト
const PLAYER_PIXEL: f32   = PIXEL_PER_GRID / 2.5;
const PLAYER_COLOR: Color = Color::YELLOW;

//追手のスプライト
const CHASER_PIXEL: f32 = PIXEL_PER_GRID / 2.;
pub const CHASER_COUNT: usize = 4;
pub const CHASER_SPRITE_PARAMS: [ ( Color, ( usize, usize ) ); CHASER_COUNT ] =
[	( Color::RED  , ( 1    , 1     ) ),
	( Color::BLUE , ( 1    , MAX_Y ) ),
	( Color::GREEN, ( MAX_X, 1     ) ),
	( Color::PINK , ( MAX_X, MAX_Y ) ),
];
const MAX_X: usize = MAP_WIDTH  - 2;
const MAX_Y: usize = MAP_HEIGHT - 2;

//スプライトのZ軸の順位
const SPRITE_DEPTH_TILE  : f32 = 0.;
const SPRITE_DEPTH_MAZE  : f32 = 0.;
const SPRITE_DEPTH_PLAYER: f32 = 1.;
const SPRITE_DEPTH_CHASER: f32 = 2.;

////////////////////////////////////////////////////////////////////////////////

//タイルのスプライトバンドルを生成
pub fn sprite_tile
(	( x, y ): ( f32, f32 ),
	color_matl: &mut ResMut<Assets<ColorMaterial>>,
) -> SpriteBundle
{	let locate   = Vec3::new( x, y, SPRITE_DEPTH_TILE );
	let square   = Vec2::new( TILE_PIXEL, TILE_PIXEL );

	SpriteBundle
	{	material : color_matl.add( TILE_COLOR.into() ),
		transform: Transform::from_translation( locate ),
		sprite   : Sprite::new( square ),
		..Default::default()
	}
}

//壁用のスプライトバンドルを生成
pub fn sprite_wall
(	( x, y ): ( f32, f32 ),
	color_matl: &mut ResMut<Assets<ColorMaterial>>,
	asset_svr: &Res<AssetServer>,
) -> SpriteBundle
{	let texture_handle = asset_svr.load( SPRITE_WALL_FILE ).into();
	let locate   = Vec3::new( x, y, SPRITE_DEPTH_MAZE );
	let square   = Vec2::new( WALL_PIXEL, WALL_PIXEL );

	SpriteBundle
	{// material : color_matl.add( WALL_COLOR.into() ),
		material : color_matl.add( texture_handle ),
		transform: Transform::from_translation( locate ),
		sprite   : Sprite::new( square ),
		..Default::default()
	}
}

//ドット用のスプライトバンドルを生成
//Native
//#[cfg(not(target_arch = "wasm32"))]
pub fn sprite_dot
(	( x, y ): ( f32, f32 ),
	_color_matl: &mut ResMut<Assets<ColorMaterial>>,
) -> ShapeBundle
{	let locate   = Vec3::new( x, y, SPRITE_DEPTH_MAZE );

	let circle = &shapes::Circle { radius: DOT_RAIDUS, ..shapes::Circle::default() };
	GeometryBuilder::build_as
	(	circle,
		ShapeColors::new( DOT_COLOR ),
        DrawMode::Fill( FillOptions::default() ),
        Transform::from_translation( locate ),
    )
}
/*//WASM
#[cfg(target_arch = "wasm32")]
pub fn sprite_dot
(	( x, y ): ( f32, f32 ),
	color_matl: &mut ResMut<Assets<ColorMaterial>>,
) -> SpriteBundle
{	let locate   = Vec3::new( x, y, SPRITE_DEPTH_MAZE );
	let square   = Vec2::new( DOT_RAIDUS * 2., DOT_RAIDUS * 2. );

	SpriteBundle
	{	material : color_matl.add( DOT_COLOR.into() ),
		transform: Transform::from_translation( locate ),
		sprite   : Sprite::new( square ),
		..Default::default()
	}
}
*/
//自機のスプライトバンドルを生成
//Native
//#[cfg(not(target_arch = "wasm32"))]
pub fn sprite_player
(	( x, y ): ( f32, f32 ),
	_color_matl: &mut ResMut<Assets<ColorMaterial>>,
) -> ShapeBundle
{	let locate = Vec3::new( x, y, SPRITE_DEPTH_PLAYER );

	let triangle = &shapes::RegularPolygon
	{	sides: 3,
		feature: shapes::RegularPolygonFeature::Radius( PLAYER_PIXEL ),
		..shapes::RegularPolygon::default()
	};
	GeometryBuilder::build_as
	(	triangle,
		ShapeColors::new( PLAYER_COLOR ),
        DrawMode::Fill( FillOptions::default() ),
		Transform::from_translation( locate )
	)
}
/*//WASM
#[cfg(target_arch = "wasm32")]
pub fn sprite_player
(	( x, y ): ( f32, f32 ),
	color_matl: &mut ResMut<Assets<ColorMaterial>>,
) -> SpriteBundle
{	let locate = Vec3::new( x, y, SPRITE_DEPTH_PLAYER );
	let square = Vec2::new( PLAYER_PIXEL, PLAYER_PIXEL );

	SpriteBundle
	{	material : color_matl.add( PLAYER_COLOR.into() ),
		transform: Transform::from_translation( locate ),
		sprite   : Sprite::new( square ),
		..Default::default()
	}
}
*/
//追手のスプライトバンドルを生成
pub fn sprite_chaser
(	( x, y ): ( f32, f32 ),
	color: Color,
	color_matl: &mut ResMut<Assets<ColorMaterial>>,
) -> SpriteBundle
{	let locate   = Vec3::new( x, y, SPRITE_DEPTH_CHASER );
	let square   = Vec2::new( CHASER_PIXEL, CHASER_PIXEL );

	let mut sprite = SpriteBundle
	{	material : color_matl.add( color.into() ),
		transform: Transform::from_translation( locate ),
		sprite   : Sprite::new( square ),
		..Default::default()
	};

	//45°傾けて菱形に見せる
	let quat = Quat::from_rotation_z( 45_f32.to_radians() );
	sprite.transform.rotate( quat ); //.rotate()は()を返すのでメソッドチェーンできない

	sprite
}

//End of c#de.