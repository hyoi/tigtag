use super::*;

//internal submodules
mod map;
pub use map::*;

////////////////////////////////////////////////////////////////////////////////

//画面デザイン(枠)
pub struct ScreenFrame<'a>
{   pub design  : Vec<&'a str>,
}

////////////////////////////////////////////////////////////////////////////////

//glamの型にメソッドを追加する準備
pub trait GridToPixel
{   fn to_sprite_pixels( &self ) -> Vec2;
    fn to_screen_pixels( &self ) -> Vec2;
}

//glamの型にメソッドを追加する
impl GridToPixel for IVec2
{   //平面座標(IVec2)から画面第一象限の座標(Vec2)を算出する
    //アンカーはグリッドの中央
    fn to_sprite_pixels( &self ) -> Vec2
    {   ( self.as_vec2() + 0.5 ) * PIXELS_PER_GRID * Vec2::new( 1.0, -1.0 )
    }

    //平面座標(IVec2)から画面第一象限の座標(Vec2)を算出する
    //アンカーはグリッドの左下
    fn to_screen_pixels( &self ) -> Vec2
    {   self.as_vec2() * PIXELS_PER_GRID * Vec2::new( 1.0, -1.0 )
    }
}

////////////////////////////////////////////////////////////////////////////////

//ゲームの状態
#[derive( Clone, Copy, Eq, PartialEq, Hash, Debug, Default, States )]
pub enum MyState
{   #[default] LoadAssets, InitApp,
    GameStart, TitleDemo, DemoLoop, StageStart, MainLoop, StageClear, GameOver,
    Pause, Debug,
}

impl MyState
{   pub fn is_pause     ( &self ) -> bool { *self == MyState::Pause      }
    pub fn is_demoplay  ( &self ) -> bool { *self == MyState::TitleDemo || *self == MyState::DemoLoop }
}

//Stateの遷移に使うTrait
pub trait GotoState { fn next( &self ) -> MyState; }

//Stateの遷移に使うマーカー(not Resource)
#[derive( Default )] pub struct GameStart;
#[derive( Default )] pub struct TitleDemo;
#[derive( Default )] pub struct StageStart;
#[derive( Default )] pub struct MainLoop;

impl GotoState for GameStart  { fn next( &self ) -> MyState { MyState::GameStart  } }
impl GotoState for TitleDemo  { fn next( &self ) -> MyState { MyState::TitleDemo  } }
impl GotoState for StageStart { fn next( &self ) -> MyState { MyState::StageStart } }
impl GotoState for MainLoop   { fn next( &self ) -> MyState { MyState::MainLoop   } }

//Stateの遷移に使うマーカー(Resouce)
#[derive( Resource )] pub struct AfterLoadAssetsTo<T: States> ( pub T );
#[derive( Resource )] pub struct AfterInitAppTo   <T: States> ( pub T );
#[derive( Resource )] pub struct TitleDemoExist   <T: States> ( pub T );

impl GotoState for AfterLoadAssetsTo<MyState> { fn next( &self ) -> MyState { self.0 } }
impl GotoState for AfterInitAppTo   <MyState> { fn next( &self ) -> MyState { self.0 } }
impl GotoState for TitleDemoExist   <MyState> { fn next( &self ) -> MyState { self.0 } }

////////////////////////////////////////////////////////////////////////////////

//System間の通知用イベント
#[derive( Event )] pub struct EventClear;
#[derive( Event )] pub struct EventOver;
#[derive( Event )] pub struct EventEatDot;

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

//UI用の隠しフレームのComponent
#[derive( Component )] pub struct HiddenFrameHeader;
#[derive( Component )] pub struct HiddenFrameCenter;
#[derive( Component )] pub struct HiddenFrameFooter;

//UIのテキストメッセージセクションの型
pub type MessageSect<'a> =
(   &'a str, //表示文字列
    &'a str, //フォントのAssets
    f32,     //フォントのサイズ
    Color,   //フォントの色
);

////////////////////////////////////////////////////////////////////////////////

//操作を受け付けるgamepadを保存するResource
#[derive( Resource, Default )]
pub struct ConnectedGamepad ( Option<Gamepad> );

pub trait TraitGamepad
{   fn id( &self ) -> Option<Gamepad>;
    fn id_mut( &mut self ) -> &mut Option<Gamepad>;
}

impl TraitGamepad for ConnectedGamepad
{   fn id( &self ) -> Option<Gamepad> { self.0 }
    fn id_mut( &mut self ) -> &mut Option<Gamepad> { &mut self.0 }
}

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

//プレイヤーのComponent
#[derive( Component )]
pub struct Player
{   pub grid         : IVec2,               //移動中は移動元の座標、停止中はその場の座標
    pub next         : IVec2,               //移動中は移動先の座標、停止中はその場の座標
    pub side         : News,                //移動向き
    pub wait         : Timer,               //移動ウエイト
    pub stop         : bool,                //移動停止フラグ
    pub speedup      : f32,                 //スピードアップ係数
    pub px_start     : Vec2,                //移動した微小区間の始点
    pub px_end       : Vec2,                //移動した微小区間の終点
    pub opt_autodrive: Option<FnAutoDrive>, //デモ時の自走プレイヤーの移動方向を決める関数ポインタ
}

impl Default for Player
{   fn default() -> Self
    {   Self
        {   grid         : IVec2::default(),
            next         : IVec2::default(),
            side         : News::default(),
            wait         : Timer::from_seconds( PLAYER_WAIT, TimerMode::Once ),
            stop         : true,
            speedup      : 1.0,
            px_start     : Vec2::default(),
            px_end       : Vec2::default(),
            opt_autodrive: None,
        }
    }
}

//関数ポインタ型(デモ時の自走プレイヤーの移動方向を決める関数)
type FnAutoDrive = fn( &Player, Query<&Chaser>, Res<Map>, Res<DemoMapParams>, &[News] ) -> News;

////////////////////////////////////////////////////////////////////////////////

//チェイサーのComponent
#[derive( Component )]
pub struct Chaser
{   pub grid      : IVec2,             //移動中は移動元の座標、停止中はその場の座標
    pub next      : IVec2,             //移動中は移動先の座標、停止中はその場の座標
    pub side      : News,              //移動向き
    pub wait      : Timer,             //移動ウエイト
    pub stop      : bool,              //移動停止フラグ
    pub speedup   : f32,               //スピードアップ係数(1.0未満なら減速、1.0より大きいと増速)
    pub px_start  : Vec2,              //移動した微小区間の始点
    pub px_end    : Vec2,              //移動した微小区間の終点
    pub color     : Color,             //表示色
    pub fn_chasing: Option<FnChasing>, //チェイサーの移動方向を決める関数ポインタ
}

impl Default for Chaser
{   fn default() -> Self
    {   Self
        {   grid      : IVec2::default(),
            next      : IVec2::default(),
            side      : News::default(),
            wait      : Timer::from_seconds( CHASER_WAIT, TimerMode::Once ),
            stop      : true,
            speedup   : 1.0,
            px_start  : Vec2::default(),
            px_end    : Vec2::default(),
            color     : Color::NONE,
            fn_chasing: None,
        }
    }
}

//関数ポインタ型(追手の移動方向を決める関数)
pub type FnChasing = fn( &mut Chaser, &Player, &[News] ) -> News;

////////////////////////////////////////////////////////////////////////////////

//End of code.