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
}

//End of code.