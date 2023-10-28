use super::*;

////////////////////////////////////////////////////////////////////////////////

//マップのデータを作る
pub fn make_new_data
(   opt_map  : Option<ResMut<Map>>,
    opt_stage: Option<ResMut<Stage>>,
)
{   let Some ( mut map ) = opt_map else { return };
    let Some ( mut stage ) = opt_stage else { return };
    
    let half_w = MAP_GRIDS_WIDTH  / 2;
    let half_h = MAP_GRIDS_HEIGHT / 2;
    let short_side = if half_w >= half_h { half_h } else { half_w };

    //二次元配列の矩形領域を指定の値によって埋める無名関数
    enum Obj { Wall, Passage }
    let mut box_fill =
    | obj, ( mut x1, mut y1), ( mut x2, mut y2 ) |
    {   if x1 > x2 { std::mem::swap( &mut x1, &mut x2 ) }
        if y1 > y2 { std::mem::swap( &mut y1, &mut y2 ) }
        for y in y1..=y2
        {   for x in x1..=x2
            {   let grid = IVec2::new( x, y );
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
        map.set_passage( IVec2::new( x, y ) );
    }

    //付随する情報の初期化
    *stage.get_mut() += 1;  //新マップを作ったらステージ数を＋１する
    map.init_byways_bit();  //全グリッドに対し、四方の通路の状態をセットする
    // map.init_demo_params(); //demo用の情報を準備する
}

// impl Map
// {   //マップ作成時にdemo用パラメータを初期化する
//     pub fn init_demo_params( &mut self )
//     {   //dotではなく道を数える(初期状態では必ず道にdotがある)
//         MAP_GRIDS_Y_RANGE.for_each
//         (   | y |
//             *self.demo.dots_sum_y_mut( y ) =
//             {   MAP_GRIDS_X_RANGE
//                 .filter( | &x | self.is_passage( IVec2::new( x, y ) ) )
//                 .count() as i32
//             }
//         );
//         MAP_GRIDS_X_RANGE.for_each
//         (   | x |
//             *self.demo.dots_sum_x_mut( x ) =
//             {   MAP_GRIDS_Y_RANGE
//                 .filter( | &y | self.is_passage( IVec2::new( x, y ) ) )
//                 .count() as i32
//             }
//         );

//         //dotsを内包する最小の矩形は決め打ちでいい(Mapをそう作っているから)
//         *self.demo.dots_rect_min_mut() = IVec2::new( 1, 1 );
//         *self.demo.dots_rect_max_mut() = IVec2::new( MAP_GRIDS_WIDTH - 2, MAP_GRIDS_HEIGHT - 2 );
//     }
// }

////////////////////////////////////////////////////////////////////////////////

type WithMapEntities = Or< ( With<SpriteWall>, With<SpriteDot> ) >;

//スプライトをspawnしてマップを表示する
pub fn spawn_sprite
(   qry_entity: Query<Entity, WithMapEntities>,
    opt_map: Option<ResMut<Map>>,
    mut cmds: Commands,
    asset_svr: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
)
{   let Some ( mut map ) = opt_map else { return };

    //スプライトがあれば削除する
    qry_entity.for_each( | id | cmds.entity( id ).despawn_recursive() );

    //壁とドットのスプライトを配置する
    let custom_size = Some( SIZE_GRID );
    let radius      = PIXELS_PER_GRID * MAGNIFY_SPRITE_DOT;
    map.remaining_dots = 0;
    for y in MAP_GRIDS_Y_RANGE
    {   for x in MAP_GRIDS_X_RANGE
        {   let grid = IVec2::new( x, y );
            let pixel = grid.to_sprite_pixels() + ADJUSTER_MAP_SPRITES;

            if map.is_wall( grid )
            {   cmds
                .spawn( ( SpriteBundle::default(), SpriteWall ) )
                .insert( Sprite { custom_size, ..default() } )
                .insert( asset_svr.load( ASSETS_SPRITE_BRICK_WALL ) as Handle<Image> )
                .insert( Transform::from_translation( pixel.extend( DEPTH_SPRITE_BRICK_WALL ) ) )
                .with_children
                (   | _cmds |
                    {   #[cfg( debug_assertions )]
                        {   let value = format!( "{:02}\n{:02}", x, y ).to_string();
                            let style = TextStyle
                            {   font     : asset_svr.load( ASSETS_FONT_PRESSSTART2P_REGULAR ),
                                font_size: PIXELS_PER_GRID * 0.3,
                                color    : Color::YELLOW,
                            };
                            let sections  = vec![ TextSection { value, style } ];
                            let alignment = TextAlignment::Center;
        
                            _cmds.spawn( Text2dBundle::default() )
                            .insert( Text { sections, alignment, ..default() } )
                            .insert( Transform::from_translation( Vec3::Z ) )
                            ;
                        }
                    }
                );
            }

            if map.is_passage( grid )
            {   let circle = MaterialMesh2dBundle //type annotations neededが出ないからこの書き方が良い
                {   mesh: meshes.add( shape::Circle::new( radius ).into() ).into(),
                    material: materials.add( ColorMaterial::from( COLOR_SPRITE_DOT ) ),
                    ..default()
                };
                let id = cmds
                .spawn( ( circle, SpriteDot ) )
                .insert( Transform::from_translation( pixel.extend( DEPTH_SPRITE_DOT ) ) )
                .id()
                ;
                *map.opt_entity_mut( grid ) = Some ( id ); //idを保存(プレー中にdespawnするため)
                map.remaining_dots += 1; //ドットを数える
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

//End of code.