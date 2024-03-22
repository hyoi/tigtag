use super::*;

////////////////////////////////////////////////////////////////////////////////

//マップのResource
#[derive( Resource )]
pub struct Map
{   pub rng            : rand::prelude::StdRng,    //マップ生成専用の乱数生成器(マップに再現性を持たせるため)
    bit_flags          : Vec<Vec<usize>>,          //マップの各グリッドの状態をbitで保存
    dot_entities       : Vec<Vec<Option<Entity>>>, //ドットをdespawnする際に使うEntityIDを保存
    pub remaining_dots : i32,                      //マップに残っているドットの数
    dummy_o_entity_none: Option<Entity>,           //o_entity_mut()の範囲外アクセスで&mut Noneを返すために使用
}

impl Default for Map
{   fn default() -> Self
    {   //develpでは定数を、releaseではランダムを乱数シードにする
        let seed_dev = 1234567890;
        let seed_rel = rand::thread_rng().gen::<u64>();
        let seed = if DEBUG() { seed_dev } else { seed_rel };

        Self
        {   rng                : StdRng::seed_from_u64( seed ),
            bit_flags          : vec![ vec![ 0   ; MAP_GRIDS_HEIGHT as usize ]; MAP_GRIDS_WIDTH as usize ],
            dot_entities       : vec![ vec![ None; MAP_GRIDS_HEIGHT as usize ]; MAP_GRIDS_WIDTH as usize ],
            remaining_dots     : 0,
            dummy_o_entity_none: None,
        }
    }
}

//マップのメソッド
//メソッド経由にすることで配列の範囲外アクセスもパニックさせず意図した値を返す。
//構造体メンバーに直接アクセスさせない(構造体メンバーは原則Not pub)。
impl Map
{   //非公開メソッド
    fn bits    ( &    self, grid: IVec2 ) ->      usize {      self.bit_flags[ grid.x as usize ][ grid.y as usize ] }
    fn bits_mut( &mut self, grid: IVec2 ) -> &mut usize { &mut self.bit_flags[ grid.x as usize ][ grid.y as usize ] }

    fn is_inside( &self, grid: IVec2 ) -> bool
    {   MAP_GRIDS_X_RANGE.contains( &grid.x ) && MAP_GRIDS_Y_RANGE.contains( &grid.y )
    }

    //非公開定数：マスの状態の定義
    const BIT_WALL     : usize = 0b00000001; //壁
    const BIT_WAY_RIGHT: usize = 0b00000010; //右に道
    const BIT_WAY_LEFT : usize = 0b00000100; //左に道
    const BIT_WAY_DOWN : usize = 0b00001000; //上に道
    const BIT_WAY_UP   : usize = 0b00010000; //下に道

    //公開メソッド
    pub fn set_wall( &mut self, grid: IVec2 )
    {   if ! self.is_inside( grid ) { return }
        *self.bits_mut( grid ) |=   Map::BIT_WALL; //壁フラグON
    }
    pub fn set_passage( &mut self, grid: IVec2 )
    {   if ! self.is_inside( grid ) { return }
        *self.bits_mut( grid ) &= ! Map::BIT_WALL; //壁フラグOFF
    }

    pub fn is_wall( &self, grid: IVec2 ) -> bool
    {   if ! self.is_inside( grid ) { return true } //範囲外は壁
        self.bits( grid ) & Map::BIT_WALL != 0
    }
    pub fn is_space( &self, grid: IVec2 ) -> bool
    {   if ! self.is_inside( grid ) { return false } //範囲外は通路ではない
        self.bits( grid ) & Map::BIT_WALL == 0
    }

    pub fn opt_entity( &self, grid: IVec2 ) -> Option<Entity>
    {   if ! self.is_inside( grid ) { return None } //範囲外はOption::Noneを返す
        self.dot_entities[ grid.x as usize ][ grid.y as usize ]
    }
    pub fn opt_entity_mut( &mut self, grid: IVec2 ) -> &mut Option<Entity>
    {   if ! self.is_inside( grid ) { return &mut self.dummy_o_entity_none } //範囲外は&mut Option::Noneを返す
        &mut self.dot_entities[ grid.x as usize ][ grid.y as usize ]
    }

    pub fn init_byways_bit( &mut self )
    {   for y in MAP_GRIDS_Y_RANGE
        {   for x in MAP_GRIDS_X_RANGE
            {   let grid = IVec2::new( x, y );
                if self.is_space( grid + News::East  ) { *self.bits_mut( grid ) |= Map::BIT_WAY_RIGHT } else { *self.bits_mut( grid ) &= ! Map::BIT_WAY_RIGHT }
                if self.is_space( grid + News::West  ) { *self.bits_mut( grid ) |= Map::BIT_WAY_LEFT  } else { *self.bits_mut( grid ) &= ! Map::BIT_WAY_LEFT  }
                if self.is_space( grid + News::South ) { *self.bits_mut( grid ) |= Map::BIT_WAY_DOWN  } else { *self.bits_mut( grid ) &= ! Map::BIT_WAY_DOWN  }
                if self.is_space( grid + News::North ) { *self.bits_mut( grid ) |= Map::BIT_WAY_UP    } else { *self.bits_mut( grid ) &= ! Map::BIT_WAY_UP    }
            }
        }
    }

