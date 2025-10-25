use super::*;

////////////////////////////////////////////////////////////////////////////////

// マップのResource
#[derive(Resource)]
pub struct Map
{
    pub rng: rand::prelude::StdRng, // マップ生成専用の乱数生成器(再現性を持たせるため)
    bit_flags: Vec<Vec<usize>>,     // マップの各グリッドの状態をbitで保存
    dot_entities: Vec<Vec<Option<Entity>>>, // ドットをdespawnする際に使うEntityIDを保存
    pub remaining_dots: i32,                // マップに残っているドットの数
    dummy_none: Option<Entity>, // 範囲外アクセスで&mut Noneを返すために使用？？？
}

impl Default for Map
{
    fn default() -> Self
    {
        // seedを決める（develpでは定数、releaseではランダム）
        let seed_dev = 1234567890;
        let seed_rel = rand::rng().random::<u64>();
        let seed = if misc::DEBUG() { seed_dev } else { seed_rel };

        Self {
            rng: StdRng::seed_from_u64(seed),
            bit_flags: vec![
                vec![0; MAP_HEIGHT_IN_CELLS as usize];
                MAP_WIDTH_IN_CELLS as usize
            ],
            dot_entities: vec![
                vec![None; MAP_HEIGHT_IN_CELLS as usize];
                MAP_WIDTH_IN_CELLS as usize
            ],
            remaining_dots: 0,
            dummy_none: None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

// マップのレンジ（外壁含む）
pub const MAP_CELLS_X_RANGE: Range<i32> = 0..MAP_WIDTH_IN_CELLS;
pub const MAP_CELLS_Y_RANGE: Range<i32> = 0..MAP_HEIGHT_IN_CELLS;

////////////////////////////////////////////////////////////////////////////////

// マップのメソッド
// メソッド経由にすることで配列の範囲外アクセスもパニックさせず意図した値を返す。
// 構造体メンバーに直接アクセスさせない(構造体メンバーは原則Not pub)。
impl Map
{
    // 非公開メソッド
    fn bits(&self, grid: IVec2) -> usize
    {
        self.bit_flags[grid.x as usize][grid.y as usize]
    }
    fn bits_mut(&mut self, grid: IVec2) -> &mut usize
    {
        &mut self.bit_flags[grid.x as usize][grid.y as usize]
    }

    fn is_inside(&self, grid: IVec2) -> bool
    {
        MAP_CELLS_X_RANGE.contains(&grid.x) && MAP_CELLS_Y_RANGE.contains(&grid.y)
    }

    // 非公開定数：マスの状態の定義
    const BIT_WALL: usize = 0b00000001; // 壁
    const BIT_PATH_RIGHT: usize = 0b00000010; // 右に道
    const BIT_PATH_LEFT: usize = 0b00000100; // 左に道
    const BIT_PATH_DOWN: usize = 0b00001000; // 上に道
    const BIT_PATH_UP: usize = 0b00010000; // 下に道

    // 公開メソッド
    pub fn set_wall(&mut self, grid: IVec2)
    {
        if !self.is_inside(grid)
        {
            return;
        }
        let flags = self.bits_mut(grid);
        *flags |= Map::BIT_WALL; // 壁フラグON
    }
    pub fn set_path(&mut self, grid: IVec2)
    {
        if !self.is_inside(grid)
        {
            return;
        }
        let flags = self.bits_mut(grid);
        *flags &= !Map::BIT_WALL; // 壁フラグOFF
    }

    pub fn is_space(&self, grid: IVec2) -> bool
    {
        if !self.is_inside(grid)
        {
            return false;
        } // 範囲外は通路ではない
        let flags = self.bits(grid);
        flags & Map::BIT_WALL == 0
    }
    pub fn is_wall(&self, grid: IVec2) -> bool
    {
        if !self.is_inside(grid)
        {
            return true;
        } // 範囲外は壁
        let flags = self.bits(grid);
        flags & Map::BIT_WALL != 0
    }

    pub fn option_entity(&self, grid: IVec2) -> Option<Entity>
    {
        if !self.is_inside(grid)
        {
            return None;
        } //範囲外はOption::Noneを返す
        self.dot_entities[grid.x as usize][grid.y as usize]
    }
    pub fn option_entity_mut(&mut self, grid: IVec2) -> &mut Option<Entity>
    {
        if !self.is_inside(grid)
        {
            return &mut self.dummy_none;
        } // 範囲外は&mut Option::Noneを返す
        &mut self.dot_entities[grid.x as usize][grid.y as usize]
    }

    pub fn init_path_bits(&mut self)
    {
        for y in MAP_CELLS_Y_RANGE
        {
            for x in MAP_CELLS_X_RANGE
            {
                let grid = IVec2::new(x, y);
                if self.is_space(grid + News::East)
                {
                    *self.bits_mut(grid) |= Map::BIT_PATH_RIGHT
                }
                else
                {
                    *self.bits_mut(grid) &= !Map::BIT_PATH_RIGHT
                }
                if self.is_space(grid + News::West)
                {
                    *self.bits_mut(grid) |= Map::BIT_PATH_LEFT
                }
                else
                {
                    *self.bits_mut(grid) &= !Map::BIT_PATH_LEFT
                }
                if self.is_space(grid + News::South)
                {
                    *self.bits_mut(grid) |= Map::BIT_PATH_DOWN
                }
                else
                {
                    *self.bits_mut(grid) &= !Map::BIT_PATH_DOWN
                }
                if self.is_space(grid + News::North)
                {
                    *self.bits_mut(grid) |= Map::BIT_PATH_UP
                }
                else
                {
                    *self.bits_mut(grid) &= !Map::BIT_PATH_UP
                }
            }
        }
    }

    pub fn get_side_spaces_list(&self, cell: IVec2) -> Vec<News>
    {
        let mut vec = Vec::<News>::with_capacity(4);
        if self.is_inside(cell)
        {
            let bits = self.bits(cell);
            if bits & Map::BIT_PATH_RIGHT != 0
            {
                vec.push(News::East)
            }
            if bits & Map::BIT_PATH_LEFT != 0
            {
                vec.push(News::West)
            }
            if bits & Map::BIT_PATH_DOWN != 0
            {
                vec.push(News::South)
            }
            if bits & Map::BIT_PATH_UP != 0
            {
                vec.push(News::North)
            }
        }
        vec // 範囲外は空になる（最外壁の外の座標だから上下左右に道はない）
    }
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
