use super::*;

////////////////////////////////////////////////////////////////////////////////

// ゲームの成績記録用のResource
#[derive(Resource, Default)]
pub struct Record
{
    score: i32,       // スコア
    hi_score: i32,    // ハイスコア
    stage: i32,       // ステージ数
    demo: DemoRecord, // demo用の記録
    is_clear: bool,   // ステージクリア時にtrue(※)
}
//※スコアとステージ数を誤って初期化しないよう制御用フラグを設けた

// demo用
#[derive(Default)]
pub struct DemoRecord
{
    hi_score: i32, // ハイスコア
    stage: i32,    // ステージ数
}

// フィールドアクセス
impl Record
{
    pub fn score(&self) -> i32 { self.score }
    pub fn score_mut(&mut self) -> &mut i32 { &mut self.score }

    pub fn hi_score(&self) -> i32 { self.hi_score }
    pub fn hi_score_mut(&mut self) -> &mut i32 { &mut self.hi_score }

    pub fn stage(&self) -> i32 { self.stage }
    pub fn stage_mut(&mut self) -> &mut i32 { &mut self.stage }

    pub fn demo_hi_score(&self) -> i32 { self.demo.hi_score }
    pub fn demo_hi_score_mut(&mut self) -> &mut i32 { &mut self.demo.hi_score }

    pub fn demo_stage(&self) -> i32 { self.demo.stage }
    pub fn demo_stage_mut(&mut self) -> &mut i32 { &mut self.demo.stage }

    pub fn is_clear(&self) -> bool { self.is_clear }
    pub fn is_clear_mut(&mut self) -> &mut bool { &mut self.is_clear }
}

////////////////////////////////////////////////////////////////////////////////

// マップのResource
#[derive(Resource)]
pub struct Map
{
    pub rng: rand::prelude::StdRng, /* マップ生成専用の乱数生成器(マップに再現性を持たせるため) */
    bit_flags: Vec<Vec<usize>>,     // マップの各グリッドの状態をbitで保存
    dot_entities: Vec<Vec<Option<Entity>>>, /* ドットをdespawnする際に使うEntityIDを保存 */
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
        let seed = if DEBUG() { seed_dev } else { seed_rel };

        Self {
            rng: StdRng::seed_from_u64(seed),
            bit_flags: vec![
                vec![0; MAP_GRIDS_HEIGHT as usize];
                MAP_GRIDS_WIDTH as usize
            ],
            dot_entities: vec![
                vec![None; MAP_GRIDS_HEIGHT as usize];
                MAP_GRIDS_WIDTH as usize
            ],
            remaining_dots: 0,
            dummy_none: None,
        }
    }
}

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
        MAP_GRIDS_X_RANGE.contains(&grid.x) && MAP_GRIDS_Y_RANGE.contains(&grid.y)
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

    pub fn is_wall(&self, grid: IVec2) -> bool
    {
        if !self.is_inside(grid)
        {
            return true;
        } // 範囲外は壁
        let flags = self.bits(grid);
        flags & Map::BIT_WALL != 0
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

    // pub fn opt_entity( &self, grid: IVec2 ) -> Option<Entity>
    // {   if ! self.is_inside( grid ) { return None } //範囲外はOption::Noneを返す
    //     self.dot_entities[ grid.x as usize ][ grid.y as usize ]
    // }
    pub fn opt_entity_mut(&mut self, grid: IVec2) -> &mut Option<Entity>
    {
        if !self.is_inside(grid)
        {
            return &mut self.dummy_none;
        } // 範囲外は&mut Option::Noneを返す
        &mut self.dot_entities[grid.x as usize][grid.y as usize]
    }

    pub fn init_path_bits(&mut self)
    {
        for y in MAP_GRIDS_Y_RANGE
        {
            for x in MAP_GRIDS_X_RANGE
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

    // pub fn get_side_spaces_list( &self, grid: IVec2 ) -> Vec<News>
    // {   let mut vec = Vec::<News>::with_capacity( 4 );
    //     if self.is_inside( grid )
    //     {   let bits = self.bits( grid );
    //         if bits & Map::BIT_PATH_RIGHT != 0 { vec.push( News::East  ) }
    //         if bits & Map::BIT_PATH_LEFT  != 0 { vec.push( News::West  ) }
    //         if bits & Map::BIT_PATH_DOWN  != 0 { vec.push( News::South ) }
    //         if bits & Map::BIT_PATH_UP    != 0 { vec.push( News::North ) }
    //     }
    //     vec //範囲外は空になる（最外壁の外の座標だから上下左右に道はない）
    // }
}

////////////////////////////////////////////////////////////////////////////////

// マップ縦横幅
pub const MAP_GRIDS_WIDTH: i32 = SCREEN_GRIDS_WIDTH; // w <= SCREEN_GRIDS_WIDTH;
pub const MAP_GRIDS_HEIGHT: i32 = SCREEN_GRIDS_HEIGHT - 2; // h <= SCREEN_GRIDS_HEIGHT - 2;

// マップのレンジ（外壁含む）
pub const MAP_GRIDS_X_RANGE: Range<i32> = 0..MAP_GRIDS_WIDTH;
pub const MAP_GRIDS_Y_RANGE: Range<i32> = 0..MAP_GRIDS_HEIGHT;

////////////////////////////////////////////////////////////////////////////////

// 四方を表す列挙型
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum News
{
    #[default]
    North,
    East,
    West,
    South,
}

// IVec2 = IVec2 + News
impl Add<News> for IVec2
{
    type Output = IVec2;
    fn add(mut self, news: News) -> IVec2
    {
        match news
        {
            News::North =>
            {
                self.y -= 1;
            }
            News::East =>
            {
                self.x += 1;
            }
            News::West =>
            {
                self.x -= 1;
            }
            News::South =>
            {
                self.y += 1;
            }
        }
        self
    }
}

////////////////////////////////////////////////////////////////////////////////

// オーファンルール対策（glamの型にメソッドを追加する準備）
pub trait GridToPixelOnMap
{
    fn to_vec2_on_game_map(&self) -> Vec2;
}

// glamの型にメソッドを追加する
impl GridToPixelOnMap for IVec2
{
    // マップと画面の座標調整値を加味してvec2へ変換する
    fn to_vec2_on_game_map(&self) -> Vec2
    {
        let grid = *self + ADJUST_MAP_ON_SCREEN;
        grid.to_vec2_of_screen()
    }
}

// アジャスタ（マップ座標から画面座標への変換調整値）
const ADJUST_MAP_ON_SCREEN: IVec2 = IVec2::new(0, 1);

////////////////////////////////////////////////////////////////////////////////

// End of code.
