use super::*;

//壁とドットのComponent
#[derive( Component )] pub struct SpriteWall;
#[derive( Component )] pub struct SpriteDot;
#[derive( Component )] pub struct TextUiNumTile ( pub Grid ); //Debug用の数値表示タイル

////////////////////////////////////////////////////////////////////////////////////////////////////

//MAPのマスの状態           0b76543210
const BIT_WALL     : usize = 0b00000001; //壁
const BIT_WAY_RIGHT: usize = 0b00000010; //右に道
const BIT_WAY_LEFT : usize = 0b00000100; //左に道
const BIT_WAY_DOWN : usize = 0b00001000; //上に道
const BIT_WAY_UP   : usize = 0b00010000; //下に道

////////////////////////////////////////////////////////////////////////////////////////////////////

//マップの構造体
#[derive( Resource )]
pub struct Map
{   pub rng            : rand::prelude::StdRng,     //マップ生成専用の乱数生成器(マップに再現性を持たせるため)
    map_bits           : Vec<Vec<usize>>,           //マップの各グリッドの状態をbitで保存
    dot_entities       : Vec<Vec<Option<Entity>>>,  //ドットをdespawnする際に使うEntityIDを保存
    pub remaining_dots : i32,                       //マップに残っているドットの数
    dummy_o_entity_none: Option<Entity>,            //o_entity_mut()の範囲外アクセスで&mut Noneを返すために使用
    demo               : DemoParams,                //demo用の情報
}
#[derive( Default )]
struct DemoParams
{   dots_rect : GridRect,                               //dotsを内包する最小の矩形
    dots_sum_x: [ i32; MAP_GRIDS_WIDTH  as usize ], //列に残っているdotsを数えた配列
    dots_sum_y: [ i32; MAP_GRIDS_HEIGHT as usize ], //行に残っているdotsを数えた配列
}
#[derive( Default )]
struct GridRect
{   min: Grid,
    max: Grid,
}

