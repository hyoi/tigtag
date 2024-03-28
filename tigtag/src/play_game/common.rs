use super::*;

////////////////////////////////////////////////////////////////////////////////

//ゲームの成績記録用のResource
#[derive( Resource, Default )]
pub struct Record
{   score   : i32, //スコア
    hi_score: i32, //ハイスコア
    stage   : i32, //ステージ数
    demo    : DemoRecord, //demo用の記録
    is_clear: bool, //ステージクリア時にtrue(※)
}
//※スコアとステージ数を誤って初期化しないよう制御用フラグを設けた

//demo用
#[derive( Default )]
pub struct DemoRecord
{   hi_score: i32, //ハイスコア
    stage   : i32, //ステージ数
}

impl Record
{   pub fn score       ( &    self ) ->      i32 {      self.score    }
    pub fn score_mut   ( &mut self ) -> &mut i32 { &mut self.score    }
    pub fn hi_score    ( &    self ) ->      i32 {      self.hi_score }
    pub fn hi_score_mut( &mut self ) -> &mut i32 { &mut self.hi_score }
    pub fn stage       ( &    self ) ->      i32 {      self.stage    }
    pub fn stage_mut   ( &mut self ) -> &mut i32 { &mut self.stage    }

    pub fn demo_hi_score    ( &    self ) ->      i32 {      self.demo.hi_score }
    pub fn demo_hi_score_mut( &mut self ) -> &mut i32 { &mut self.demo.hi_score }
    pub fn demo_stage       ( &    self ) ->      i32 {      self.demo.stage    }
    pub fn demo_stage_mut   ( &mut self ) -> &mut i32 { &mut self.demo.stage    }

    pub fn is_clear    ( &    self ) ->      bool {      self.is_clear }
    pub fn is_clear_mut( &mut self ) -> &mut bool { &mut self.is_clear }
}

////////////////////////////////////////////////////////////////////////////////

//System間の通知用イベント
#[derive( Event )] pub struct EventClear;
#[derive( Event )] pub struct EventOver;
#[derive( Event )] pub struct EventEatDot ( pub IVec2 );
#[derive( Event )] pub struct EventTimerPlayer;
#[derive( Event )] pub struct EventTimerChasers ( pub Vec<Color> );

////////////////////////////////////////////////////////////////////////////////

//四方を表す列挙型
#[derive( Default, Clone, Copy, PartialEq, Eq, Hash, Debug )]
pub enum News { #[default] North, East, West, South }

impl News
{   //時計回りで方角を得る
    pub fn turn_right( &self ) -> Self
    {   match self
        {   News::North => News::East,
            News::East  => News::South,
            News::West  => News::North,
            News::South => News::West,
        }
    }

    //反時計回りで方角を得る
    pub fn turn_left( &self ) -> Self
    {   match self
        {   News::North => News::West,
            News::East  => News::North,
            News::West  => News::South,
            News::South => News::East,
        }
    }

    //背面の方角を得る
    pub fn back_side( &self ) -> Self
    {   match self
        {   News::North => News::South,
            News::East  => News::West,
            News::West  => News::East,
            News::South => News::North,
        }
    }
}

//IVec2 = IVec2 + News
impl Add<News> for IVec2
{   type Output = IVec2;
    fn add( mut self, news: News ) -> IVec2
    {   match news
        {   News::North => { self.y -= 1; }
            News::East  => { self.x += 1; }
            News::West  => { self.x -= 1; }
            News::South => { self.y += 1; }
        }
        self
    }
}

//IVec2 = IVec2 + &News
impl Add<&News> for IVec2
{   type Output = IVec2;
    fn add( mut self, news: &News ) -> IVec2
    {   match news
        {   News::North => { self.y -= 1; }
            News::South => { self.y += 1; }
            News::East  => { self.x += 1; }
            News::West  => { self.x -= 1; }
        }
        self
    }
}

//IVec2 += News
impl AddAssign<News> for IVec2
{   fn add_assign( &mut self, news: News )
    {   match news
        {   News::North => { self.y -= 1; }
            News::East  => { self.x += 1; }
            News::West  => { self.x -= 1; }
            News::South => { self.y += 1; }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

//スプライトシートでアニメーションするためのトレイト
pub trait CharacterAnimation
{   fn anime_timer_mut( &mut self ) -> &mut Timer;
    fn sprite_sheet_frame( &self ) -> usize;
    fn sprite_sheet_offset( &self, news: News ) -> usize;
    fn direction( &self ) -> News;
}

////////////////////////////////////////////////////////////////////////////////

//End of code.