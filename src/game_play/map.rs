use super::*;

//新マップのデータを作る
pub fn make_new_data
(   mut map   : ResMut<Map>,
    mut record: ResMut<Record>,
)
{   let half_w = MAP_GRIDS_WIDTH  / 2;
    let half_h = MAP_GRIDS_HEIGHT / 2;
    let short_side = if half_w >= half_h { half_h } else { half_w };

    //無名関数：二次元配列の矩形領域を指定の値によって埋める
    enum Obj { Wall, Passage }
    let mut box_fill =
    | obj, ( mut x1, mut y1), ( mut x2, mut y2 ) |
    {   if x1 > x2 { std::mem::swap( &mut x1, &mut x2 ) }
        if y1 > y2 { std::mem::swap( &mut y1, &mut y2 ) }
        for y in y1..=y2
        {   for x in x1..=x2
            {   let grid = Grid::new( x, y );
                match obj
                {   Obj::Wall    => map.set_wall   ( grid ),
                    Obj::Passage => map.set_passage( grid ),
                }
            }
        }
    };

    //基本的な回廊
    for xy in 0..=short_side
    {   let obj = if xy % 2 == 0 { Obj::Wall } else { Obj::Passage };
        let xy1 = ( xy, xy );
        let xy2 = ( MAP_GRIDS_WIDTH - 1 - xy, MAP_GRIDS_HEIGHT - 1 - xy );
        box_fill( obj, xy1, xy2 );
    }

    //十字の通路
    let xy1 = ( 1, half_h );
    let xy2 = ( MAP_GRIDS_WIDTH - 2, MAP_GRIDS_HEIGHT - 1 - half_h );
    box_fill( Obj::Passage, xy1, xy2 );
    let xy1 = ( half_w, 1 );
    let xy2 = ( MAP_GRIDS_WIDTH - 1 - half_w, MAP_GRIDS_HEIGHT - 2 );
    box_fill( Obj::Passage, xy1, xy2 );

    //十字通路の中央に壁を作る
    if short_side % 2 == 0
    {   if half_w >= half_h
        {   if MAP_GRIDS_HEIGHT % 2 != 0
            {   let xy1 = ( short_side, short_side );
                let xy2 = ( MAP_GRIDS_WIDTH - 1 - short_side, short_side );
                box_fill( Obj::Wall, xy1, xy2 );
            }
        }
        else if MAP_GRIDS_WIDTH % 2 != 0
        {   let xy1 = ( short_side, short_side );
            let xy2 = ( short_side, MAP_GRIDS_HEIGHT - 1 - short_side );
            box_fill( Obj::Wall, xy1, xy2 );
        }
    }

    //ランダムに壁を通路に置き換える
    let n = MAP_GRIDS_WIDTH * MAP_GRIDS_HEIGHT / 10; //例: 40☓25／10＝100
    for _ in 0..n
    {   let x = map.rng.gen_range( 2..MAP_GRIDS_WIDTH  - 2 );
        let y = map.rng.gen_range( 2..MAP_GRIDS_HEIGHT - 2 );
        map.set_passage( Grid::new( x, y ) );
    }

    //新マップを作ったらステージ数を＋１する
    record.stage += 1;
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//スプライトをspawnしてマップを表示する
pub fn spawn_sprite
(   q1: Query<Entity, With<SpriteWall>>,
    q2: Query<Entity, With<SpriteDot>>,
    mut map: ResMut<Map>,
    mut cmds: Commands,
    asset_svr: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
)
{   //スプライトがあれば削除する
    q1.for_each( | id | cmds.entity( id ).despawn_recursive() );
    q2.for_each( | id | cmds.entity( id ).despawn_recursive() );

    //壁とドットのスプライトを配置する
    let custom_size = Some( Pixel::new( PIXELS_PER_GRID, PIXELS_PER_GRID ) );
    let radius      = PIXELS_PER_GRID * MAGNIFY_SPRITE_DOT;
    map.remaining_dots = 0;
    for y in MAP_GRIDS_RANGE_Y
    {   for x in MAP_GRIDS_RANGE_X
        {   let grid = Grid::new( x, y );
            let pixel = grid.into_pixel_map();

            if map.is_wall( grid )
            {   cmds
                .spawn_bundle( SpriteBundle::default() )
                .insert( Sprite { custom_size, ..default() } )
                .insert( asset_svr.load( ASSETS_SPRITE_BRICK_WALL ) as Handle<Image> )
                .insert( Transform::from_translation( pixel.extend( DEPTH_SPRITE_BRICK_WALL ) ) )
                .insert( SpriteWall )
                ;
            }

            if map.is_passage( grid )
            {   let circle = MaterialMesh2dBundle //type annotations neededが出ないからこの書き方が良い
                {   mesh: meshes.add( shape::Circle::new( radius ).into() ).into(),
                    material: materials.add( ColorMaterial::from( COLOR_SPRITE_DOT ) ),
                    ..default()
                };
                let id =
                cmds
                .spawn_bundle( circle )
                .insert( Transform::from_translation( pixel.extend( DEPTH_SPRITE_DOT ) ) )
                .insert( SpriteDot )
                .id()
                ;
                *map.o_entity_mut( grid ) = Some ( id ); //idを保存(プレー中にdespawnするため)
                map.remaining_dots += 1; //ドットを数える
            }
        }
    }
}

//End of code.