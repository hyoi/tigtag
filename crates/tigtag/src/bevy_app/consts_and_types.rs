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

// プレイヤーのComponent
#[derive(Component)]
pub struct Player
{
    pub grid: IVec2,      // 移動中は移動元の座標、停止中はその場の座標
    pub next_grid: IVec2, // 移動中は移動先の座標、停止中はその場の座標
    pub direction: News,  // 移動の向き
    pub timer: Timer,     // 移動のタイマー
    pub is_stop: bool,    // 移動停止フラグ
    pub speedup: f32,     // スピードアップ係数
    pub px_start: Vec2,   // 1フレーム時間に移動した微小区間の始点
    pub px_end: Vec2,     // 1フレーム時間に移動した微小区間の終点
    pub opt_fn_autodrive: Option<FnAutoDrive>, /* デモ時に自キャラの移動方向を決める関数 */
    pub anime_timer: Timer,                    // キャラアニメーションのタイマー
    pub sprite_sheet_frame: u32,               // キャラアニメーションのフレーム数
    pub sprite_sheet_indexes: FxHashMap<News, u32>, /* キャラアニメーションの先頭位置(offset値) */
}

// 関数ポインタ型(デモ時の自走プレイヤーの移動方向を決める関数)
type FnAutoDrive =
    fn(&Player, Query<&Chaser>, Res<Map>, Res<DemoMapParams>, &[News]) -> News;

impl Default for Player
{
    fn default() -> Self
    {
        Self {
            grid: IVec2::default(),
            next_grid: IVec2::default(),
            direction: News::South,
            timer: Timer::from_seconds(PLAYER_TIME_PER_GRID, TimerMode::Once),
            is_stop: true,
            speedup: 1.0,
            px_start: Vec2::default(),
            px_end: Vec2::default(),
            opt_fn_autodrive: None,
            anime_timer: Timer::from_seconds(
                ANIME_TIMER_PLAYER,
                TimerMode::Repeating,
            ),
            sprite_sheet_frame: SPRITE_SHEET_COLS_PLAYER,
            sprite_sheet_indexes: (*SPRITE_SHEET_IDXS_PLAYER).clone(),
        }
    }
}

// プレイヤーの設定値
pub const PLAYER_TIME_PER_GRID: f32 = 0.15; // 0.09; //１グリッド進むために必要な時間
                                            // const PLAYER_SPEED: f32 = PIXELS_PER_GRID / PLAYER_TIME_PER_GRID; // 速度
pub const PLAYER_SPRITE_SCALING: f32 = 0.4; // primitive shape表示時の縮小係数
pub const PLAYER_SPRITE_COLOR: Color = Color::Srgba(css::YELLOW);

// スプライトシートを使ったアニメーションの情報
pub const SPRITE_SHEET_SIZE_PLAYER: UVec2 = UVec2::new(8, 8);
pub const SPRITE_SHEET_COLS_PLAYER: u32 = 4;
pub const SPRITE_SHEET_ROWS_PLAYER: u32 = 4;
pub static SPRITE_SHEET_IDXS_PLAYER: LazyLock<FxHashMap<News, u32>> =
    LazyLock::new(|| {
        FxHashMap::from_iter([
            (News::North, 0),
            (News::East, 4),
            (News::West, 8),
            (News::South, 12),
        ])
    });
pub const ANIME_TIMER_PLAYER: f32 = 0.15;

////////////////////////////////////////////////////////////////////////////////

// チェイサーのComponent
#[derive(Component)]
pub struct Chaser
{
    pub grid: IVec2,      // 移動中は移動元の座標、停止中はその場の座標
    pub next_grid: IVec2, // 移動中は移動先の座標、停止中はその場の座標
    pub direction: News,  // 移動の向き
    pub timer: Timer,     // 移動のタイマー
    pub is_stop: bool,    // 移動停止フラグ
    pub speedup: f32, // スピードアップ係数(1.0未満なら減速、1.0より大きいと増速)
    pub px_start: Vec2, // 1フレーム時間に移動した微小区間の始点
    pub px_end: Vec2, // 1フレーム時間に移動した微小区間の終点
    pub opt_fn_autochase: Option<FnAutoChase>, // 敵キャラの移動方向を決める関数
    pub color: Color, // 敵キャラの表示色
    pub anime_timer: Timer, // アニメーションのタイマー
    pub sprite_sheet_frame: u32, // アニメーションのフレーム数
    pub sprite_sheet_indexes: FxHashMap<News, u32>, /* アニメーションの先頭位置(offset値) */
}

// 関数ポインタ型(敵キャラの移動方向を決める関数)
pub type FnAutoChase = fn(&mut Chaser, &Player, &[News]) -> News;