//マップ構造体の初期化
impl Default for Map
{   fn default() -> Self
    {   //develpでは定数を乱数シードにする。releaseではランダムにする。
        let seed = if cfg!( debug_assertions )
        {   1234567890
        }
        else
        {   rand::thread_rng().gen::<u64>()
        };

        Self
        {   rng                : StdRng::seed_from_u64( seed ),
            map_bits           : vec![ vec![ 0   ; MAP_GRIDS_HEIGHT as usize ]; MAP_GRIDS_WIDTH as usize ],
            dot_entities       : vec![ vec![ None; MAP_GRIDS_HEIGHT as usize ]; MAP_GRIDS_WIDTH as usize ],
            remaining_dots     : 0,
            dummy_o_entity_none: None,
            demo               : DemoParams::default(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//マップ構造体のメソッド
//メソッド経由にすることで配列の範囲外アクセスもパニックさせず意図した値を返す。
//構造体メンバーに直接アクセスさせない(構造体メンバーはNot pub)。
impl Map
{   fn bits    ( &    self, grid: Grid ) ->      usize {      self.map_bits[ grid.x as usize ][ grid.y as usize ] }
    fn bits_mut( &mut self, grid: Grid ) -> &mut usize { &mut self.map_bits[ grid.x as usize ][ grid.y as usize ] }

    fn is_inside( &self, grid: Grid ) -> bool
    {   MAP_GRIDS_RANGE_X.contains( &grid.x ) && MAP_GRIDS_RANGE_Y.contains( &grid.y )
    }

    pub fn set_wall( &mut self, grid: Grid )
    {   if ! self.is_inside( grid ) { return }
        *self.bits_mut( grid ) |=   BIT_WALL; //壁フラグON
    }
    pub fn set_passage( &mut self, grid: Grid )
    {   if ! self.is_inside( grid ) { return }
        *self.bits_mut( grid ) &= ! BIT_WALL; //壁フラグOFF
    }

    pub fn is_wall( &self, grid: Grid ) -> bool
    {   if ! self.is_inside( grid ) { return true } //範囲外は壁
        self.bits( grid ) & BIT_WALL != 0
    }
    pub fn is_passage( &self, grid: Grid ) -> bool
    {   if ! self.is_inside( grid ) { return false } //範囲外は通路ではない
        self.bits( grid ) & BIT_WALL == 0
    }

    pub fn o_entity( &self, grid: Grid ) -> Option<Entity>
    {   if ! self.is_inside( grid ) { return None } //範囲外はOption::Noneを返す
        self.dot_entities[ grid.x as usize ][ grid.y as usize ]
    }
    pub fn o_entity_mut( &mut self, grid: Grid ) -> &mut Option<Entity>
    {   if ! self.is_inside( grid ) { return &mut self.dummy_o_entity_none } //範囲外は&mut Option::Noneを返す
        &mut self.dot_entities[ grid.x as usize ][ grid.y as usize ]
    }

    pub fn init_byways_bit( &mut self )
    {   for y in MAP_GRIDS_RANGE_Y
        {   for x in MAP_GRIDS_RANGE_X
            {   let grid = Grid::new( x, y );
                if self.is_passage( grid + DxDy::Right ) { *self.bits_mut( grid ) |= BIT_WAY_RIGHT } else { *self.bits_mut( grid ) &= ! BIT_WAY_RIGHT }
                if self.is_passage( grid + DxDy::Left  ) { *self.bits_mut( grid ) |= BIT_WAY_LEFT  } else { *self.bits_mut( grid ) &= ! BIT_WAY_LEFT  }
                if self.is_passage( grid + DxDy::Down  ) { *self.bits_mut( grid ) |= BIT_WAY_DOWN  } else { *self.bits_mut( grid ) &= ! BIT_WAY_DOWN  }
                if self.is_passage( grid + DxDy::Up    ) { *self.bits_mut( grid ) |= BIT_WAY_UP    } else { *self.bits_mut( grid ) &= ! BIT_WAY_UP    }
            }
        }
    }

    pub fn get_byways_list( &self, grid: Grid ) -> Vec<DxDy>
    {   let mut vec = Vec::<DxDy>::with_capacity( 4 );
        if self.is_inside( grid )
        {   let bits = self.bits( grid );
            if bits & BIT_WAY_RIGHT != 0 { vec.push( DxDy::Right ) }
            if bits & BIT_WAY_LEFT  != 0 { vec.push( DxDy::Left  ) }
            if bits & BIT_WAY_DOWN  != 0 { vec.push( DxDy::Down  ) }
            if bits & BIT_WAY_UP    != 0 { vec.push( DxDy::Up    ) }
        }
        vec //範囲外は空になる（最外壁の外の座標だから上下左右に道はない）
    }

    pub fn init_demo_params( &mut self )
    {   //init時はdotのEntityがspawnされていないかもなので、数える対象は道にする
        MAP_GRIDS_RANGE_Y.for_each
        (   | y |
            self.demo.dots_sum_y[ y as usize ] =
            {   MAP_GRIDS_RANGE_X
                .filter( | &x | self.is_passage( Grid::new( x, y ) ) )
                .count() as i32
            }
        );
        MAP_GRIDS_RANGE_X.for_each
        (   | x |
            self.demo.dots_sum_x[ x as usize ] =
            {   MAP_GRIDS_RANGE_Y
                .filter( | &y | self.is_passage( Grid::new( x, y ) ) )
                .count() as i32
            }
        );

        //dotsを内包する最小の矩形は決め打ちでいい(Mapをそう作っているから)
        self.demo.dots_rect = GridRect
        {   min: Grid::new( 1, 1 ),
            max: Grid::new( MAP_GRIDS_WIDTH - 2, MAP_GRIDS_HEIGHT - 2 ),
        };
    }

    pub fn update_demo_params( &mut self, grid: Grid )
    {   //プレイヤーの位置の列・行のdotsを減らす
        self.demo.dots_sum_x[ grid.x as usize ] -= 1;
        self.demo.dots_sum_y[ grid.y as usize ] -= 1;

        //dotsを内包する最小の矩形を更新する
        let mut i = 0;
        for _ in MAP_GRIDS_RANGE_X
        {   if self.demo.dots_sum_x[ i as usize ] != 0 { break }
            i += 1;
        }
        self.demo.dots_rect.min.x = i;
        i = 0;
        for _ in MAP_GRIDS_RANGE_Y
        {   if self.demo.dots_sum_y[ i as usize ] != 0 { break }
            i += 1;
        }
        self.demo.dots_rect.min.y = i;

        i = MAP_GRIDS_WIDTH - 1;
        for _ in MAP_GRIDS_RANGE_X
        {   if self.demo.dots_sum_x[ i as usize ] != 0 { break }
            i -= 1;
        }
        self.demo.dots_rect.max.x = i;
        i = MAP_GRIDS_HEIGHT - 1;
        for _ in MAP_GRIDS_RANGE_Y
        {   if self.demo.dots_sum_y[ i as usize ] != 0 { break }
            i -= 1;
        }
        self.demo.dots_rect.max.y = i;
    }

    pub fn is_inside_dots_rect( &self, grid: Grid ) -> bool
    {   let Grid { x: x1, y: y1 } = self.demo.dots_rect.min;
        let Grid { x: x2, y: y2 } = self.demo.dots_rect.max;

        ( x1..=x2 ).contains( &grid.x ) && ( y1..=y2 ).contains( &grid.y )
    }

    pub fn how_far_to_dots_rect( &self, grid: Grid ) -> i32
    {   let Grid { x: x1, y: y1 } = self.demo.dots_rect.min;
        let Grid { x: x2, y: y2 } = self.demo.dots_rect.max;

        let dx = if grid.x < x1 { x1 - grid.x } else if grid.x > x2 { grid.x - x2 } else { 0 };
        let dy = if grid.y < y1 { y1 - grid.y } else if grid.y > y2 { grid.y - y2 } else { 0 };

        dx + dy
    }

    //debug用スプライトの表示座標等を算出する
    #[cfg( debug_assertions )]
    pub fn debug_pixel_demo_rect( &self ) -> ( f32, f32, f32, f32 )
    {   let px_min = self.demo.dots_rect.min.into_pixel_map();
        let px_max = self.demo.dots_rect.max.into_pixel_map();

        let px_w = px_max.x - px_min.x;
        let px_h = px_min.y - px_max.y; //pixelはY軸が逆向き
        let px_x = px_min.x + px_w / 2.0;
        let px_y = px_max.y + px_h / 2.0; //pixelはY軸が逆向き

        ( px_x, px_y, px_w + PIXELS_PER_GRID, px_h + PIXELS_PER_GRID )
    }
}

//End of code.