use super::*;

//壁とドットのComponent
#[derive( Component )] pub struct SpriteWall;
#[derive( Component )] pub struct SpriteDot;
#[derive( Component )] pub struct TextUiNumTile ( pub Grid ); //Debug用の数値表示タイル

////////////////////////////////////////////////////////////////////////////////////////////////////

//MAPのマスの状態           0b76543210
const BIT_WALL   : usize = 0b00000001; //壁
const BIT_PASSAGE: usize = 0b00000010; //通路（広間ではない空間）

////////////////////////////////////////////////////////////////////////////////////////////////////

//マップの構造体
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
//メソッド経由にして範囲外アクセスも意図した値を返す。
//パニックさせないため構造体メンバーに直接アクセスさせない(Not pub)。
impl Map
{   fn bits    ( &    self, grid: Grid ) ->      usize {      self.map_bits[ grid.x as usize ][ grid.y as usize ] }
    fn bits_mut( &mut self, grid: Grid ) -> &mut usize { &mut self.map_bits[ grid.x as usize ][ grid.y as usize ] }

    pub fn set_wall( &mut self, grid: Grid )
    {   if MAP_GRIDS_RANGE_X.contains( &grid.x ) && MAP_GRIDS_RANGE_Y.contains( &grid.y )
        {   *self.bits_mut( grid ) &= ! BIT_PASSAGE;    //通路フラグOFF
            *self.bits_mut( grid ) |=   BIT_WALL;       //壁フラグON
        }
    }
    pub fn set_passage( &mut self, grid: Grid )
    {   if MAP_GRIDS_RANGE_X.contains( &grid.x ) && MAP_GRIDS_RANGE_Y.contains( &grid.y )
        {   *self.bits_mut( grid ) |=   BIT_PASSAGE;    //通路フラグON
            *self.bits_mut( grid ) &= ! BIT_WALL;       //壁フラグOFF
        }
    }

    pub fn is_wall( &self, grid: Grid ) -> bool
    {   if MAP_GRIDS_RANGE_X.contains( &grid.x ) && MAP_GRIDS_RANGE_Y.contains( &grid.y )
        {   self.bits( grid ) & BIT_WALL != 0
        }
        else { true } //範囲外は壁
    }
    pub fn is_passage( &self, grid: Grid ) -> bool
    {   if MAP_GRIDS_RANGE_X.contains( &grid.x ) && MAP_GRIDS_RANGE_Y.contains( &grid.y )
        {   self.bits( grid ) & BIT_PASSAGE != 0
        }
        else { false } //範囲外は通路ではない
    }

    pub fn o_entity( &self, grid: Grid ) -> Option<Entity>
    {   if MAP_GRIDS_RANGE_X.contains( &grid.x ) && MAP_GRIDS_RANGE_Y.contains( &grid.y )
        {   self.dot_entities[ grid.x as usize ][ grid.y as usize ]
        }
        else { None } //範囲外はOption::Noneを返す
    }
    pub fn o_entity_mut( &mut self, grid: Grid ) -> &mut Option<Entity>
    {   if MAP_GRIDS_RANGE_X.contains( &grid.x ) && MAP_GRIDS_RANGE_Y.contains( &grid.y )
        {   &mut self.dot_entities[ grid.x as usize ][ grid.y as usize ]
        }
        else { &mut self.dummy_o_entity_none } //範囲外は&mut Option::Noneを返す
    }
}

//End of code.