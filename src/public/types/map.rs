use super::*;

////////////////////////////////////////////////////////////////////////////////

//壁とドットのComponent
#[derive( Component )] pub struct SpriteWall;
#[derive( Component )] pub struct SpriteDot;

////////////////////////////////////////////////////////////////////////////////

//マスの状態                  0b76543210
const BIT_WALL     : usize = 0b00000001; //壁
const BIT_WAY_RIGHT: usize = 0b00000010; //右に道
const BIT_WAY_LEFT : usize = 0b00000100; //左に道
const BIT_WAY_DOWN : usize = 0b00001000; //上に道
const BIT_WAY_UP   : usize = 0b00010000; //下に道

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

////////////////////////////////////////////////////////////////////////////////

//マップのメソッド
//メソッド経由にすることで配列の範囲外アクセスもパニックさせず意図した値を返す。
//構造体メンバーに直接アクセスさせない(構造体メンバーは原則Not pub)。
impl Map
{   fn bits    ( &    self, grid: IVec2 ) ->      usize {      self.bit_flags[ grid.x as usize ][ grid.y as usize ] }
    fn bits_mut( &mut self, grid: IVec2 ) -> &mut usize { &mut self.bit_flags[ grid.x as usize ][ grid.y as usize ] }

    fn is_inside( &self, grid: IVec2 ) -> bool
    {   MAP_GRIDS_X_RANGE.contains( &grid.x ) && MAP_GRIDS_Y_RANGE.contains( &grid.y )
    }

    pub fn set_wall( &mut self, grid: IVec2 )
    {   if ! self.is_inside( grid ) { return }
        *self.bits_mut( grid ) |=   BIT_WALL; //壁フラグON
    }
    pub fn set_passage( &mut self, grid: IVec2 )
    {   if ! self.is_inside( grid ) { return }
        *self.bits_mut( grid ) &= ! BIT_WALL; //壁フラグOFF
    }

    pub fn is_wall( &self, grid: IVec2 ) -> bool
    {   if ! self.is_inside( grid ) { return true } //範囲外は壁
        self.bits( grid ) & BIT_WALL != 0
    }
    pub fn is_passage( &self, grid: IVec2 ) -> bool
    {   if ! self.is_inside( grid ) { return false } //範囲外は通路ではない
        self.bits( grid ) & BIT_WALL == 0
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
                if self.is_passage( grid + News::East  ) { *self.bits_mut( grid ) |= BIT_WAY_RIGHT } else { *self.bits_mut( grid ) &= ! BIT_WAY_RIGHT }
                if self.is_passage( grid + News::West  ) { *self.bits_mut( grid ) |= BIT_WAY_LEFT  } else { *self.bits_mut( grid ) &= ! BIT_WAY_LEFT  }
                if self.is_passage( grid + News::South ) { *self.bits_mut( grid ) |= BIT_WAY_DOWN  } else { *self.bits_mut( grid ) &= ! BIT_WAY_DOWN  }
                if self.is_passage( grid + News::North ) { *self.bits_mut( grid ) |= BIT_WAY_UP    } else { *self.bits_mut( grid ) &= ! BIT_WAY_UP    }
            }
        }
    }

    pub fn get_byways_list( &self, grid: IVec2 ) -> Vec<News>
    {   let mut vec = Vec::<News>::with_capacity( 4 );
        if self.is_inside( grid )
        {   let bits = self.bits( grid );
            if bits & BIT_WAY_RIGHT != 0 { vec.push( News::East  ) }
            if bits & BIT_WAY_LEFT  != 0 { vec.push( News::West  ) }
            if bits & BIT_WAY_DOWN  != 0 { vec.push( News::South ) }
            if bits & BIT_WAY_UP    != 0 { vec.push( News::North ) }
        }
        vec //範囲外は空になる（最外壁の外の座標だから上下左右に道はない）
    }
}

////////////////////////////////////////////////////////////////////////////////

//demo用のマップ情報Resource
#[derive( Resource, Default )]
pub struct DemoMapParams
{   dots_rect : IVec2Rect,                          //dotsを内包する最小の矩形
    dots_sum_x: [ i32; MAP_GRIDS_WIDTH  as usize ], //列に残っているdotsを数えた配列
    dots_sum_y: [ i32; MAP_GRIDS_HEIGHT as usize ], //行に残っているdotsを数えた配列
}

#[derive( Default )]
struct IVec2Rect { min: IVec2, max: IVec2 }

impl DemoMapParams
{   pub fn dots_sum_x    ( &    self, x: i32 ) ->      i32 {      self.dots_sum_x[ x as usize ] }
    pub fn dots_sum_x_mut( &mut self, x: i32 ) -> &mut i32 { &mut self.dots_sum_x[ x as usize ] }
    pub fn dots_sum_y    ( &    self, y: i32 ) ->      i32 {      self.dots_sum_y[ y as usize ] }
    pub fn dots_sum_y_mut( &mut self, y: i32 ) -> &mut i32 { &mut self.dots_sum_y[ y as usize ] }

    pub fn dots_rect_min    ( &    self ) ->       IVec2 {      self.dots_rect.min }
    pub fn dots_rect_min_mut( &mut self ) ->  &mut IVec2 { &mut self.dots_rect.min }
    pub fn dots_rect_max    ( &    self ) ->       IVec2 {      self.dots_rect.max }
    pub fn dots_rect_max_mut( &mut self ) ->  &mut IVec2 { &mut self.dots_rect.max }
}

////////////////////////////////////////////////////////////////////////////////

//End of code.