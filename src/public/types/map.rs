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

//マップの構造体
#[derive( Resource )]
pub struct Map
{   pub rng            : rand::prelude::StdRng,     //マップ生成専用の乱数生成器(マップに再現性を持たせるため)
    bit_flags          : Vec<Vec<usize>>,           //マップの各グリッドの状態をbitで保存
    dot_entities       : Vec<Vec<Option<Entity>>>,  //ドットをdespawnする際に使うEntityIDを保存
    pub remaining_dots : i32,                       //マップに残っているドットの数
    dummy_o_entity_none: Option<Entity>,            //o_entity_mut()の範囲外アクセスで&mut Noneを返すために使用
    // pub demo           : DemoParams,                //demo用の情報
}

// #[derive( Default )]
// pub struct DemoParams
// {   dots_rect : IVec2Rect,                          //dotsを内包する最小の矩形
//     dots_sum_x: [ i32; MAP_GRIDS_WIDTH  as usize ], //列に残っているdotsを数えた配列
//     dots_sum_y: [ i32; MAP_GRIDS_HEIGHT as usize ], //行に残っているdotsを数えた配列
// }

// #[derive( Default )]
// struct IVec2Rect
// {   min: IVec2,
//     max: IVec2,
// }

//マップ構造体の初期化
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
            // demo               : DemoParams::default(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

//マップ構造体のメソッド
//メソッド経由にすることで配列の範囲外アクセスもパニックさせず意図した値を返す。
//構造体メンバーに直接アクセスさせない(構造体メンバーはNot pub)。
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

    // pub fn o_entity( &self, grid: IVec2 ) -> Option<Entity>
    // {   if ! self.is_inside( grid ) { return None } //範囲外はOption::Noneを返す
    //     self.dot_entities[ grid.x as usize ][ grid.y as usize ]
    // }
    pub fn o_entity_mut( &mut self, grid: IVec2 ) -> &mut Option<Entity>
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

//demo用情報のメソッド
// impl DemoParams
// {   pub fn dots_sum_x( &self, x: i32 ) -> i32 { self.dots_sum_x[ x as usize ] }
//     pub fn dots_sum_y( &self, y: i32 ) -> i32 { self.dots_sum_y[ y as usize ] }
//     pub fn dots_sum_x_mut( &mut self, x: i32 ) -> &mut i32 { &mut self.dots_sum_x[ x as usize ] }
//     pub fn dots_sum_y_mut( &mut self, y: i32 ) -> &mut i32 { &mut self.dots_sum_y[ y as usize ] }

//     pub fn dots_rect_min( &self ) ->  IVec2 { self.dots_rect.min }
//     pub fn dots_rect_max( &self ) ->  IVec2 { self.dots_rect.max }
//     pub fn dots_rect_min_mut( &mut self ) ->  &mut IVec2 { &mut self.dots_rect.min }
//     pub fn dots_rect_max_mut( &mut self ) ->  &mut IVec2 { &mut self.dots_rect.max }

//     //自機がdotを食べたらdemo用パラメータを更新する
//     pub fn update_params( &mut self, grid: IVec2 )
//     {   //プレイヤーの位置の列・行のdotsを減らす
//         *self.dots_sum_x_mut( grid.x ) -= 1;
//         *self.dots_sum_y_mut( grid.y ) -= 1;

//         //dotsを内包する最小の矩形のminを更新する
//         let ( mut x, mut y ) = ( 0, 0 );
//         for _ in MAP_GRIDS_X_RANGE
//         {   if self.dots_sum_x( x ) != 0 { break } else { x += 1; }
//         }
//         for _ in MAP_GRIDS_Y_RANGE
//         {   if self.dots_sum_y( y ) != 0 { break } else { y += 1; }
//         }
//         *self.dots_rect_min_mut() = IVec2::new( x, y );

//         //dotsを内包する最小の矩形のmaxを更新する
//         ( x, y ) = ( MAP_GRIDS_WIDTH - 1, MAP_GRIDS_HEIGHT - 1 );
//         for _ in MAP_GRIDS_X_RANGE
//         {   if self.dots_sum_x( x ) != 0 { break } else { x -= 1; }
//         }
//         for _ in MAP_GRIDS_Y_RANGE
//         {   if self.dots_sum_y( y ) != 0 { break } else { y -= 1; }
//         }
//         *self.dots_rect_max_mut() = IVec2::new( x, y );
//     }

//     //指定のマスが、残dotsの最小矩形の中か？
//     pub fn is_inside_rect( &self, grid: IVec2 ) -> bool
//     {   let IVec2 { x: x1, y: y1 } = self.dots_rect_min();
//         let IVec2 { x: x2, y: y2 } = self.dots_rect_max();

//         ( x1..=x2 ).contains( &grid.x ) && ( y1..=y2 ).contains( &grid.y )
//     }

//     //指定のマスから残dotsの最小矩形までの単純距離(dx+dy)を求める
//     pub fn how_far_to_rect( &self, grid: IVec2 ) -> i32
//     {   let IVec2 { x: x1, y: y1 } = self.dots_rect_min();
//         let IVec2 { x: x2, y: y2 } = self.dots_rect_max();

//         let dx = if grid.x < x1 { x1 - grid.x } else if grid.x > x2 { grid.x - x2 } else { 0 };
//         let dy = if grid.y < y1 { y1 - grid.y } else if grid.y > y2 { grid.y - y2 } else { 0 };

//         dx + dy
//     }
// }

////////////////////////////////////////////////////////////////////////////////

//End of code.