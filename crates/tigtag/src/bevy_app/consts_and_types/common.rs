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