impl Default for Chaser
{
    fn default() -> Self
    {
        Self {
            grid: IVec2::default(),
            next_grid: IVec2::default(),
            direction: News::South,
            timer: Timer::from_seconds(CHASER_TIME_PER_GRID, TimerMode::Once),
            is_stop: true,
            speedup: 1.0,
            px_start: Vec2::default(),
            px_end: Vec2::default(),
            opt_fn_autochase: None,
            color: Color::NONE,
            anime_timer: Timer::from_seconds(
                ANIME_TIMER_CHASER,
                TimerMode::Repeating,
            ),
            sprite_sheet_frame: SPRITE_SHEET_COLS_CHASER,
            sprite_sheet_indexes: (*SPRITE_SHEET_IDXS_CHASER).clone(),
        }
    }
}

// 敵キャラの設定値
pub const CHASER_START_POSITION: &[IVec2] = // スタート座標
    &[
        IVec2::new(1, 1),
        IVec2::new(1, MAX_Y),
        IVec2::new(MAX_X, 1),
        IVec2::new(MAX_X, MAX_Y),
    ];
const MAX_X: i32 = MAP_GRIDS_WIDTH - 2;
const MAX_Y: i32 = MAP_GRIDS_HEIGHT - 2;

// 各色ごとの情報（色と移動方向の決定関数とassetファイル名）
pub const CHASERS_SPRITE_INFO: &[(Color, Option<FnAutoChase>, &str)] = &[
    (
        Color::Srgba(css::RED),
        chasers::CHOICE_WAY_RED,
        ASSETS_SPRITE_SHEET_CHASER_RED,
    ),
    (
        Color::Srgba(css::GREEN),
        chasers::CHOICE_WAY_GREEN,
        ASSETS_SPRITE_SHEET_CHASER_GREEN,
    ),
    (
        Color::Srgba(css::PINK),
        chasers::CHOICE_WAY_PINK,
        ASSETS_SPRITE_SHEET_CHASER_PINK,
    ),
    (
        Color::Srgba(css::BLUE),
        chasers::CHOICE_WAY_BLUE,
        ASSETS_SPRITE_SHEET_CHASER_BLUE,
    ),
];

pub const CHASER_TIME_PER_GRID: f32 = 0.20; // 0.13; //１グリッド進むために必要な時間
                                            // const CHASER_SPEED: f32 = PIXELS_PER_GRID / CHASER_TIME_PER_GRID; //速度
pub const CHASER_SPRITE_SCALING: f32 = 0.35; // primitive shape表示時の縮小係数
                                             // const CHASER_ACCEL: f32 = 0.4; //スピードアップの割増

// スプライトシートを使ったアニメーションの情報
pub const SPRITE_SHEET_SIZE_CHASER: UVec2 = UVec2::new(8, 8);
pub const SPRITE_SHEET_COLS_CHASER: u32 = 4;
pub const SPRITE_SHEET_ROWS_CHASER: u32 = 4;
static SPRITE_SHEET_IDXS_CHASER: LazyLock<FxHashMap<News, u32>> =
    LazyLock::new(|| {
        FxHashMap::from_iter([
            (News::North, 0),
            (News::East, 4),
            (News::West, 8),
            (News::South, 12),
        ])
    });
const ANIME_TIMER_CHASER: f32 = 0.15;

////////////////////////////////////////////////////////////////////////////////

// demo用のマップ情報Resource
#[derive(Resource, Default)]
pub struct DemoMapParams
{
    dots_rect: IVec2Rect, // dotsを内包する最小の矩形
    dots_sum_x: [i32; MAP_GRIDS_WIDTH as usize], // 列に残っているdotsを数えた配列
    dots_sum_y: [i32; MAP_GRIDS_HEIGHT as usize], // 行に残っているdotsを数えた配列
}

#[derive(Default)]
struct IVec2Rect
{
    min: IVec2,
    max: IVec2,
}

// impl DemoMapParams
// {   pub fn dots_sum_x    ( &    self, x: i32 ) ->      i32 {      self.dots_sum_x[ x as usize ] }
//     pub fn dots_sum_x_mut( &mut self, x: i32 ) -> &mut i32 { &mut self.dots_sum_x[ x as usize ] }
//     pub fn dots_sum_y    ( &    self, y: i32 ) ->      i32 {      self.dots_sum_y[ y as usize ] }
//     pub fn dots_sum_y_mut( &mut self, y: i32 ) -> &mut i32 { &mut self.dots_sum_y[ y as usize ] }

//     pub fn dots_rect_min    ( &    self ) ->       IVec2 {      self.dots_rect.min }
//     pub fn dots_rect_min_mut( &mut self ) ->  &mut IVec2 { &mut self.dots_rect.min }
//     pub fn dots_rect_max    ( &    self ) ->       IVec2 {      self.dots_rect.max }
//     pub fn dots_rect_max_mut( &mut self ) ->  &mut IVec2 { &mut self.dots_rect.max }
// }

////////////////////////////////////////////////////////////////////////////////

// スプライトシートでアニメーションするためのトレイト
pub trait CharacterAnimation
{
    fn anime_timer_mut(&mut self) -> &mut Timer;
    fn sprite_sheet_frame(&self) -> u32;
    fn sprite_sheet_offset(&self, news: News) -> u32;
    fn direction(&self) -> News;
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
