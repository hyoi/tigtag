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
}

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
    pub fn stage(&self) -> i32 { self.stage }
    pub fn stage_mut(&mut self) -> &mut i32 { &mut self.stage }

    pub fn score(&self) -> i32 { self.score }
    pub fn score_mut(&mut self) -> &mut i32 { &mut self.score }

    pub fn hi_score(&self) -> i32 { self.hi_score }
    pub fn hi_score_mut(&mut self) -> &mut i32 { &mut self.hi_score }

    pub fn demo_hi_score(&self) -> i32 { self.demo.hi_score }
    pub fn demo_hi_score_mut(&mut self) -> &mut i32 { &mut self.demo.hi_score }

    pub fn demo_stage(&self) -> i32 { self.demo.stage }
    pub fn demo_stage_mut(&mut self) -> &mut i32 { &mut self.demo.stage }
}

////////////////////////////////////////////////////////////////////////////////

// System間通知用メッセージ
pub use my_messages::*;
#[rustfmt::skip]
mod my_messages
{
    use super::*;
    #[derive(Message)] pub struct CountDownEnded;
    #[derive(Message, Default)] pub struct SkipOverlayMessage;
    #[derive(Message)] pub struct DotsAllEaten ;
    #[derive(Message)] pub struct DotEaten ;
    #[derive(Message)] pub struct PlayerCaught;
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
            News::North => self.y -= 1,
            News::East => self.x += 1,
            News::West => self.x -= 1,
            News::South => self.y += 1,
        }
        self
    }
}

// IVec2 += News
impl AddAssign<News> for IVec2
{
    fn add_assign(&mut self, news: News)
    {
        match news
        {
            News::North => self.y -= 1,
            News::East => self.x += 1,
            News::West => self.x -= 1,
            News::South => self.y += 1,
        }
    }
}

// IVec2 = IVec2 + &mut News
// impl Add<&mut News> for IVec2
// {
//     type Output = IVec2;
//     fn add(mut self, news: &mut News) -> IVec2
//     {
//         match news
//         {
//             News::North => self.y -= 1,
//             News::South => self.y += 1,
//             News::East => self.x += 1,
//             News::West => self.x -= 1,
//         }
//         self
//     }
// }

impl News
{
    // 背面の方角を得る
    pub fn back(&self) -> Self
    {
        match self
        {
            News::North => News::South,
            News::East => News::West,
            News::West => News::East,
            News::South => News::North,
        }
    }

    // //時計回りで方角を得る
    // pub fn turn_right( &self ) -> Self
    // {   match self
    //     {   News::North => News::East,
    //         News::East  => News::South,
    //         News::West  => News::North,
    //         News::South => News::West,
    //     }
    // }

    // //反時計回りで方角を得る
    // pub fn turn_left( &self ) -> Self
    // {   match self
    //     {   News::North => News::West,
    //         News::East  => News::North,
    //         News::West  => News::South,
    //         News::South => News::East,
    //     }
    // }
}

////////////////////////////////////////////////////////////////////////////////

// オーファンルール対策（glamの型にメソッドを追加する準備）
pub trait GridToPixelOnMap
{
    fn to_screen_pixels_map_adjusted(&self) -> Vec2;
}

// glamの型にメソッドを追加する
impl GridToPixelOnMap for IVec2
{
    // マップと画面の座標調整値を加味してvec2へ変換する
    fn to_screen_pixels_map_adjusted(&self) -> Vec2
    {
        let grid = *self + ADJUST_MAP_ON_SCREEN;
        grid.to_screen_pixels()
    }
}

// アジャスタ（マップ座標から画面座標への変換調整値）
const ADJUST_MAP_ON_SCREEN: IVec2 = IVec2::new(0, 1);

////////////////////////////////////////////////////////////////////////////////

// End of code.