    pub fn get_side_spaces_list( &self, grid: IVec2 ) -> Vec<News>
    {   let mut vec = Vec::<News>::with_capacity( 4 );
        if self.is_inside( grid )
        {   let bits = self.bits( grid );
            if bits & Map::BIT_WAY_RIGHT != 0 { vec.push( News::East  ) }
            if bits & Map::BIT_WAY_LEFT  != 0 { vec.push( News::West  ) }
            if bits & Map::BIT_WAY_DOWN  != 0 { vec.push( News::South ) }
            if bits & Map::BIT_WAY_UP    != 0 { vec.push( News::North ) }
        }
        vec //範囲外は空になる（最外壁の外の座標だから上下左右に道はない）
    }
}

////////////////////////////////////////////////////////////////////////////////

//壁とドットのComponent
#[derive( Component )] pub struct SpriteWall;
#[derive( Component )] pub struct SpriteDot;

//glamの型にメソッドを追加する準備
pub trait GridToPixelOnMap
{   fn to_vec2_on_game_map( &self ) -> Vec2;
}

//glamの型にメソッドを追加する
impl GridToPixelOnMap for IVec2
{   //マップと画面の座標調整値を加味してvec2へ変換する
    fn to_vec2_on_game_map( &self ) -> Vec2
    {   ( *self + ADJUST_MAP_ON_SCREEN ).to_vec2_on_screen()
    }
}

//ドットのスプライトの情報
const SPRITE_DOT_COLOR: Color = Color::rgb( 1.0, 1.0, 0.7 );
const SPRITE_DOT_SCALING: f32 = 0.08;

////////////////////////////////////////////////////////////////////////////////

//マップのデータを作る
pub fn make_new_data
(   opt_map   : Option<ResMut<Map>>,
    opt_record: Option<ResMut<Record>>,
)
{   let Some ( mut map ) = opt_map else { return };
    let Some ( mut record ) = opt_record else { return };

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
    *record.stage_mut() += 1; //新マップを作ったらステージ数を＋１する
    map.init_byways_bit();    //全グリッドに対し、四方の壁・通の状態をセットする
}

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
    qry_entity.iter().for_each( | id | cmds.entity( id ).despawn_recursive() );

    //壁とドットのスプライトを配置する
    let custom_size = Some( GRID_CUSTOM_SIZE );
    let radius      = PIXELS_PER_GRID * SPRITE_DOT_SCALING;
    map.remaining_dots = 0;

    for y in MAP_GRIDS_Y_RANGE
    {   for x in MAP_GRIDS_X_RANGE
        {   let grid = IVec2::new( x, y );
            let vec2 = grid.to_vec2_on_game_map();

            //壁のスプライト
            if map.is_wall( grid )
            {   let id = cmds.spawn( ( SpriteBundle::default(), SpriteWall ) )
                .insert( Sprite { custom_size, ..default() } )
                .insert( asset_svr.load( ASSETS_SPRITE_BRICK_WALL ) as Handle<Image> )
                .insert( Transform::from_translation( vec2.extend( DEPTH_SPRITE_BRICK_WALL ) ) )
                .id()
                ;

                //debug用
                if DEBUG()
                {   let value = format!( "{:02}\n{:02}", x, y ).to_string();
                    let style = TextStyle
                    {   font_size: PIXELS_PER_GRID * 0.4,
                        color    : Color::YELLOW,
                        ..default()
                    };
                    let sections = vec![ TextSection { value, style } ];
                    let justify = JustifyText::Center;

                    let child = cmds.spawn( Text2dBundle::default() )
                    .insert( Text { sections, justify, ..default() } )
                    .insert( Transform::from_translation( Vec3::Z ) )
                    .id()
                    ;
                    cmds.entity( id ).add_child( child );
                }
            }

            //ドットのスプライト
            if map.is_space( grid )
            {   let id = cmds.spawn
                (   MaterialMesh2dBundle
                    {   mesh: meshes.add( Circle::new( radius ).mesh().resolution( 64 ).build() ).into(),
                        material: materials.add( ColorMaterial::from( SPRITE_DOT_COLOR ) ),
                        transform: Transform::from_translation( vec2.extend( DEPTH_SPRITE_DOT ) ),
                        ..default()
                    }
                )
                .insert( SpriteDot )
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